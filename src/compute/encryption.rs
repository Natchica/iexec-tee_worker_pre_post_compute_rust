use crate::compute::{
    errors::ReplicateStatusCause,
    utils::env_utils::{get_env_var_or_error, TeeSessionEnvironmentVariable},
};
use aes::cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyIvInit};
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use cbc::Encryptor;
use log::{error, info};
use rand::{rngs::OsRng, RngCore};
use rsa::{pkcs8::DecodePublicKey, Pkcs1v15Encrypt, RsaPublicKey}; // Use pkcs8 for X509
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};
use walkdir::WalkDir;
use zip::{write::FileOptions, ZipWriter};

pub const AES_KEY_RSA_FILENAME: &str = "aes-key.rsa";
pub const ENCRYPTION_PREFIX: &str = "encrypted-";

type Aes256CbcEncrypt = Encryptor<aes::Aes256>;

pub fn generate_aes_key() -> Result<Vec<u8>, ReplicateStatusCause> {
    let mut key_bytes = [0u8; 32]; // 256-bit key
    if let Err(e) = OsRng.try_fill_bytes(&mut key_bytes) {
        error!("Failed to generate AES key: {}", e);
        return Err(ReplicateStatusCause::PostComputeEncryptionFailed);
    }
    Ok(key_bytes.to_vec())
}

pub fn aes_encrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>, ReplicateStatusCause> {
    let mut iv = [0u8; 16]; // 128-bit IV
    if let Err(e) = OsRng.try_fill_bytes(&mut iv) {
        error!("Failed to generate IV for AES encryption: {}", e);
        return Err(ReplicateStatusCause::PostComputeEncryptionFailed);
    }

    let cipher = Aes256CbcEncrypt::new_from_slices(key, &iv).map_err(|e| {
        error!(
            "Failed to initialize AES cipher [key_len:{}]: {}",
            key.len(),
            e
        );
        ReplicateStatusCause::PostComputeEncryptionFailed
    })?;

    // Data needs to be mutable and have enough capacity for padding.
    // The `encrypt_padded_mut` method requires the buffer to be `data.len() + N`
    // where N is the block size (16 bytes for AES).
    let mut buffer = Vec::with_capacity(data.len() + 16);
    buffer.extend_from_slice(data);

    let ct = cipher
        .encrypt_padded_mut::<Pkcs7>(&mut buffer, data.len())
        .map_err(|e| {
            error!(
                "Failed to encrypt with AES [data_len:{}, key_len:{}]: {}",
                data.len(),
                key.len(),
                e
            );
            ReplicateStatusCause::PostComputeEncryptionFailed
        })?;

    let mut iv_and_encrypted_data = Vec::new();
    iv_and_encrypted_data.extend_from_slice(&iv);
    iv_and_encrypted_data.extend_from_slice(ct);

    Ok(iv_and_encrypted_data)
}

pub fn plain_text_to_rsa_public_key(
    plain_text_rsa_pub: &str,
) -> Result<RsaPublicKey, ReplicateStatusCause> {
    let pem_content = plain_text_rsa_pub
        .replace("-----BEGIN PUBLIC KEY-----", "")
        .replace("-----END PUBLIC KEY-----", "")
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

    let der_bytes = BASE64_STANDARD.decode(pem_content).map_err(|e| {
        error!("Failed to decode base64 RSA public key: {}", e);
        ReplicateStatusCause::PostComputeMalformedEncryptionPublicKey
    })?;

    // For X509EncodedKeySpec (SubjectPublicKeyInfo)
    RsaPublicKey::from_public_key_der(&der_bytes).map_err(|e| {
        error!("Failed to parse DER RSA public key: {}", e);
        ReplicateStatusCause::PostComputeMalformedEncryptionPublicKey
    })
}

pub fn rsa_encrypt(
    data: &[u8],
    public_key: &RsaPublicKey,
) -> Result<Vec<u8>, ReplicateStatusCause> {
    public_key
        .encrypt(&mut OsRng, Pkcs1v15Encrypt, data)
        .map_err(|e| {
            error!("Failed to encrypt with RSA: {}", e);
            ReplicateStatusCause::PostComputeEncryptionFailed
        })
}

fn zip_folder(
    source_dir_path: &str,
    output_zip_path: &str,
) -> Result<(), ReplicateStatusCause> {
    info!(
        "Starting to zip folder {} to {}",
        source_dir_path, output_zip_path
    );
    let zip_file = File::create(output_zip_path).map_err(|e| {
        error!("Failed to create zip file {}: {}", output_zip_path, e);
        ReplicateStatusCause::PostComputeEncryptionFailed
    })?;
    let mut zip = ZipWriter::new(zip_file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755); // Standard permissions

    let base_path = Path::new(source_dir_path);
    for entry in WalkDir::new(source_dir_path).min_depth(1) {
        // min_depth(1) to exclude the source_dir_path itself
        let entry = entry.map_err(|e| {
            error!(
                "Error walking directory {}: {}",
                source_dir_path, e
            );
            ReplicateStatusCause::PostComputeEncryptionFailed
        })?;
        let path = entry.path();
        let name = path
            .strip_prefix(base_path)
            .unwrap() // Should not fail if path is from WalkDir of base_path
            .to_str()
            .ok_or_else(|| {
                error!("Invalid path found during zipping: {:?}", path);
                ReplicateStatusCause::PostComputeEncryptionFailed
            })?;

        if path.is_file() {
            info!("Adding file to zip: {}", name);
            zip.start_file(name, options).map_err(|e| {
                error!("Failed to start file {} in zip: {}", name, e);
                ReplicateStatusCause::PostComputeEncryptionFailed
            })?;
            let mut f = File::open(path).map_err(|e| {
                error!("Failed to open file {} for zipping: {}", path.display(), e);
                ReplicateStatusCause::PostComputeEncryptionFailed
            })?;
            std::io::copy(&mut f, &mut zip).map_err(|e| {
                error!("Failed to copy file {} to zip: {}", path.display(), e);
                ReplicateStatusCause::PostComputeEncryptionFailed
            })?;
        } else if path.is_dir() {
            info!("Adding directory to zip: {}", name);
            zip.add_directory(name, options).map_err(|e| {
                error!("Failed to add directory {} to zip: {}", name, e);
                ReplicateStatusCause::PostComputeEncryptionFailed
            })?;
        }
    }
    zip.finish().map_err(|e| {
        error!("Failed to finish zip file {}: {}", output_zip_path, e);
        ReplicateStatusCause::PostComputeEncryptionFailed
    })?;
    info!(
        "Successfully zipped folder {} to {}",
        source_dir_path, output_zip_path
    );
    Ok(())
}

#[allow(clippy::too_many_lines)]
pub fn encrypt_data(
    in_data_file_path_str: &str,
    plain_text_rsa_pub: &str,
    produce_zip: bool,
) -> Result<String, ReplicateStatusCause> {
    let in_data_file_path = Path::new(in_data_file_path_str);

    let in_data_filename = in_data_file_path
        .file_name()
        .ok_or_else(|| {
            error!(
                "Failed to get filename from path: {}",
                in_data_file_path_str
            );
            ReplicateStatusCause::PostComputeEncryptionFailed
        })?
        .to_str()
        .ok_or_else(|| {
            error!(
                "Filename is not valid UTF-8: {:?}",
                in_data_file_path.file_name()
            );
            ReplicateStatusCause::PostComputeEncryptionFailed
        })?;

    let work_dir = in_data_file_path
        .parent()
        .ok_or_else(|| {
            error!(
                "Failed to get parent directory from path: {}",
                in_data_file_path_str
            );
            ReplicateStatusCause::PostComputeEncryptionFailed
        })?
        .to_str()
        .ok_or_else(|| {
            error!(
                "Work directory path is not valid UTF-8: {:?}",
                in_data_file_path.parent()
            );
            ReplicateStatusCause::PostComputeEncryptionFailed
        })?;

    let in_data_file_stem = Path::new(in_data_filename)
        .file_stem()
        .ok_or_else(|| {
            error!("Failed to get file stem from: {}", in_data_filename);
            ReplicateStatusCause::PostComputeEncryptionFailed
        })?
        .to_str()
        .ok_or_else(|| {
            error!(
                "File stem is not valid UTF-8: {:?}",
                Path::new(in_data_filename).file_stem()
            );
            ReplicateStatusCause::PostComputeEncryptionFailed
        })?;

    let out_enc_dir_path_str =
        format!("{}/{}{}", work_dir, ENCRYPTION_PREFIX, in_data_file_stem);
    let out_enc_dir_path = PathBuf::from(&out_enc_dir_path_str);

    let out_encrypted_data_filename = format!("{}.aes", in_data_filename);
    let out_encrypted_data_path = out_enc_dir_path.join(&out_encrypted_data_filename);
    let out_encrypted_aes_key_path = out_enc_dir_path.join(AES_KEY_RSA_FILENAME);

    let data_to_encrypt = match fs::read(in_data_file_path) {
        Ok(data) => data,
        Err(e) => {
            error!(
                "Failed to encryptData (readFile error: {}): {}",
                in_data_file_path_str, e
            );
            return Ok("".to_string());
        }
    };

    let aes_key = match generate_aes_key() {
        Ok(key) => key,
        Err(e) => {
            error!("Failed to encryptData (generateAesKey error): {:?}", e);
            return Ok("".to_string());
        }
    };

    let encrypted_data = match aes_encrypt(&data_to_encrypt, &aes_key) {
        Ok(data) => data,
        Err(e) => {
            error!("Failed to encryptData (aesEncrypt error): {:?}", e);
            return Ok("".to_string());
        }
    };

    let rsa_public_key = match plain_text_to_rsa_public_key(plain_text_rsa_pub) {
        Ok(key) => key,
        Err(e) => {
            error!(
                "Failed to encryptData (plainText2RsaPublicKey error): {:?}",
                e
            );
            return Ok("".to_string());
        }
    };

    let encrypted_aes_key = match rsa_encrypt(&aes_key, &rsa_public_key) {
        Ok(key) => key,
        Err(e) => {
            error!("Failed to encryptData (rsaEncrypt error): {:?}", e);
            return Ok("".to_string());
        }
    };

    if let Err(e) = fs::create_dir_all(&out_enc_dir_path) {
        error!(
            "Failed to encryptData (createDirectory error: {}): {}",
            out_enc_dir_path.display(),
            e
        );
        return Ok("".to_string());
    }

    if let Err(e) = fs::write(&out_encrypted_data_path, encrypted_data) {
        error!(
            "Failed to encryptData (writeBytes encryptedData error: {}): {}",
            out_encrypted_data_path.display(),
            e
        );
        return Ok("".to_string());
    }
    info!(
        "Successfully wrote encrypted data to: {}",
        out_encrypted_data_path.display()
    );

    if let Err(e) = fs::write(&out_encrypted_aes_key_path, encrypted_aes_key) {
        error!(
            "Failed to encryptData (writeBytes encryptedAesKey error: {}): {}",
            out_encrypted_aes_key_path.display(),
            e
        );
        return Ok("".to_string());
    }
    info!(
        "Successfully wrote encrypted AES key to: {}",
        out_encrypted_aes_key_path.display()
    );

    if produce_zip {
        let out_encrypted_zip_path_str = format!("{}.zip", out_enc_dir_path_str);
        match zip_folder(&out_enc_dir_path_str, &out_encrypted_zip_path_str) {
            Ok(_) => {
                info!(
                    "Successfully created encrypted zip: {}",
                    out_encrypted_zip_path_str
                );
                Ok(out_encrypted_zip_path_str)
            }
            Err(e) => {
                error!("Failed to encryptData (zipFolder error): {:?}", e);
                Ok("".to_string())
            }
        }
    } else {
        Ok(out_enc_dir_path_str)
    }
}

pub fn eventually_encrypt_result(
    in_data_file_path: &str,
) -> Result<String, ReplicateStatusCause> {
    match get_env_var_or_error(
        TeeSessionEnvironmentVariable::ResultEncryptionPublicKey,
        // This specific cause is for when the var is missing, not for other failures.
        ReplicateStatusCause::PostComputeEncryptionPublicKeyMissing,
    ) {
        Ok(plain_text_rsa_pub) => {
            // get_env_var_or_error should ensure that the string is not empty.
            // If it could be empty, an explicit check here would be:
            // if plain_text_rsa_pub.is_empty() {
            //     error!("RESULT_ENCRYPTION_PUBLIC_KEY is empty after retrieval.");
            //     return Err(ReplicateStatusCause::PostComputeEncryptionPublicKeyMissing);
            // }
            info!("Starting encryption as RESULT_ENCRYPTION_PUBLIC_KEY is set.");
            match encrypt_data(in_data_file_path, &plain_text_rsa_pub, true) {
                Ok(path) => {
                    if path.is_empty() {
                        // encrypt_data signals internal error by returning empty string
                        error!(
                            "Encryption process failed internally for {}, an empty path was returned.",
                            in_data_file_path
                        );
                        Err(ReplicateStatusCause::PostComputeEncryptionFailed)
                    } else {
                        info!(
                            "Successfully encrypted {} to {}",
                            in_data_file_path, path
                        );
                        Ok(path)
                    }
                }
                Err(e) => {
                    error!(
                        "Encryption process failed for {} with error: {:?}",
                        in_data_file_path, e
                    );
                    Err(e) // Propagate the specific error from encrypt_data
                }
            }
        }
        Err(ReplicateStatusCause::PostComputeEncryptionPublicKeyMissing) => {
            // This means the env var was not found, so encryption is skipped.
            info!("Result encryption not enabled (RESULT_ENCRYPTION_PUBLIC_KEY not found or empty).");
            Ok(in_data_file_path.to_string())
        }
        Err(other_cause) => {
            // This would be unexpected if get_env_var_or_error only returns the specified missing cause or Ok.
            error!(
                "Unexpected error while checking for encryption public key: {:?}",
                other_cause
            );
            Err(other_cause)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute::errors::ReplicateStatusCause;
    use rsa::{pkcs1::EncodeRsaPublicKey, RsaPrivateKey}; // Corrected import for EncodeRsaPublicKey if needed by helpers
    use std::{
        fs,
        io::{Cursor, Read}, // Added Cursor for ZipArchive
        path::Path,
    };
    use temp_env::with_vars;
    use tempfile::{tempdir, NamedTempFile};
    use zip::ZipArchive;

    // Helper to generate a test RSA private key (and derive public key)
    fn generate_test_rsa_keys_for_testing() -> (RsaPrivateKey, RsaPublicKey) {
        let mut rng = OsRng;
        let bits = 2048; // Standard size for RSA keys
        let private_key =
            RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key for tests");
        let public_key = RsaPublicKey::from(&private_key);
        (private_key, public_key)
    }

    // Helper to get a PEM-formatted public key string for tests
    fn get_pem_string_for_test_public_key(public_key: &RsaPublicKey) -> String {
        // RsaPublicKey::to_public_key_der() is from rsa::pkcs8::EncodePublicKey trait
        let public_key_der = public_key
            .to_public_key_der()
            .expect("Failed to encode public key to DER for test");
        let base64_encoded_key = BASE64_STANDARD.encode(public_key_der.as_bytes());
        // Format into a typical PEM like structure
        format!(
            "-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----",
            base64_encoded_key
                .as_bytes()
                .chunks(64)
                .map(|chunk| std::str::from_utf8(chunk).unwrap())
                .collect::<Vec<&str>>()
                .join("\n")
        )
    }

    #[test]
    fn test_generate_aes_key() {
        let key_result = generate_aes_key();
        assert!(key_result.is_ok());
        let key = key_result.unwrap();
        assert_eq!(key.len(), 32);
    }

    #[test]
    fn test_aes_encrypt() {
        let key = generate_aes_key().unwrap();
        let data = b"test data for aes encryption";

        let encrypted_result = aes_encrypt(data, &key);
        assert!(encrypted_result.is_ok());
        let iv_and_encrypted_data = encrypted_result.unwrap();

        assert!(iv_and_encrypted_data.len() > data.len()); // IV + Padding
        assert_eq!(iv_and_encrypted_data.len(), 16 + 32); // IV (16) + Data (28 -> padded to 32)

        let iv = &iv_and_encrypted_data[0..16];
        let ciphertext_block = &iv_and_encrypted_data[16..32]; // First block of ciphertext
        assert_ne!(iv, ciphertext_block, "IV should be random and not directly match the first ciphertext block");
    }

    #[test]
    fn test_plain_text_to_rsa_public_key() {
        let (_private_key, public_key) = generate_test_rsa_keys_for_testing();
        let pem_str = get_pem_string_for_test_public_key(&public_key);

        let parsed_key_result = plain_text_to_rsa_public_key(&pem_str);
        assert!(parsed_key_result.is_ok());

        let malformed_pem_str = "THIS IS NOT A VALID PEM STRING";
        let parsed_key_result_malformed = plain_text_to_rsa_public_key(malformed_pem_str);
        assert!(parsed_key_result_malformed.is_err());
        assert_eq!(
            parsed_key_result_malformed.unwrap_err(),
            ReplicateStatusCause::PostComputeMalformedEncryptionPublicKey
        );

        let empty_pem_str = "";
        let parsed_key_result_empty = plain_text_to_rsa_public_key(empty_pem_str);
        assert!(parsed_key_result_empty.is_err());
        assert_eq!(
            parsed_key_result_empty.unwrap_err(),
            ReplicateStatusCause::PostComputeMalformedEncryptionPublicKey // Or specific error for empty
        );

        let pem_without_header = pem_str.replace("-----BEGIN PUBLIC KEY-----", "");
        let parsed_key_result_no_header = plain_text_to_rsa_public_key(&pem_without_header);
        assert!(parsed_key_result_no_header.is_err()); // Assuming strict parsing or base64 failure
    }

    #[test]
    fn test_rsa_encrypt() {
        let (_private_key, public_key) = generate_test_rsa_keys_for_testing();
        let data_to_encrypt = generate_aes_key().unwrap(); // Use a 32-byte AES key as data

        let encrypted_result = rsa_encrypt(&data_to_encrypt, &public_key);
        assert!(encrypted_result.is_ok());
        let encrypted_data = encrypted_result.unwrap();

        assert!(!encrypted_data.is_empty());
        assert_ne!(encrypted_data, data_to_encrypt);
        assert_eq!(encrypted_data.len(), public_key.size()); // RSA encryption output size matches key size
    }

    #[test]
    fn test_encrypt_data() {
        let temp_input_file = NamedTempFile::new().expect("Failed to create temp input file");
        let mut file = temp_input_file.as_file();
        file.write_all(b"Some test data to encrypt.")
            .expect("Failed to write to temp file");
        let input_file_path_str = temp_input_file.path().to_str().unwrap();
        let input_file_name = temp_input_file
            .path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();

        let (_private_key, public_key) = generate_test_rsa_keys_for_testing();
        let test_rsa_pub_key_pem = get_pem_string_for_test_public_key(&public_key);

        // Case 1: produce_zip = false
        let temp_work_dir_false_zip = tempdir().expect("Failed to create temp work dir");
        let unique_input_path_false_zip = temp_work_dir_false_zip.path().join("input_for_false.txt");
        fs::copy(temp_input_file.path(), &unique_input_path_false_zip).unwrap();


        let result_no_zip = encrypt_data(
            unique_input_path_false_zip.to_str().unwrap(),
            &test_rsa_pub_key_pem,
            false,
        );
        assert!(result_no_zip.is_ok(), "encrypt_data (no zip) failed: {:?}", result_no_zip.err());
        let out_enc_dir_str = result_no_zip.unwrap();
        assert!(!out_enc_dir_str.is_empty());

        let out_enc_dir_path = Path::new(&out_enc_dir_str);
        assert!(out_enc_dir_path.exists(), "Encrypted directory should exist");
        assert!(out_enc_dir_path.is_dir());

        let aes_key_path = out_enc_dir_path.join(AES_KEY_RSA_FILENAME);
        assert!(aes_key_path.exists());
        assert!(fs::metadata(&aes_key_path).unwrap().len() > 0);

        let enc_data_filename = format!("{}.aes", unique_input_path_false_zip.file_name().unwrap().to_str().unwrap());
        let enc_data_path = out_enc_dir_path.join(enc_data_filename);
        assert!(enc_data_path.exists());
        assert!(fs::metadata(&enc_data_path).unwrap().len() > 0);

        fs::remove_dir_all(out_enc_dir_path).expect("Failed to clean up encrypted dir (no zip)");


        // Case 2: produce_zip = true
        let temp_work_dir_true_zip = tempdir().expect("Failed to create temp work dir for zip test");
        let unique_input_path_true_zip = temp_work_dir_true_zip.path().join("input_for_true.txt");
        fs::copy(temp_input_file.path(), &unique_input_path_true_zip).unwrap();
        let input_file_name_true_zip = unique_input_path_true_zip.file_name().unwrap().to_str().unwrap();


        let result_zip = encrypt_data(
            unique_input_path_true_zip.to_str().unwrap(),
            &test_rsa_pub_key_pem,
            true,
        );
        assert!(result_zip.is_ok(), "encrypt_data (zip) failed: {:?}", result_zip.err());
        let out_zip_path_str = result_zip.unwrap();
        assert!(!out_zip_path_str.is_empty());
        assert!(out_zip_path_str.ends_with(".zip"));

        let out_zip_path = Path::new(&out_zip_path_str);
        assert!(out_zip_path.exists(), "Output zip file should exist");

        let zip_file_content = fs::read(out_zip_path).expect("Failed to read zip file");
        let mut archive = ZipArchive::new(Cursor::new(zip_file_content)).expect("Failed to open zip archive");

        let expected_aes_key_in_zip = AES_KEY_RSA_FILENAME;
        let expected_data_in_zip = format!("{}.aes", input_file_name_true_zip);

        assert!(archive.by_name(expected_aes_key_in_zip).is_ok(), "AES key missing in zip");
        assert!(archive.by_name(&expected_data_in_zip).is_ok(), "Encrypted data missing in zip");

        fs::remove_file(out_zip_path).expect("Failed to clean up zip file");
        let encrypted_dir_path_for_zip = out_zip_path_str.strip_suffix(".zip").unwrap();
        if Path::new(encrypted_dir_path_for_zip).exists() { // zip_folder doesn't remove original dir
            fs::remove_dir_all(encrypted_dir_path_for_zip).expect("Failed to clean up intermediate encrypted dir (zip)");
        }


        // Error case: Invalid RSA public key
        let result_invalid_key = encrypt_data(
            input_file_path_str,
            "INVALID_KEY_PEM_STRING",
            true
        );
        assert!(result_invalid_key.is_ok()); // As per current design, returns Ok("")
        assert_eq!(result_invalid_key.unwrap(), "");

        // Error case: Input file does not exist
        let result_no_file = encrypt_data(
            "/path/to/non/existent/file.txt",
            &test_rsa_pub_key_pem,
            true,
        );
        assert!(result_no_file.is_ok()); // As per current design, returns Ok("")
        assert_eq!(result_no_file.unwrap(), "");
    }

    #[test]
    fn test_eventually_encrypt_result_success() {
        let temp_input_file = NamedTempFile::new().expect("Failed to create temp input file");
        temp_input_file.as_file().write_all(b"Test data.").unwrap();
        let input_file_path_str = temp_input_file.path().to_str().unwrap();
        let input_file_name = temp_input_file.path().file_name().unwrap().to_str().unwrap();

        let (_private_key, public_key) = generate_test_rsa_keys_for_testing();
        let test_rsa_pub_key_pem = get_pem_string_for_test_public_key(&public_key);

        with_vars(
            [(
                TeeSessionEnvironmentVariable::ResultEncryptionPublicKey.name(),
                Some(test_rsa_pub_key_pem),
            )],
            || {
                let result = eventually_encrypt_result(input_file_path_str);
                assert!(result.is_ok(), "eventually_encrypt_result failed: {:?}", result.err());
                let output_path_str = result.unwrap();
                assert!(!output_path_str.is_empty());
                assert!(output_path_str.ends_with(".zip"));

                let output_path = Path::new(&output_path_str);
                assert!(output_path.exists());

                let zip_file_content = fs::read(output_path).expect("Failed to read zip file");
                let mut archive = ZipArchive::new(Cursor::new(zip_file_content)).expect("Failed to open zip archive");

                let expected_aes_key_in_zip = AES_KEY_RSA_FILENAME;
                let expected_data_in_zip = format!("{}.aes", input_file_name);
                assert!(archive.by_name(expected_aes_key_in_zip).is_ok());
                assert!(archive.by_name(&expected_data_in_zip).is_ok());

                fs::remove_file(output_path).unwrap();
                let encrypted_dir_path = output_path_str.strip_suffix(".zip").unwrap();
                 if Path::new(encrypted_dir_path).exists() {
                    fs::remove_dir_all(encrypted_dir_path).unwrap();
                }
            },
        );
    }

    #[test]
    fn test_eventually_encrypt_result_missing_env_var() {
        let temp_input_file = NamedTempFile::new().expect("Failed to create temp input file");
        let input_file_path_str = temp_input_file.path().to_str().unwrap();

        with_vars(
            [(
                TeeSessionEnvironmentVariable::ResultEncryptionPublicKey.name(),
                None, // Ensure it's not set
            )],
            || {
                let result = eventually_encrypt_result(input_file_path_str);
                assert!(result.is_ok());
                assert_eq!(result.unwrap(), input_file_path_str); // Should return original path
            },
        );
    }

    #[test]
    fn test_eventually_encrypt_result_malformed_key() {
        let temp_input_file = NamedTempFile::new().expect("Failed to create temp input file");
        let input_file_path_str = temp_input_file.path().to_str().unwrap();

        with_vars(
            [(
                TeeSessionEnvironmentVariable::ResultEncryptionPublicKey.name(),
                Some("THIS_IS_A_MALFORMED_KEY"),
            )],
            || {
                let result = eventually_encrypt_result(input_file_path_str);
                assert!(result.is_err());
                // encrypt_data returns Ok(""), eventually_encrypt_result maps this to PostComputeEncryptionFailed
                // If plain_text_to_rsa_public_key fails first, that error might propagate.
                // Current plain_text_to_rsa_public_key returns PostComputeMalformedEncryptionPublicKey
                // encrypt_data then returns Ok("").
                // eventually_encrypt_result then maps Ok("") to PostComputeEncryptionFailed.
                match result.unwrap_err() {
                    ReplicateStatusCause::PostComputeEncryptionFailed => {
                        // This is the expected outcome if encrypt_data returned "" due to malformed key
                    }
                    ReplicateStatusCause::PostComputeMalformedEncryptionPublicKey => {
                        // This would be if eventually_encrypt_result directly propagated the error
                        // from plain_text_to_rsa_public_key if it happened before encrypt_data's logic.
                        // Based on current encrypt_data, it returns Ok(""), so this path is less likely.
                         panic!("Expected PostComputeEncryptionFailed due to encrypt_data's empty string return, but got PostComputeMalformedEncryptionPublicKey");
                    }
                    other => panic!("Unexpected error type: {:?}", other),
                }
            },
        );
    }
}
