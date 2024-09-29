# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2024-09-29

### Added

- Support of `--no-deps` flag to disable dependency resolution

## [1.0.0] - 2024-09-27

### Changed

- Parsing and resolving dependencies from using `uv` as library to running `uv` in subprocess

### Added

- Support of reading dependencies from Python environment
- Support of reading dependencies from `pyproject.toml`
- Support of `index-url`  

## [0.2.0] - 2024-07-30

### Added

- Support of `json` formatter
- Support of `columns` formatter
- Support of `cyclonedx-json` formatter
- `cli` configuration validator

## [0.1.0] - 2024-07-24

### Added

- Initial version

[1.1.0]: https://github.com/fyusifov/uv-audit/pull/9
[1.0.0]: https://github.com/fyusifov/uv-audit/pull/7
[0.2.0]: https://github.com/fyusifov/uv-audit/pull/3
[0.1.0]: https://github.com/fyusifov/uv-audit/tree/0.1.0
