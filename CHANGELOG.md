# Changelog

## 0.1.0 (2025-09-04)


### Features

* Add app_runner crate ([#8](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/issues/8)) ([d351e5d](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/d351e5d8e0a7b32e15fada268d279dea8ab379b6))
* add Docker support with multi-stage builds for pre- and post-compute binaries and configure CI/CD workflows ([85ed568](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/85ed56842275e76125e19e94e8216a30bf5af770))
* Add env_logger logging implementation ([#18](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/issues/18)) ([5b4a917](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/5b4a917b9e5a015574f6dcfdc8a27158d0b31fe1))
* add ExitMode enum to represent process exit states ([#26](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/issues/26)) ([4de72d5](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/4de72d584f7b2b864451965cf445c63427f8759b))
* Add hash_utils crate ([#4](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/issues/4)) ([5b1c515](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/5b1c51578a159a0ac3d8572171a0b7e2c390b3f4))
* add ResultSenderApiClient for sending computed files to worker API to resolve cyclic dependencies error ([3cc9ac9](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/3cc9ac9bbe9851e9b72023cd155d3badd71522e1))
* Add signer crate ([#6](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/issues/6)) ([793c15c](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/793c15cd21b5024354403a4f7b04a8a023d0478e))
* enhance error log reporting for REST calls ([#20](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/issues/20)) ([3d69879](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/3d6987909c8ddf22a615fa4d5e6cb42c57f24e75))
* handle result encryption ([#19](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/issues/19)) ([a95dfd2](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/a95dfd24ad8f36362ab09375120bf13dd0763537))
* implement Default trait for DefaultPostComputeRunner and PreComputeApp structs to fix clippy linter errors ([8cf46cb](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/8cf46cb37fef5804f1ffef6131ebefeffebe22df))
* implement result upload to Dropbox ([#29](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/issues/29)) ([973108a](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/973108a7b41f7e757c031bc7e3355c959a434002))
* improve logger initialization to set a default log level ([4781363](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/4781363de5ef9f4cd2c729802b16cb25e81ea5e6))
* introduce shared crate for common dependencies and utilities ([c7c03a2](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/c7c03a2f521aec6382dc5445a8fa976e1052977e))
* introducing ComputeStage enum for better error handling ([7f2aaed](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/7f2aaedf961c796bf4de598df24bc16abc028c94))
* migrate to workspace monorepo ([0b93915](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/0b93915dc853d57651b673ad977ccc3f08bd45ac))
* organize pre- and post-compute project as lib crates ([abeca64](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/abeca6492cd5318305d0866eff25aca21f8e5cb9))
* Read and update computed.json file ([#10](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/issues/10)) ([83715ec](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/83715ec9ece298d35b821b323bfccdcc1f2fe139))
* Sign computed.json and send it to worker ([#13](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/issues/13)) ([1b53250](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/1b53250448bf167436a95a080caa932491cb786f))
* upload non encrypted result ([#16](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/issues/16)) ([fb17048](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/fb17048dc6ad6ba77af7e49e3a6d9de3ba5bd075))


### Bug Fixes

* format ComputedFile to kebab-case to match API ([#23](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/issues/23)) ([9635f8c](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/9635f8c7793805b23c978631e6d298e4609ec9e0))
* redirect log output from stderr to stdout ([#24](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/issues/24)) ([17d6d34](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/17d6d345d1d93beb5583a2da97a0f9695661e84e))
* Refactor `TeeSessionEnvironmentVariable` enum fields to PascalCase ([#12](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/issues/12)) ([afaff92](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/afaff920fe40a0c4e23108f74b560b0e2bf2ecf3))
* refactor code to fix Clippy linter errors ([30167bc](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/30167bcff0dab08a795c703beb5395dfed6eb942))
* Remove PostComputeError wrapper  ([#9](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/issues/9)) ([7b695a3](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/7b695a39be3fe33c6f27bf26e11ee4c41a80a59e))

## Changelog

All notable changes to the TEE Worker Pre/Post Compute Rust workspace will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
