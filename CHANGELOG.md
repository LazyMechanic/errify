# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2024-05-17
### Changed
- `WrapErr` trait no longer has a generic and takes `self` as the first argument.
### Removed
- Optional error type argument in macros.

## [0.2.2] - 2024-05-13
### Fixed
- Implementation `WrapErr` for `anyhow::Error` and `eyre::Report` no longer requires `std::error::Error` for generic `<E>` type, but requires `Debug + Display`.

## [0.2.1] - 2024-05-13
### Fixed
- Function visibility added to generated function.

## [0.2.0] - 2024-05-13
### Added
- `WrapErr` trait for a custom error type to save context.
- Support for a custom error type in macros.
### Changed
- Rename `context` -> `errify` and `with_context` -> `errify_with` macros.

## [0.1.0] - 2024-05-12
### Initial crate release