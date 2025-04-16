#![allow( dead_code )]

use min_ecs::*;

#[cfg_attr( feature = "serde", derive( serde::Serialize, serde::Deserialize ) )]
#[derive( Debug, Clone, Copy, PartialEq,  )]
pub struct Points ( u32 );

//---
#[cfg( feature = "serde" )]
minecs! {
	#[derive( serde::Serialize, serde::Deserialize )]
	world MinEcs;
	entity TestEntity ( f64, usize ) {
		bonus_fld: usize,
	}
	entity Character ( f64, usize ) {
		health: Points,
		armor: Points,
		incoming_damage: Vec< Points >,
	}
}
#[cfg( not( feature = "serde" ))]
minecs! {
	world MinEcs;
	entity TestEntity ( f64, usize ) {
		bonus_fld: usize,
	}
	entity Character ( f64, usize ) {
		health: Points,
		armor: Points,
		incoming_damage: Vec< Points >,
	}
}

pub fn main() {
	/*
	removal();
	// */
	
	//*
	iteration();
	// */
	
	//*
	run_system();
	// */
}

fn iteration () {
	let mut world = MinEcs::new();
	
	let entity_0 = world.new_entity::<TestEntity>();
	world.insert( entity_0, 67.0 );
	
	let entity_1 = world.new_entity::<TestEntity>();
	world.insert( entity_1, 422 );
	
	let entity_2 = world.new_entity::<TestEntity>();
	world.insert( entity_2, 2 );
	world.insert( entity_2, 10.0 );
	world.insert_fn( TestEntity::set_bonus_fld, entity_2, 1024 );
	
	println!( "ecs.iter::< usize >() => " );
	for v in world.iter::< usize >() {
		println!( "    {v:#?}" );
	}
	
	println!( "after iteration: " );
	for v in world.iter_mut::< usize >() {
		*v += 10;
		println!( "    {v:#?}" );
	}
	
	println!( "ecs.iter::< f64 >() => " );
	for v in world.iter::< f64 >() {
		println!( "    {v:#?}" );
	}
}

fn removal () {
	let mut world = MinEcs::new();
	
	let entity_0 = world.new_entity::< TestEntity >();
	world.insert( entity_0, 67.0 );
	
	let entity_1 = world.new_entity::< TestEntity >();
	world.insert( entity_1, 422 );
	
	let entity_2 = world.new_entity();
	world.insert( entity_2, 2 );
	world.insert( entity_2, 10.0 );
	world.insert_fn( TestEntity::set_bonus_fld, entity_2, 1024 );
	
	println!( "//------------------------------------------------------------------------------" );
	println!( "// before removal:\nentity_0 = {:#?}", world.entity( entity_0 ).unwrap() );
	
	let comp_id = world.entity( entity_0 ).unwrap().get().unwrap();
	let mut opt: Option< &f64 >;
	opt = world.get( comp_id );
	println!( "ecs.get( comp_id ) = {:#?}", opt );
	
	_ = world.remove::<f64>( entity_0 );
	
	println!( "//------------------------------------------------------------------------------" );
	println!( "// after removal:\nentity_0 = {:#?}", world.entity( entity_0 ).unwrap() );
	
	opt = world.get( comp_id );
	println!( "ecs.get( comp_id ) = {:#?}", opt );
	
	println!( "//------------------------------------------------------------------------------" );
	println!( "// before insert:\nentity_1 = {:#?}", world.entity( entity_1 ).unwrap() );
	world.insert( entity_1, 418.0 );
	
	println!( "//------------------------------------------------------------------------------" );
	println!( "// after insert:\nentity_1 = {:#?}", world.entity( entity_1 ).unwrap() );
	
	opt = world.get( comp_id );
	println!( "ecs.get( comp_id ) = {:#?}", opt );
	
	println!( "//------------------------------------------------------------------------------" );
	println!( "// before removing `bonus_fld` from:\nentity_2 = {:#?}", world.entity( entity_2 ).unwrap() );
	
	let res = world.remove_fn( TestEntity::remove_bonus_fld, entity_2 );
	println!( "\necs.remove_fn( TestEntity::remove_bonus_fld, entity_2 ) = {:#?}", res );
	
	println!( "//------------------------------------------------------------------------------" );
	println!( "// after removing `bonus_fld` from:\nentity_2 = {:#?}", world.entity( entity_2 ).unwrap() );
	
	
	world.insert_fn( TestEntity::set_bonus_fld, entity_0, 4096 );
	println!( "//------------------------------------------------------------------------------" );
	println!( "// after inserting `bonus_fld` to:\nentity_0 = {:#?}", world.entity( entity_0 ).unwrap() );
	let opt = world.get( world.entity( entity_0 ).unwrap().bonus_fld().unwrap() );
	println!( "ecs.get( comp_id ) = {:#?}", opt );
}

fn run_system () {
	let mut world = MinEcs::new();
	
	let entity_0 = world.new_entity();
	world.insert_fn( Character::set_health, entity_0, Points( 100 ) );
	world.insert_fn( Character::set_armor, entity_0, Points( 50 ) );
	world.insert_fn( Character::set_incoming_damage, entity_0, vec![ Points( 25 ), Points( 10 ), Points( 20 ) ] );
	
	let entity_1 = world.new_entity();
	world.insert_fn( Character::set_health, entity_1, Points( 100 ) );
	world.insert_fn( Character::set_incoming_damage, entity_1, vec![ Points( 25 ), Points( 10 ), Points( 20 ) ] );
	
	world.run_system( |ca, entity: &Character| {
		println!( "//------------------------------------------------------------------------------" );
		let mut new_hp;
		let mut new_armor;
		
		if let (Some( id_hp ), Some( id_armor ), Some( id_dmg )) = (entity.health(), entity.armor(), entity.incoming_damage()) {
			let hp = ca.get( id_hp ).unwrap().inner();
			let armor = ca.get( id_armor ).unwrap().inner();
			let dmg_vec = ca.get( id_dmg ).unwrap().inner();
			
			new_hp = hp.0;
			new_armor = Some( armor.0 );
			println!( "initial hp = {new_hp}" );
			println!( "initial armor = {}", armor.0 );
			
			for dmg in dmg_vec {
				if let Some( armor_val ) = new_armor {
					if armor_val > dmg.0 {
						new_armor = Some( armor_val - dmg.0 );
					} else if armor_val != 0 {
						let diff = dmg.0 - armor_val;
						new_armor = None;
						
						if hp.0 > diff {
							new_hp -= diff;
						} else {
							new_hp = 0;
							break;
						}
					}
				}
				 else if hp.0 > dmg.0 {
					new_hp -= dmg.0;
				} else {
					new_hp = 0;
					break;
				}
			}
			
		} else if let (Some( id_hp ), Some( id_dmg )) = (entity.health(), entity.incoming_damage()) {
			let hp = ca.get( id_hp ).unwrap().inner();
			let dmg_vec = ca.get( id_dmg ).unwrap().inner();
			
			new_hp = hp.0;
			new_armor = None;
			println!( "initial hp = {new_hp}" );
			
			for dmg in dmg_vec {
				if hp.0 > dmg.0 {
					new_hp -= dmg.0;
				} else {
					new_hp = 0;
					break;
				}
			}
		} else {
			return;
		}
		
		ca.get_mut( entity.health().unwrap() ).unwrap().inner_mut().0 = new_hp;
		println!( "hp = {new_hp}" );
		if let Some( val ) = new_armor {
			ca.get_mut( entity.armor().unwrap() ).unwrap().inner_mut().0 = val;
			println!( "armor = {val}" );
		} else if let Some( id ) = entity.armor() {
			_ = ca.remove( id );
			println!( "armor removed" );
		}
		
		ca.get_mut( entity.incoming_damage().unwrap() ).unwrap().inner_mut().clear();
	});
}


