# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.4] - 2024-12-12

- Add a public archive note and state the last supported rust version `1.71.0-nightly (nightly-2023-04-19)`.
- Removed the CI.
- Removed dependency on ConstClosure, use `#[feature(const_closures)]` instead.

## [0.3.3] - 2022-11-10

### Fixes
- Updated dependencies.

## [0.3.2] - 2022-09-29

- Updated feature gates.

### Fixes
- Fixed clippy lints.

## [0.3.1] - 2022-09-25

Moved to [ink-feather-org](https://github.com/ink-feather-org/const_sort_rs).

### Fixes
- Fixed clippy lints.

## [0.3.0] - 2022-09-22

### Fixes
- Added `#[const_trait]` to the public traits as this is became a hard required.

## [0.2.1] - 2022-09-22

## [0.2.0] - 2022-09-22

## [0.1.0] - 2022-09-14

[Unreleased]: https://github.com/ink-feather-org/const_sort_rs/compare/v0.3.4...HEAD
[0.3.3]: https://github.com/ink-feather-org/const_sort_rs/compare/v0.3.3...v0.3.4
[0.3.3]: https://github.com/ink-feather-org/const_sort_rs/compare/v0.3.2...v0.3.3
[0.3.2]: https://github.com/ink-feather-org/const_sort_rs/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/ink-feather-org/const_sort_rs/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/ink-feather-org/const_sort_rs/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/ink-feather-org/const_sort_rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/ink-feather-org/const_sort_rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/ink-feather-org/const_sort_rs/releases/tag/v0.1.0
