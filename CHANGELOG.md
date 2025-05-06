# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## 0.2.0-rc.2 - 2025.05.06

### Changed
- removed unused bin section from cargo.toml
- fixed `minecs_common` incorrectly depending on serde by default, even when `min_ecs` was not.
- a few improvements and bug fixes in README and documentation.

## 0.2.0-rc.1 - 2025.04.17

### Removed
- world (previously referred to as ecs) no longer has methods for manipulating each named component of an entity. Use new world methods: `insert_fn` and `remove_fn` instead.
- sub-crate `minecs_common` dependency on `paste`.
### Changed
- reworked traits and macro syntax.
- a single world can manage multiple entities.
- identifier of 'CompVec' is generated automatically, by appending "Components" to worlds' identifier.
- world method `new_entity` allows 'turbofish' syntax to specify entity type.
- type `Component` is no longer generic over entity.
### Added
- world methods: `insert_fn`, `remove_fn` - their primary use is when manipulating named components.
- world method: `run_system_mut` which allows modifying both components and entity.

## 0.1.1 - 2025.03.04

### Changed
- renamed `Component` methods from `borrow` and `borrow_mut` to `inner` and `inner_mut` respectively, to avoid confusion with `std::borrow` traits.
- renamed methods in trait `EcsMain` from `borrow_entity` and `borrow_mut_entity` to `entity` and `entity_mut` respectively.

Minor improvements and fixes.

## 0.1.0 - 2025.03.04
Initial release.
