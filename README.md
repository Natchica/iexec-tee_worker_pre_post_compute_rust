# TEE Worker Compute Rust

This workspace contains TEE (Trusted Execution Environment) workers for distributed computing operations.

## Binaries

This project provides two main executables:

- **tee-worker-pre-compute**: Handles input preparation, validation, and encryption
- **tee-worker-post-compute**: Processes computation results and manages output delivery

## Usage

Run the binaries directly:

```bash
# Pre-compute operations
cargo run --bin tee-worker-pre-compute

# Post-compute operations
cargo run --bin tee-worker-post-compute
```

## Project Structure

- `pre-compute/`: Pre-computation logic and binary
- `post-compute/`: Post-computation logic and binary
- `shared/`: Common utilities and types used by both compute stages
