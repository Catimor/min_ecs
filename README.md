<p align="center">
	<a href="#description">Description</a> •
	<a href="#usage">Usage</a> •
	<a href="#installation">Installation</a> •
	<a href="#error-handling">Error Handling</a> •
	<a href="#versioning">Versioning</a> •
	<a href="#msrv-policy">MSRV policy</a> •
	<a href="#license">License</a>
</p>

# MinEcs ![Static Badge](https://img.shields.io/badge/MinEcs_MSRV-1.77-purple) ![Static Badge](https://img.shields.io/badge/Version-0.1.1-purple)

### **This project is personal/experimental and NOT production ready.**


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
	ecs MinEcs< CompArray, TestEntity> {
		types [ f64, usize, /* --ect-- */ ]
		some_fld: usize, // trailing comma is optional
	}
);

/*
`minecs!` usage:
1. optional derive attribute, ( on top of: `Debug`, `Clone`, `PartialEq` ) fe. `#[derive( serde::Serialize, serde::Deserialize )]`
2. ecs declaration, fe. `ecs MinEcs< CompArray, TestEntity >`
	1. keyword `ecs`
	2. identifier - name of the ecs,
	3. angled braces surrounding two identifiers separated by a comma: component_array and entity,
3. curly braces `{}` surrounding component declarations ( either or both )
	- keyword `types` followed by square brackets `[]` surrounding comma separated list of not-repeating types; fe. `types [usize, f64]`,
	- comma separated field declarations, such as for struct, in form: identifier, colon, type; fe. `names: Vec< Rc< str >>`.
*/

// create mutable instance
let mut ecs = MinEcs::new();

// create a new entity and get its' id
let entity_id_0 = ecs.new_entity();

// add component to the entity - in this case `f64`
ecs.insert( entity_id_0, 67.0 );

// named fields have dedicated insert methods
ecs.insert_some_fld( entity_id_0, 422 );

// iteration over a single component
for comp in ecs.iter::< f64 >() {
	println!( "{comp:#?}" );
}

// accessing a specific entity
let entity = ecs.borrow_entity( entity_id_0 ).unwrap();

// accessing a generic component of the entity, component type specified in next line
let comp_id = entity.get().unwrap();

// get component from ecs
let comp: &f64 = ecs.get( comp_id ).unwrap();
// without specifying type of the `comp` the previous line would need to be changed to:
//`let comp_id: CompId< f64, TestEntity > = entity.get().unwrap();`

// running systems for each entity (may be re-worked in the future)
// currently requires function / closure which directly manipulates `&mut V: CompVec` and individual `&E: Entity`. Ecs runs such fn / closure for each entity.
ecs.run_system( |comp_vec, entity| {
	println!( "//------------------------------------------------------------------------------" );
	println!( "entity: {:#?}", entity );
	
	if let Some( id ) = entity.get() {
		let opt: Option< &Component< usize, TestEntity> > = comp_vec.get( id );
		println!( "comp_vec.get( id ) = {:#?}", opt );
	}
	if let Some( id ) = entity.get() {
		let opt: Option< &f64 > = comp_vec.get( id ).map( Component::borrow );
		println!( "comp_vec.get( id ).map( Component::borrow ) = {:#?}", opt );
	}
	if let Some( id ) = entity.some_fld() {
		let val = comp_vec.get( id ).unwrap().borrow();
		println!( "comp_vec.get( id ).unwrap().borrow() = {val}" );
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
	provides `serde::Serialize` and `serde::Deserialize` impl on exported types. Types generatedd via macro `minecs!` will need an attribute section.
	

## Error Handling

MinEcs usually returns `Option`s, but some methods return `EcsErr` instead. The variants are as follow:
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
