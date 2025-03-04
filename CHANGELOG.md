# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Changed
- `Component` methods: `borrow` and `borrow_mut` are renamed to `inner` and `inner_mut` respectively to avoid confusion with `std::borrow` traits.
- methods in trait `EcsMain`: `borrow_entity` and `borrow_mut_entity` are renaimed to `entity` and `entity_mut` respectively to avoid confusion with `std::borrow` traits.

## 0.1.0 - 2025.03.04
Initial release.
