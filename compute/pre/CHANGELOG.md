# Changelog

## 0.1.0 (2025-09-04)


### Features

* add ResultSenderApiClient for sending computed files to worker API to resolve cyclic dependencies error ([3cc9ac9](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/3cc9ac9bbe9851e9b72023cd155d3badd71522e1))
* implement Default trait for DefaultPostComputeRunner and PreComputeApp structs to fix clippy linter errors ([8cf46cb](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/8cf46cb37fef5804f1ffef6131ebefeffebe22df))
* introduce shared crate for common dependencies and utilities ([c7c03a2](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/c7c03a2f521aec6382dc5445a8fa976e1052977e))
* introducing ComputeStage enum for better error handling ([7f2aaed](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/7f2aaedf961c796bf4de598df24bc16abc028c94))
* migrate to workspace monorepo ([0b93915](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/0b93915dc853d57651b673ad977ccc3f08bd45ac))
* organize pre- and post-compute project as lib crates ([abeca64](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/abeca6492cd5318305d0866eff25aca21f8e5cb9))


### Bug Fixes

* refactor code to fix Clippy linter errors ([30167bc](https://github.com/Natchica/iexec-tee_worker_pre_post_compute_rust/commit/30167bcff0dab08a795c703beb5395dfed6eb942))
