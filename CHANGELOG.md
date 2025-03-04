# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## 0.1.1 - 2025.03.04

### Changed
- renamed `Component` methods from `borrow` and `borrow_mut` to `inner` and `inner_mut` respectively, to avoid confusion with `std::borrow` traits.
- renamed methods in trait `EcsMain` from `borrow_entity` and `borrow_mut_entity` to `entity` and `entity_mut` respectively.

Minor improvements and fixes.

## 0.1.0 - 2025.03.04
Initial release.
