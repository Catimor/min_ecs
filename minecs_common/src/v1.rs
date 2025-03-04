pub use paste;
//------------------------------------------------------------------------------
// --Traits

// array of Vec< Component< T >>
pub trait CompVec {
	fn new () -> Self;
	fn shrink ( &mut self );
}

// per T fn
pub trait CompVecFn< T, E >: CompVec
where
	EntityId< E >: Clone,
{
	fn insert ( &mut self, item: Component< T, E > ) -> CompId< T, E >;
	fn remove ( &mut self, id: CompId< T, E > ) -> Result< (), EcsErr >;
	
	fn get ( &self, id: CompId< T, E > ) -> Option< &Component< T, E > >;
	fn get_mut ( &mut self, id: CompId< T, E > ) -> Option< &mut Component< T, E > >;
	
	fn len ( &self, _: std::marker::PhantomData< T > ) -> usize;
	
	fn iter ( &self ) -> CompIter< T, E >;// core::slice::Iter< 'a, Component< T, E > >
	fn iter_mut ( &mut self ) -> CompIterMut< T, E >;// core::slice::IterMut< 'a, Component< T, E > >
}

// ECS -------------------------------------------------------------------------

// per E fn
pub trait EcsMain< E > {
	/// Attempts to reduce memory usage by calling `Vec::shrink_to_fit` on every component vector.
	fn shrink ( &mut self );
	
	/// Creates and returns a new entity id.
	fn new_entity ( &mut self ) -> EntityId< E >;
	
	
	/// Attempts to borrow entity with specified id.
	/// On success returns `Some( &Entity )` or `None` otherwise.
	fn borrow_entity ( &self, id: EntityId< E > ) -> Option< &E >;
	
	/// Attempts to mutably borrow entity with specified id.
	/// On success returns `Some( &mut Entity )` or `None` otherwise.
	fn borrow_mut_entity ( &mut self, id: EntityId< E > ) -> Option< &mut E >;
	
	
	/// Returns true if component with given id exists, or false otherwise.
	fn has_component< T > ( &self, id: CompId< T, E > ) -> bool
	where
		E: EntityFn<T>
	;
	//fn insert_raw_entity< T: RawEntity< E > > ( &mut self, raw_entity: T ) -> EntityId< E >;
}

// per T fn
pub trait EcsCompFn< T, E > {
	/// Attempts to add component to the entity, potentially discarding the previous component.
	/// On success returns `Some( CompId< T, E > )` or `None` otherwise.
	fn insert ( &mut self, id: EntityId< E >, item: T ) -> Option< CompId< T, E >>;
	
	
	/// Attempts to borrow component with specified id.
	/// On success returns `Some( &T )` or `None` otherwise.
	fn get ( &self, id: CompId< T, E > ) -> Option< &T >;
	
	/// Attempts to mutably borrow component with specified id.
	/// On success returns `Some( &mut T )` or `None` otherwise.
	fn get_mut ( &mut self, id: CompId< T, E > ) -> Option< &mut T >;
	
	
	/// Attempts to call a function `Fn( &T ) -> U` on component with specified id.
	/// On success returns `Some( U )` or `None` otherwise.
	fn call< F, U > ( &self, id: CompId< T, E >, fcn: F ) -> Option< U >
	where
		F: Fn( &T ) -> U
	;
	
	/// Attempts to call a function `FnMut( &mut T ) -> U` on component with specified id.
	/// On success returns `Some( U )` or `None` otherwise.
	fn call_mut< F, U > ( &mut self, id: CompId<T, E>, fcn: F ) -> Option< U >
	where
		F: FnMut( &mut T ) -> U
	;
	
}

pub trait EcsFn< V, E > {
	/// Returns an iterator over the slice.
	/// 
	/// The iterator yields all items from start to end.
	fn iter< T > ( &self ) -> CompIter< T, E >
	where
		V: CompVecFn< T, E >,
		E: Clone,
	;
	
	/// Returns an iterator that allows modifying each value.
	/// 
	/// The iterator yields all items from start to end.
	fn iter_mut< T > ( &mut self ) -> CompIterMut< T, E >
	where
		V: CompVecFn< T, E >,
		E: Clone,
	;
	
	/// Attempts to remove specified component.
	/// On success returns `Ok(())` or `EcsErr` otherwise.
	fn remove< T > ( &mut self, e_id: EntityId< E > ) -> Result< (), EcsErr >
	where
		V: CompVecFn< T, E >,
		E: EntityFn< T > + Clone,
	;
	
	/// Calls a provided function or closure for each entity in ecs.
	/// FnMut gets access to `&mut V: CompVec` and `&E: Entity` during each iteration.
	/// 
	/// ```rust
	/// #use min_ecs::*;
	/// minecs!( ecs MinEcs< CompArray, TestEntity> { types [ f64, usize, ] some_fld: usize } );
	/// 
	/// let eid_0 = ecs.new_entity();
	/// let eid_1 = ecs.new_entity();
	/// 
	/// ecs.insert( eid_0, 20 );
	/// ecs.insert( eid_0, 67.0 );
	/// 
	/// ecs.insert( eid_1, 49 );
	/// ecs.insert_some_fld( eid_1, 422 );
	/// 
	/// ecs.run_system( |comp_vec, entity| {
	/// 	println!( "//------------------------------------------------------------------------------" );
	/// 	println!( "entity: {:#?}", entity );
	/// 	
	/// 	if let Some( id ) = entity.get() {
	/// 		let opt: Option< &Component< usize, TestEntity> > = comp_vec.get( id );
	/// 		println!( "comp_vec.get( id ) = {:#?}", opt );
	/// 	}
	/// 	if let Some( id ) = entity.get() {
	/// 		let opt: Option< &f64 > = comp_vec.get( id ).map( Component::borrow );
	/// 		println!( "comp_vec.get( id ).map( Component::borrow ) = {:#?}", opt );
	/// 	}
	/// 	if let Some( id ) = entity.some_fld() {
	/// 		let val = comp_vec.get( id ).unwrap().borrow();
	/// 		println!( "comp_vec.get( id ).unwrap().borrow() = {val}" );
	/// 	}
	/// });
	/// ```
	fn run_system< F: FnMut( &mut V, &E )> ( &mut self, system_fn: F );
}

// Entity ----------------------------------------------------------------------

pub trait Entity {
	/// Creates a new empty `Entity`.
	fn new () -> Self;
}

// per T fn
pub trait EntityFn< T >
where
	Self: Sized
{
	/// Associates the specified component id with this entity.
	/// Returns the old id in `Some( CompId< T, Self > )` if present or `None` otherwise.
	fn set ( &mut self, id: CompId< T, Self > ) -> Option< CompId< T, Self >>;
	
	/// Associates the specified component id if it is `Option::Some` with this entity.
	/// Returns the old id in `Some( CompId< T, Self > )` if present or `None` otherwise.
	fn try_set ( &mut self, id: Option< CompId< T, Self >> ) -> Option< CompId< T, Self >>;
	
	/// Returns the component id in `Some( CompId< T, Self > )` if present or `None` otherwise.
	fn get ( &self ) -> Option< CompId< T, Self > >;
	
	/// Sets the component id to `None`.
	/// Returns the component id in `Some( CompId< T, Self > )` if present or `None` otherwise.
	fn remove ( &mut self ) -> Option< CompId< T, Self > >;
}

// RawEntity -------------------------------------------------------------------

/*
pub trait RawEntity< E: Entity > {
	/// Creates a new empty `RawEntity`.
	fn new () -> Self;
}

// per T fn
pub trait RawEntityFn< T, E >
where
	E: Entity,
	Self: Sized,
{
	/// Associates the specified component id with this entity.
	/// Returns the old id in `Some( CompId< T, Self > )` if present or `None` otherwise.
	fn set ( &mut self, item: T ) -> Option< T >;
	
	/// Associates the specified component id if it is `Option::Some` with this entity.
	/// Returns the old id in `Some( CompId< T, Self > )` if present or `None` otherwise.
	fn try_set ( &mut self, item: Option< T > ) -> Option< T >;
	
	/// Returns the component id in `Some( CompId< T, Self > )` if present or `None` otherwise.
	fn get ( &self ) -> Option< T >;
}
// */

// --Traits
//------------------------------------------------------------------------------
// struct - Component

#[cfg_attr( feature = "serde", derive( serde::Serialize, serde::Deserialize ) )]
#[derive( Debug, Clone, PartialEq )]
pub struct Component< T, E > {
	id: EntityId< E >,
	inner: T,
}

impl< T, E > Component< T, E >
where
	T: Clone,
	EntityId< E >: Copy,
{
	/// Creates a new component.
	/// 
	/// # Correctness
	/// 
	/// The association between component and entity is incomplete at this point.
	/// 
	/// The process of adding a `Component` to the `Entity` is as follows:
	/// 
	/// 1. new `Entity` registered in ECS -> `EntityId`,
	/// 1. new `Component` created + `EntityId` stored in `Component`, <- this step
	/// 1. `Component` registered in ECS -> `CompId`,
	/// 1. `CompId` stored in `Entity`.
	pub fn new ( id: EntityId< E >, value: T ) -> Self {
		Self {
			id,
			inner: value,
		}
	}
	
	/// Used to overwrite `self` in place, instead of first deallocating old value and then allocating a new one.
	/// 
	/// # Correctness
	/// 
	/// The association between component and entity is incomplete at this point.
	/// 
	/// Please refer to `Component::new` for details.
	pub fn overwrite ( &mut self, item: Self ) {
		self.id = item.id;
		self.inner = item.inner;
	}
	
	/// Returns id of the entity this component is associated to.
	pub fn id ( &self ) -> EntityId< E > {
		self.id
	}
	
	/// Returns the contained component, consuming the `self` value.
	pub fn unwrap ( self ) -> T {
		self.inner
	}
	
	/// Borrows the contained component.
	pub fn borrow ( &self ) -> &T {
		&self.inner
	}
	
	/// Mutably borrows the contained component.
	pub fn borrow_mut ( &mut self ) -> &mut T {
		&mut self.inner
	}
}

// struct - Component
//------------------------------------------------------------------------------
// struct - EntityId

#[cfg_attr( feature = "serde", derive( serde::Serialize, serde::Deserialize ) )]
#[derive( Debug, Clone, PartialEq )]
pub struct EntityId< E > {
	id: usize,
	marker: std::marker::PhantomData< E >,
}

impl< E > EntityId< E > {
	/// Creates a new entity id.
	/// 
	/// # Correctness
	/// 
	/// The returned id is not automatically registered in the ECS and may be invalid.
	/// 
	/// The process of adding a `Component` to the `Entity` is as follows:
	/// 
	/// 1. new `Entity` registered in ECS -> `EntityId`, <- this step
	/// 1. new `Component` created + `EntityId` stored in `Component`,
	/// 1. `Component` registered in ECS -> `CompId`,
	/// 1. `CompId` stored in `Entity`.
	pub fn new ( value: usize ) -> Self {
		Self {
			id: value,
			marker: std::marker::PhantomData::< E >,
		}
	}
}

impl< E > From< EntityId< E > > for usize {
	fn from( value: EntityId< E > ) -> Self {
		value.id
	}
}
impl< E > From< usize > for EntityId< E > {
	fn from( value: usize ) -> Self {
		Self::new( value )
	}
}

impl< E: Clone > Copy for EntityId< E > {}

// struct - EntityId
//------------------------------------------------------------------------------
// struct - CompId

#[cfg_attr( feature = "serde", derive( serde::Serialize, serde::Deserialize ) )]
#[derive( Debug, Clone, PartialEq )]
pub struct CompId< T, E > {
	id: usize,
	comp_marker: std::marker::PhantomData< T >,
	entity_marker: std::marker::PhantomData< E >,
}

impl< T, E > CompId< T, E > {
	/// Creates a new component id.
	/// 
	/// # Correctness
	/// 
	/// The returned id is not automatically registered in the ECS and may be invalid.
	/// 
	/// The process of adding a `Component` to the `Entity` is as follows:
	/// 
	/// 1. new `Entity` registered in ECS -> `EntityId`,
	/// 1. new `Component` created + `EntityId` stored in `Component`,
	/// 1. `Component` registered in ECS -> `CompId`, <- this step
	/// 1. `CompId` stored in `Entity`.
	pub fn new ( id: usize ) -> Self {
		Self {
			id,
			comp_marker: std::marker::PhantomData::< T >,
			entity_marker: std::marker::PhantomData::< E >,
		}
	}
}

impl< T, E > From< CompId< T, E > > for usize {
	fn from( value: CompId< T, E > ) -> Self {
		value.id
	}
}
impl< T, E > From< usize > for CompId< T, E > {
	fn from( value: usize ) -> Self {
		Self::new( value )
	}
}

impl< T: Clone, E: Clone > Copy for CompId< T, E > {}

// struct - CompId
//------------------------------------------------------------------------------
// struct - CompIter

#[derive( Debug )]
pub struct CompIter< 'a, T, E > {
	data: core::slice::Iter< 'a, Component< T, E > >,
}

impl< 'a, T, E > From< core::slice::Iter< 'a, Component< T, E > > > for CompIter< 'a, T, E > {
	fn from( value: core::slice::Iter< 'a, Component< T, E > > ) -> Self {
		Self {
			data: value,
		}
	}
}

impl< 'a, T: Clone, E: Clone > Iterator for CompIter< 'a, T, E > {
	type Item = &'a T;
	
	fn next( &mut self ) -> Option< Self::Item > {
		self.data.next().map( Component::borrow )
	}
}

// struct - CompIter
//------------------------------------------------------------------------------
// struct - CompIterMut

#[derive( Debug )]
pub struct CompIterMut< 'a, T, E > {
	data: core::slice::IterMut< 'a, Component< T, E > >,
}

impl< 'a, T, E > From< core::slice::IterMut< 'a, Component< T, E > > > for CompIterMut< 'a, T, E > {
	fn from( value: core::slice::IterMut< 'a, Component< T, E > > ) -> Self {
		Self {
			data: value,
		}
	}
}

impl< 'a, T: Clone, E: Clone > Iterator for CompIterMut< 'a, T, E > {
	type Item = &'a mut T;
	
	fn next( &mut self ) -> Option< Self::Item > {
		self.data.next().map( Component::borrow_mut )
	}
}

// struct - CompIterMut
//------------------------------------------------------------------------------
// enum - EcsErr

#[derive( Debug, Clone, PartialEq,  )]
pub enum EcsErr {
	CompVecMissingComponent( usize, usize ), // `Component` present in `Entity` cannot be found in `CompVec`
	EntityMissingComponent( usize ), // `Component` not present in `Entity`
	NoSuchEntityId( usize ), // `Entity` with specified id does not exist
	NoSuchCompId( usize ), // `Component` with specified id does not exist
}

impl std::fmt::Display for EcsErr {
	fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result {
		match self {
			Self::CompVecMissingComponent( e_id, c_id ) => format!( "component with id: {c_id} is present in entity with id {e_id}, but cannot be found in CompVec" ),
			Self::EntityMissingComponent( id ) => format!( "missing component in entity with id: {id}" ),
			Self::NoSuchEntityId( id ) => format!( "entity with id: {id} does not exist" ),
			Self::NoSuchCompId( id ) => format!( "component with id: {id} does not exist" ),
		}.fmt(f)
	}
}
impl std::error::Error for EcsErr {}

// enum - EcsErr
//------------------------------------------------------------------------------
// macro - min_ecs!

#[macro_export]
/// This macro is intended for internal use only, it is automatically called by `new_minecs!`.
/// 
/// # Usage
/// 
/// `$name`: identifier of the new ECS - must be new<br>
/// // comma<br>
/// `$ca`: identifier of existing type which implements: `min_ecs::CompVec`<br>
/// // comma<br>
/// `$entity`: identifier of existing type which implements: `min_ecs::Entity`<br>
/// // optional additional fields section: `$( , $fld_name: ident : $t: ty )*`<br>
/// 	comma<br>
/// 	`$fld_name`: identifier of the new field<br>
/// 	colon<br>
/// 	`$t`: type of the new field, must be present in `$ca`<br>
/// // optional derives section: `; $( $derives: ty ),+`<br>
/// 	semicolon<br>
/// 	`$derives`: identifier (fe. `Hash`) or path (fe. `serde::Serialize`), entries must be separated by comma<br>
/// // optional trailing comma<br>
macro_rules! new_ecs {
	( $name: ident, $ca: ident, $entity: ident $( , $fld_name: ident : $t: ty )* $(,)? ) => {
		$crate::new_ecs!( @inner, $name, $ca, $entity $( , $fld_name : $t )*; Debug, Clone, PartialEq, );
	};
	( $name: ident, $ca: ident, $entity: ident $( , $fld_name: ident : $t: ty )* ; $( $derives: ty ),+ $(,)? ) => {
		$crate::new_ecs!( @inner, $name, $ca, $entity $( , $fld_name : $t )*; $( $derives ),+ );
	};
	( @inner, $name: ident, $ca: ident, $entity: ident $( , $fld_name: ident : $t: ty )* ; $( $derives: ty ),+ $(,)? ) => {
		#[derive( $( $derives ),+ )]
		pub struct $name {
			entities: Vec< $entity >,
			components: $ca,
		}
		
		impl EcsMain< $entity > for $name where
			EntityId< $entity >: Copy,
		{
			fn shrink ( &mut self ) {
				self.components.shrink();
			}
			
			fn new_entity ( &mut self ) -> EntityId< $entity > {
				let idx = self.entities.len();
				self.entities.push( $entity::new() );
				EntityId::from( idx )
			}
			
			fn borrow_entity ( &self, id: EntityId< $entity > ) -> Option< &$entity > {
				self.entities.get( usize::from( id ) )
			}
			
			fn borrow_mut_entity ( &mut self, id: EntityId< $entity > ) -> Option< &mut $entity > {
				self.entities.get_mut( usize::from( id ) )
			}
	
			fn has_component<T> ( &self, id: CompId< T, $entity > ) -> bool
			where
			$entity: EntityFn<T>
			{
				if let Some( entity ) = self.entities.get( usize::from( id ) ) {
					entity.get().is_some()
				} else {
					false
				}
			}
		}
		
		impl< T > EcsCompFn< T, $entity > for $name where
			$ca: CompVec + CompVecFn< T, $entity >,
			$entity: Entity + EntityFn< T >,
			EntityId< $entity >: Copy,
			CompId< T, $entity >: Copy,
			T: Clone,
		{
			fn get ( &self, id: CompId< T, $entity > ) -> Option< &T > {
				self.components.get( id ).map( Component::borrow )
			}
	
			fn get_mut ( &mut self, id: CompId< T, $entity > ) -> Option< &mut T > {
				self.components.get_mut( id ).map( Component::borrow_mut )
			}
			
			fn call< F: Fn( &T ) -> U, U > ( &self, id: CompId< T, $entity >, fcn: F ) -> Option< U > {
				if let Some( comp ) = self.get( id ) {
					Some( fcn( comp ) )
				} else {
					None
				}
			}
			
			fn call_mut< F: FnMut( &mut T ) -> U, U > ( &mut self, id: CompId< T, $entity >, mut fcn: F ) -> Option< U > {
				if let Some( comp ) = self.get_mut( id ) {
					Some( fcn( comp ) )
				} else {
					None
				}
			}
			
			fn insert ( &mut self, id: EntityId< $entity >, item: T ) -> Option< CompId< T, $entity >> {
				if let Some( entity ) = self.entities.get_mut( usize::from( id ) ) {
					let comp = Component::new( id, item );
					
					let comp_id = self.components.insert( comp );
					entity.set( comp_id );
					
					Some( comp_id )
				} else {
					None
				}
			}
		}
		
		impl EcsFn< $ca, $entity > for $name where
			$ca: CompVec,
			$entity: Entity,
		{
			fn iter< T > ( &self ) -> CompIter< T, $entity >
			where
				$ca: CompVec + CompVecFn< T, $entity >,
			{
				self.components.iter()
			}
			
			fn iter_mut< T > ( &mut self ) -> CompIterMut< T, $entity >
			where
				$ca: CompVec + CompVecFn< T, $entity >,
			{
				self.components.iter_mut()
			}
			
			fn remove<T> ( &mut self, e_id: EntityId< $entity > ) -> Result< (), EcsErr >
			where
				$ca: CompVec + CompVecFn< T, $entity >,
				$entity: EntityFn< T >,
			{
				if let Some( entity ) = self.entities.get_mut( usize::from( e_id ) ) {
					let comp_id = entity.get();
					//let comp_id = entity.remove();
					if let Some( cid ) = comp_id {
						let out = self.components.remove( cid );
						if out.is_ok() {
							entity.remove();
						}
						
						out
					} else {
						Err( EcsErr::EntityMissingComponent( usize::from( e_id ) ))
					}
				} else {
					Err( EcsErr::NoSuchEntityId( usize::from( e_id ) ))
				}
			}
			
			fn run_system< F: FnMut( &mut $ca, &$entity )> ( &mut self, mut system_fn: F ) {
				for ent in self.entities.iter() {
					system_fn( &mut self.components, ent );
				}
			}
			
		}
		
		impl $name {
			pub fn new () -> Self {
				Self {
					entities: Vec::new(),
					components: $ca::new(),
				}
			}
			
			//pub fn borrow_mut ( &mut self ) -> ( &mut $ca, &Vec< $entity > ) {
			//	( &mut self.components, &self.entities )
			//}
		}
		
		$crate::paste::paste!{
			impl $name {
				$(
					/// Attempts to add component to the entity, potentially discarding the previous component.
					/// On success returns `Some( CompId< T, E > )` or `None` otherwise.
					pub fn [<insert_ $fld_name>] ( &mut self, id: EntityId< $entity >, item: $t ) -> Option< CompId< $t, $entity >> {
						if let Some( entity ) = self.entities.get_mut( usize::from( id ) ) {
							let comp = Component::new( id, item );
							
							let comp_id = self.components.insert( comp );
							_ = entity.[<set_ $fld_name>]( comp_id );
							
							Some( comp_id )
						} else {
							None
						}
					}
					
					/// Attempts to remove specified component.
					/// On success returns `Ok(())` or `EcsErr` otherwise.
					pub fn [<remove_ $fld_name>] ( &mut self, id: EntityId< $entity > ) -> Result< (), EcsErr > {
						let e_id = usize::from( id );
						if let Some( entity ) = self.entities.get_mut( e_id ) {
							let opt = entity.$fld_name();
							
							if let Some( cid ) = opt {
								let out = self.components.remove( cid );
								
								if out.is_ok() {
									_ = entity.[<remove_ $fld_name>]();
								}
								
								out
							} else {
								Err( EcsErr::EntityMissingComponent( usize::from( e_id ) ))
							}
						} else {
							Err( EcsErr::NoSuchEntityId( e_id ) )
						}
					}
				)*
			}
		}
	};
}

// macro - min_ecs!
//------------------------------------------------------------------------------
// macro - from_raw_entity!

//macro_rules! from_raw_entity {
//	( $name: ident, $ca: ident, $entity: ident, $raw_entity: ident $( , $fld_name: ident : $t: ty )+ ; $( $derives: ty ),+ $(,)? ) => {
//		
//	};
//	( $name: ident, $ca: ident, $entity: ident, $raw_entity: ident $( , $fld_name: ident : $t: ty )+ ; $( $derives: ty ),+ $(,)? ) => {
//		
//	};
//}

// macro - from_raw_entity!
//------------------------------------------------------------------------------
// --TestImpl

/*
#[allow( dead_code )]
struct TestEcs< V: CompVec, E > {
	entities: Vec< E >,
	components: V
}

impl< V, E > EcsMain< E > for TestEcs< V, E > where
	V: CompVec,
	E: Entity,
	EntityId< E >: Copy,
{
	fn shrink ( &mut self ) {
		self.components.shrink();
	}
	
	fn new_entity ( &mut self ) -> EntityId< E > {
		let idx = self.entities.len();
		self.entities.push( E::new() );
		EntityId::from( idx )
	}
	
	fn borrow_entity ( &self, id: EntityId< E > ) -> Option< &E > {
		self.entities.get( usize::from( id ) )
	}
	
	fn borrow_mut_entity ( &mut self, id: EntityId< E > ) -> Option< &mut E > {
		self.entities.get_mut( usize::from( id ) )
	}
	
	fn has_component< T > ( &self, id: CompId< T, E > ) -> bool where
		E: EntityFn< T >,
	{
		if let Some( entity ) = self.entities.get( usize::from( id.id ) ) {
			entity.get().is_some()
		} else {
			false
		}
	}
	
}

impl< T, V, E > EcsCompFn< T, E > for TestEcs< V, E > where
	V: CompVec + CompVecFn< T, E >,
	E: Entity + EntityFn< T >,
	EntityId< E >: Copy,
	CompId< T, E >: Copy,
	T: Clone,
{
	fn get ( &self, id: CompId< T, E > ) -> Option< &T > {
		self.components.get( id ).map( Component::borrow )
	}
	
	fn get_mut ( &mut self, id: CompId< T, E > ) -> Option< &mut T > {
		self.components.get_mut( id ).map( Component::borrow_mut )
	}
	
	fn call< F: Fn( &T ) -> U, U > ( & self, id: CompId<T, E>, fcn: F ) -> Option< U > {
		if let Some( comp ) = self.get( id ) {
			Some( fcn( comp ) )
		} else {
			None
		}
	}
	
	fn call_mut< F: FnMut( &mut T ) -> U, U > ( &mut self, id: CompId<T, E>, mut fcn: F ) -> Option< U > {
		if let Some( comp ) = self.get_mut( id ) {
			Some( fcn( comp ) )
		} else {
			None
		}
	}
	
	fn insert ( &mut self, id: EntityId< E >, item: T ) -> Option< CompId< T, E >> {
		if let Some( entity ) = self.entities.get_mut( usize::from( id ) ) {
			let comp = Component::new( id, item );
			
			let comp_id = self.components.insert( comp );
			entity.set( comp_id );
			
			Some( comp_id )
		} else {
			None
		}
	}
	
}

impl< V, E > EcsFn< V, E > for TestEcs< V, E > where
	V: CompVec,
	E: Entity,
{
	fn iter< T > ( &self ) -> CompIter< T, E >
	where
		V: CompVec + CompVecFn< T, E >,
		E: Clone,
	{
		self.components.iter()
	}
	
	fn iter_mut< T > ( &mut self ) -> CompIterMut< T, E >
	where
		V: CompVec + CompVecFn< T, E >,
		E: Clone,
	{
		self.components.iter_mut()
	}
	
	fn remove< T > ( &mut self, e_id: EntityId< E > ) -> Result< (), EcsErr >
	where
		V: CompVec + CompVecFn< T, E >,
		E: EntityFn< T > + Clone,
	{
		if let Some( entity ) = self.entities.get_mut( usize::from( e_id ) ) {
			let comp_id = entity.get();
			//let comp_id = entity.remove();
			if let Some( cid ) = comp_id {
				let out = self.components.remove( cid );
				
				if out.is_ok() {
					entity.remove();
				}
				
				out
			} else {
				Err( EcsErr::EntityMissingComponent( usize::from( e_id ) ))
			}
		} else {
			Err( EcsErr::NoSuchEntityId( usize::from( e_id ) ))
		}
	}
	
	fn run_system< F: FnMut( &mut V, &E )> ( &mut self, mut system_fn: F ) {
		for ent in self.entities.iter() {
			system_fn( &mut self.components, ent );
		}
	}
	
}

#[allow( dead_code )]
impl< V: CompVec, E: Entity > TestEcs< V, E > {
	pub fn new () -> Self {
		Self {
			entities: Vec::new(),
			components: V::new(),
		}
	}
}

#[allow( dead_code )]
#[derive(Clone)]
struct Voidunia {
	type_0: Option< CompId< usize, Self > >,
	type_1: Option< CompId< f64, Self > >,
}

impl Entity for Voidunia {
	fn new () -> Self {
		Self { type_0: None, type_1: None }
	}
}

#[allow( unused )]
impl<T> EntityFn< T > for Voidunia {
	fn get ( &self ) -> Option< CompId< T, Self > > {
		todo!()
	}
	fn remove ( &mut self ) -> Option< CompId< T, Self > > {
		todo!()
	}
	fn set ( &mut self, id: CompId< T, Self > ) -> Option< CompId< T, Self >> {
		todo!()
	}
	fn try_set ( &mut self, id: Option< CompId< T, Self >> ) -> Option< CompId< T, Self >> {
		todo!()
	}
}

#[allow( dead_code )]
#[derive(Clone)]
struct Testunia {
	type_0: Vec< Component< usize, Voidunia >>,
	type_1: Vec< Component< f64, Voidunia >>,
	type_0_recycle: Vec< usize >,
	type_1_recycle: Vec< usize >,
}

#[allow( unused )]
impl CompVec for Testunia {
	fn new () -> Self {
		todo!()
	}
	
	fn shrink ( &mut self ) {
		todo!()
	}
}

#[allow( unused )]
impl<T> CompVecFn< T, Voidunia > for Testunia where
	T: Clone,
	EntityId< Voidunia >: Clone,
{
	fn get ( &self, id: CompId< T, Voidunia > ) -> Option< &Component< T, Voidunia > > {
		todo!()
	}
	
	fn get_mut ( &mut self, id: CompId< T, Voidunia > ) -> Option< &mut Component< T, Voidunia > > {
		todo!()
	}
	
	fn insert ( &mut self, item: Component< T, Voidunia > ) -> CompId< T, Voidunia > {
		//if let Some( idx ) = self.fld_recycle.pop() {
		//	if let Some( comp ) = self.fld.get_mut( idx ) {
		//		comp.overwrite( item );
		//		
		//		return idx.into()
		//	}
		//	
		//	self.fld_recycle.push( idx );
		//}
		//
		//let idx = self.fld.len();
		//self.fld.push( item );
		//idx.into()
		todo!()
	}
	
	fn remove ( &mut self, id: CompId< T, Voidunia > ) -> Result<(), EcsErr > {
		//self.fld.contains()
		todo!()
	}
	
	fn len ( &self, _: std::marker::PhantomData< T > ) -> usize {
		todo!()
	}
	
	fn iter ( &self ) -> CompIter< T, Voidunia > {
		todo!()
	}
	
	fn iter_mut ( &mut self ) -> CompIterMut< T, Voidunia > {
		todo!()
	}
	
}
// */
