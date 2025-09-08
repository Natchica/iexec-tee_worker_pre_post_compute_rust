# Changelog

## [0.4.1](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/compare/v0.4.0...v0.4.1) (2025-09-08)


### Bug Fixes

* **docs:** add missing period to comment in signer.rs ([48298e7](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/48298e7b544c54bdf9191ab9ac858c5a856cb32e))

## [0.4.1](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/compare/v0.4.0...v0.4.1) (2025-09-08)


### Bug Fixes

* **docs:** add missing period to comment in signer.rs ([48298e7](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/48298e7b544c54bdf9191ab9ac858c5a856cb32e))

## [0.4.0](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/compare/v0.3.0...v0.4.0) (2025-09-08)


### Features

* add ResultSenderApiClient for sending computed files to worker API to resolve cyclic dependencies error ([3cc9ac9](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/3cc9ac9bbe9851e9b72023cd155d3badd71522e1))
* **docs:** add missing periods to comments in computed_file.rs, pre_compute_args.rs, and signer.rs ([33e56be](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/33e56bef369549c1d1a8dcb5d01debd8fd9a9bb0))
* introduce shared crate for common dependencies and utilities ([c7c03a2](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/c7c03a2f521aec6382dc5445a8fa976e1052977e))
* introducing ComputeStage enum for better error handling ([7f2aaed](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/7f2aaedf961c796bf4de598df24bc16abc028c94))
* migrate to workspace monorepo ([0b93915](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/0b93915dc853d57651b673ad977ccc3f08bd45ac))
* **signer:** update comment formatting in signer.rs for clarity ([4eecbcb](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/4eecbcb0e4c2d71b390db06eb8602c903d2e3a64))


### Bug Fixes

* **docs:** update comment in signer.rs to remove trailing period ([c9090ea](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/c9090ea9059dbd218161bc9981813ee17677e514))
* refactor code to fix Clippy linter errors ([30167bc](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/30167bcff0dab08a795c703beb5395dfed6eb942))
* **signer:** remove commented sections and clean up test function formatting in signer.rs ([c956d8e](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/c956d8e2402700a2ace17b364e7ad3912a2ab0c8))
* **signer:** remove trailing period from documentation comment in signer.rs ([1f7fcc7](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/1f7fcc7a0b20a86899c78f4e071a84811731be78))
* **signer:** standardize formatting of test function signatures in signer.rs ([d99c372](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/d99c372dd786142592f8ca22b6ee40c67cc0ce73))

## [0.3.0](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/compare/shared-v0.2.0...shared-v0.3.0) (2025-09-08)


### Features

* **signer:** update comment formatting in signer.rs for clarity ([4eecbcb](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/4eecbcb0e4c2d71b390db06eb8602c903d2e3a64))


### Bug Fixes

* **signer:** remove trailing period from documentation comment in signer.rs ([1f7fcc7](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/1f7fcc7a0b20a86899c78f4e071a84811731be78))

## [0.2.1](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/compare/shared-v0.2.0...shared-v0.2.1) (2025-09-08)


### Bug Fixes

* **signer:** remove trailing period from documentation comment in signer.rs ([1f7fcc7](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/1f7fcc7a0b20a86899c78f4e071a84811731be78))

## [0.2.0](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/compare/shared-v0.1.0...shared-v0.2.0) (2025-09-08)


### Features

* add ResultSenderApiClient for sending computed files to worker API to resolve cyclic dependencies error ([3cc9ac9](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/3cc9ac9bbe9851e9b72023cd155d3badd71522e1))
* introduce shared crate for common dependencies and utilities ([c7c03a2](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/c7c03a2f521aec6382dc5445a8fa976e1052977e))
* introducing ComputeStage enum for better error handling ([7f2aaed](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/7f2aaedf961c796bf4de598df24bc16abc028c94))
* migrate to workspace monorepo ([0b93915](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/0b93915dc853d57651b673ad977ccc3f08bd45ac))


### Bug Fixes

* refactor code to fix Clippy linter errors ([30167bc](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/30167bcff0dab08a795c703beb5395dfed6eb942))
* **signer:** remove commented sections and clean up test function formatting in signer.rs ([c956d8e](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/c956d8e2402700a2ace17b364e7ad3912a2ab0c8))
* **signer:** standardize formatting of test function signatures in signer.rs ([d99c372](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/d99c372dd786142592f8ca22b6ee40c67cc0ce73))
