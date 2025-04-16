<p align="center">
	<a href="#description">Description</a> •
	<a href="#usage">Usage</a> •
	<a href="#installation">Installation</a> •
	<a href="#error-handling">Error Handling</a> •
	<a href="#versioning">Versioning</a> •
	<a href="#msrv-policy">MSRV policy</a> •
	<a href="#license">License</a>
</p>

# MinEcs ![Static Badge](https://img.shields.io/badge/MinEcs_MSRV-1.77-purple) ![Static Badge](https://img.shields.io/badge/Version-0.2.0-purple)

### **This project is NOT production ready.**


## Description

MinEcs is an Entity Component System library created primarily as a learning project.

The primary goals are to provide:
 - functionality suitable for small and hobby projects,
 - fully type stable, no `dyn`,
 - simple usage and convenience.


## Usage

```rust
// import types and traits, optionally `pub use` to re-export from crate / module
use min_ecs::*;

// the following macro will create the necessary types:
minecs!(
	#[derive( serde::Serialize, serde::Deserialize )]
	world MinEcs;
	entity TestEntity ( f64, usize ) {
		bonus_fld: usize,
	}
);

/*
`minecs!` usage:
1. optional derive attribute, ( on top of: `Debug`, `Clone`, `PartialEq` ) fe. `#[derive( serde::Serialize, serde::Deserialize )]`
2. world declaration, fe. `world MinEcs;`
	1. keyword `world`
	2. identifier - name of the world,
	3. semicolon
3. at least one entity declaration, fe. `entity TestEntity`
	1. keyword `entity`
	2. identifier - name of the entity,
	3. either or both:
		- parentheses `()` surrounding comma separated list of not-repeating types, followed either by curly braces or semicolon; fe. `( f64, usize );`,
		- curly braces `{}` surrounding comma separated field declarations in form: identifier, colon, type; fe. `names: Vec< Rc< str >>`.
*/

// create mutable instance
let mut world = MinEcs::new();

// create a new entity and get its' id
let entity_id_0 = world.new_entity();

// add component to the entity - in this case `f64`
world.insert( entity_id_0, 67.0 );

// World doesn't have dedicated insert methods for individual named fields.
// Instead, an `insert_fn` method must be used, where dedicated insert method of `E: Entity` is provided, followed by entity_id and component.
world.insert_fn( TestEntity::set_some_fld, entity_id_0, 422 );

// iteration over a single component
for comp in world.iter::< f64 >() {
	println!( "{comp:#?}" );
}

// accessing a specific entity
let entity = world.entity( entity_id_0 ).unwrap();

// accessing a generic component of the entity, component type specified in next line
let comp_id = entity.get().unwrap();

// get component from world
let comp: &f64 = world.get( comp_id ).unwrap();
// without specifying type of the `comp` we'd need to change the previous line to:
//`let comp_id: CompId< f64, TestEntity > = entity.get().unwrap();`

// running systems for each entity; may be re-worked in the future.
// currently requires function / closure which directly manipulates `&mut V: CompVec` and `&E: Entity`. Ecs runs such fn / closure for each entity.
world.run_system( |comp_vec, entity| {
	println!( "//------------------------------------------------------------------------------" );
	println!( "entity: {:#?}", entity );
	
	if let Some( id ) = entity.get() {
		let opt: Option< &Component< usize > > = comp_vec.get( id );
		println!( "comp_vec.get( id ) = {:#?}", opt );
	}
	if let Some( id ) = entity.get() {
		let opt: Option< &f64 > = comp_vec.get( id ).map( Component::inner );
		println!( "comp_vec.get( id ).map( Component::inner ) = {:#?}", opt );
	}
	if let Some( id ) = entity.some_fld() {
		let val = comp_vec.get( id ).unwrap().inner();
		println!( "comp_vec.get( id ).unwrap().inner() = {val}" );
	}
});
```


## Installation

Via github:
1. Download Zip archive (Code dropdown on main project page) and then unzip it.
2. In the cargo.toml add a dependency, adjusting the path as needed:
```TOML
[dependencies]
min_ecs = { path = "../min_ecs-master" }# Assuming your project and the unzipped archive are in the same folder
```


### Features

* **serde**
	provides `serde::Serialize` and `serde::Deserialize` impl on exported types. Types generated via macro `minecs!` will still need a derives section.


## Error Handling

MinEcs usually returns `Option`s, but some methods return `Result< _, EcsErr >` instead. The variants are as follow:
- `CompVecMissingComponent` - `Component` is present in `Entity`, but cannot be found in `CompVec`
- `EntityMissingComponent`: `Component` not present in `Entity`
- `NoSuchEntityId`: `Entity` with specified id does not exist
- `NoSuchCompId`: `Component` with specified id does not exist


## Versioning

This project uses <a href="https://semver.org">SemVer 2.0.0</a>


## MSRV policy

During development MSRV may be changed at any time. It will increase the minor version.<br>
Upon reaching 1.0.0, increasing MSRV will be considered a breaking change, and will increase the major version.


## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
