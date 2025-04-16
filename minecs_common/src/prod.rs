//------------------------------------------------------------------------------
// --Traits

// array of Vec< Component< T >>
pub trait CompVec {
	/// Creates a new empty `CompVec`
	fn new () -> Self;
	
	/// Attempts to reduce memory usage by calling `Vec::shrink_to_fit` on every component vector.
	fn shrink ( &mut self );
}

// per T fn
pub trait CompVecT< T >
where
	Self: CompVec,
	T: Clone,
{	
	/// Returns amount of components (including retained for overwrite) of type `T`.
	fn len ( &self, _: std::marker::PhantomData< T > ) -> usize;
	
	
	/// Returns an iterator over components of type `T`.
	/// 
	/// The iterator yields all items from start to end.
	fn iter ( &self ) -> CompIter< T >;
	
	/// Returns an iterator over components of type `T`, that allows modifying each value.
	/// 
	/// The iterator yields all items from start to end.
	fn iter_mut ( &mut self ) -> CompIterMut< T >;
	
}

// per T fn
pub trait CompVecTE< T, E >
where
	Self: CompVec,
	T: Clone,
	EntityId< E >: Clone,
{
	/// Adds component `T` to an internal collection.
	/// 
	/// Returns id of the component.
	fn insert ( &mut self, item: Component< T > ) -> CompId< T, E >;
	
	/// Removes component `T` with specified id.
	/// 
	/// On success returns  `Ok(())`, or `EcsErr` otherwise.
	/// 
	/// # Errors
	/// 
	/// EcsErr::NoSuchCompId - when component with specified id does not exist
	fn remove ( &mut self, id: CompId< T, E > ) -> Result< (), EcsErr >;
	
	
	/// Attempts to borrow component with specified id.
	/// 
	/// On success returns `Some( &Component< T > )` or `None` otherwise.
	fn get ( &self, id: CompId< T, E > ) -> Option< &Component< T > >;
	
	/// Attempts to mutably borrow component with specified id.
	/// 
	/// On success returns `Some( &mut Component< T > )` or `None` otherwise.
	fn get_mut ( &mut self, id: CompId< T, E > ) -> Option< &mut Component< T > >;
	
}

// ECS -------------------------------------------------------------------------

pub trait Ecs< L > {
	/// Attempts to reduce memory usage by calling `Vec::shrink_to_fit` on every component vector.
	fn shrink ( &mut self );
	
	/// Creates and returns a new entity id.
	fn new_entity< E > ( &mut self ) -> EntityId< E > where
		E: Entity,
		L: EntityListE< E >
	;
}

// per E fn
pub trait EcsEL< E, L > where
	Self: Sized,
	E: Entity,
	L: EntityListE< E >,
{
	/// Attempts to borrow entity with specified id.
	/// 
	/// On success returns `Some( &Entity )` or `None` otherwise.
	fn entity ( &self, id: EntityId< E > ) -> Option< &E >;
	
	/// Attempts to mutably borrow entity with specified id.
	/// 
	/// On success returns `Some( &mut Entity )` or `None` otherwise.
	fn entity_mut ( &mut self, id: EntityId< E > ) -> Option< &mut E >;
	
	
	/// Returns true if component with given id exists, or false otherwise.
	fn has_component< T > ( &self, id: EntityId< E > ) -> bool where
		E: EntityFn<T>
	;
	//fn insert_raw_entity< T: RawEntity< E > > ( &mut self, raw_entity: T ) -> EntityId< E >;
}

// per T fn
pub trait EcsTE< T, E > {
	/// Attempts to add component to the entity, potentially discarding the previous component.
	/// 
	/// On success returns `Some( CompId< T, E > )` or `None` otherwise.
	fn insert ( &mut self, id: EntityId< E >, item: T ) -> Option< CompId< T, E >>;
	
	
	/// Attempts to borrow component with specified id.
	/// 
	/// On success returns `Some( &T )` or `None` otherwise.
	fn get ( &self, id: CompId< T, E > ) -> Option< &T >;
	
	/// Attempts to mutably borrow component with specified id.
	/// 
	/// On success returns `Some( &mut T )` or `None` otherwise.
	fn get_mut ( &mut self, id: CompId< T, E > ) -> Option< &mut T >;
	
	
	/// Attempts to call a function `Fn( &T ) -> U` on component with specified id.
	/// 
	/// On success returns `Some( U )` or `None` otherwise.
	fn call< F, U > ( &self, id: CompId< T, E >, fcn: F ) -> Option< U > where
		F: Fn( &T ) -> U
	;
	
	/// Attempts to call a function `FnMut( &mut T ) -> U` on component with specified id.
	/// 
	/// On success returns `Some( U )` or `None` otherwise.
	fn call_mut< F, U > ( &mut self, id: CompId<T, E>, fcn: F ) -> Option< U > where
		F: FnMut( &mut T ) -> U
	;
	
}

pub trait EcsV< V > {
	/// Returns an iterator over components of type `T`.
	/// 
	/// The iterator yields all items from start to end.
	fn iter< T > ( &self ) -> CompIter< T >
	where
		V: CompVecT< T >,
		T: Clone,
	;
	
	/// Returns an iterator over components of type `T`, that allows modifying each value.
	/// 
	/// The iterator yields all items from start to end.
	fn iter_mut< T > ( &mut self ) -> CompIterMut< T >
	where
		V: CompVecT< T >,
		T: Clone,
	;
	
}

pub trait EcsVE< V, E > {
	/// Attempts to remove specified component.
	/// 
	/// On success returns `Ok(())` or `EcsErr` otherwise.
	/// 
	/// # Errors
	/// 
	/// EcsErr::CompVecMissingComponent - when component is present in entity, but cannot be found in `CompVec`
	/// EcsErr::EntityMissingComponent - when component is not present in entity
	/// EcsErr::NoSuchEntityId - when entity with specified id does not exist
	/// EcsErr::NoSuchCompId - when component with specified id does not exist
	fn remove< T > ( &mut self, e_id: EntityId< E > ) -> Result< (), EcsErr > where
		V: CompVecTE< T, E >,
		E: EntityFn< T > + Clone,
		T: Clone,
	;
	
	
	/// Attempts to add component to the entity, using specified method of `E: Entity`, potentially discarding the previous component.
	/// Primary use case is to insert a named field.
	/// 
	/// On success returns `Some( CompId< T, E > )` or `None` otherwise.
	fn insert_fn< T, F > ( &mut self, fcn: F, id: EntityId< E >, item: T ) -> Option< CompId< T, E >> where
		V: CompVecTE< T, E >,
		E: Clone,
		T: Clone,
		F: FnMut( &mut E, CompId< T, E > ) -> Option< CompId< T, E >>
	;
	
	/// Attempts to remove specified component, using specified method of `E: Entity`.
	/// Primary use case is to remove a named field.
	/// 
	/// On success returns `Ok(())` or `EcsErr` otherwise.
	/// 
	/// # Errors
	/// 
	/// EcsErr::CompVecMissingComponent - when component is present in entity, but cannot be found in `CompVec`
	/// EcsErr::EntityMissingComponent - when component is not present in entity
	/// EcsErr::NoSuchEntityId - when entity with specified id does not exist
	/// EcsErr::NoSuchCompId - when component with specified id does not exist
	fn remove_fn< T, F > ( &mut self, fcn: F, e_id: EntityId< E > ) -> Result< (), EcsErr > where
		V: CompVecTE< T, E >,
		E: Clone,
		T: Clone,
		F: FnMut( &mut E ) -> Option< CompId< T, E >>
	;
	
	
	/// Calls a provided function or closure for each entity in world.
	/// 
	/// FnMut gets access to `&mut V: CompVec` and `&E: Entity` during each iteration.
	/// 
	/// ```ignore
	/// use min_ecs::*;
	/// minecs!( world MinEcs; comp_vec Cv; entity TestEntity ( f64, usize, ) { some_fld: usize, } );
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
	/// 		let opt: Option< &Component< usize > > = comp_vec.get( id );
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
	fn run_system< F > ( &mut self, system_fn: F ) where
		F: FnMut( &mut V, &E ),
	;
	
	/// Calls a provided function or closure for each entity in world and allows modyfying it.
	/// 
	/// FnMut gets access to `&mut V: CompVec` and `&mut E: Entity` during each iteration.
	fn run_system_mut< F > ( &mut self, system_fn: F ) where
		F: FnMut( &mut V, &mut E ),
	;
	
}

// EntityListE ------------------------------------------------------------------

pub trait EntityListE< E > where
	E: Entity
{
	/// Creates a new `Entity` and returns its id.
	fn new_entity ( &mut self ) -> EntityId< E >;
	
	
	/// Attempts to borrow entity with specified id.
	/// 
	/// On success returns `Some( &E )` or `None` otherwise.
	fn get ( &self, id: EntityId< E > ) -> Option< &E >;
	
	/// Attempts to mutably borrow entity with specified id.
	/// 
	/// On success returns `Some( &mut E )` or `None` otherwise.
	fn get_mut ( &mut self, id: EntityId< E > ) -> Option< &mut E >;
	
	
	/// Returns an iterator over entities of type `E`.
	/// 
	/// The iterator yields all items from start to end.
	fn iter ( &self ) -> std::slice::Iter< E >;
	
	/// Returns an iterator over entities of type `E`, that allows modifying each elemennt.
	/// 
	/// The iterator yields all items from start to end.
	fn iter_mut ( &mut self ) -> std::slice::IterMut< E >;
}

pub trait EntityListTE< T, E > where
	E: Entity + EntityFn< T >
{
	/// Returns id of component stored by specified entity.
	/// 
	/// If entity has component id returns `Some( CompId< T, E > )` or `None` otherwise.
	fn comp_id ( &self, id: EntityId< E > ) -> Option< CompId< T, E > >;
	
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
	/// 
	/// Returns the old id in `Some( CompId< T, Self > )` if present or `None` otherwise.
	fn set ( &mut self, id: CompId< T, Self > ) -> Option< CompId< T, Self >>;
	
	/// Associates the specified component id if it is `Option::Some` with this entity.
	/// 
	/// Returns the old id in `Some( CompId< T, Self > )` if present or `None` otherwise.
	fn try_set ( &mut self, id: Option< CompId< T, Self >> ) -> Option< CompId< T, Self >>;
	
	/// Returns the component id in `Some( CompId< T, Self > )` if present or `None` otherwise.
	fn get ( &self ) -> Option< CompId< T, Self > >;
	
	/// Sets the component id to `None`.
	/// 
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
pub struct Component< T: Clone > {
	inner: T,
	entity_id: usize,
}

impl< T: Clone > Component< T > {
	/// Creates a new component.
	/// 
	/// # Correctness
	/// 
	/// The association between component and entity is incomplete at this point.
	/// 
	/// The process of adding a `Component` to the `Entity` is as follows:
	/// 
	/// 1. new `Entity` registered in ECS -> `EntityId`,
	/// 2. new `Component` created + `EntityId` stored in `Component`, <- this step
	/// 3. `Component` registered in ECS -> `CompId`,
	/// 4. `CompId` stored in `Entity`.
	#[inline]
	pub const fn new< E > ( id: EntityId< E >, value: T ) -> Self where
		EntityId< E >: Copy,
		E: Entity,
	{
		Self {
			inner: value,
			entity_id: id.id(),
		}
	}
	
	/// Used to overwrite `self` in place, instead of first deallocating old value and then allocating a new one.
	/// 
	/// # Correctness
	/// 
	/// The association between component and entity is incomplete at this point.
	/// 
	/// Please refer to `Component::new` for details.
	#[inline]
	pub fn overwrite ( &mut self, item: Self ) {
		self.inner = item.inner;
		self.entity_id = item.entity_id;
	}
	
	/// Returns id of the entity this component is associated to.
	#[inline]
	pub const fn id< E > ( &self ) -> EntityId< E > where
		EntityId< E >: Copy,
		E: Entity,
	{
		EntityId::new( self.entity_id )
	}
	
	/// Returns the contained component, consuming the `self` value.
	#[inline]
	pub fn unwrap ( self ) -> T {
		self.inner
	}
	
	#[inline]
	pub const fn inner( &self ) -> &T {
		&self.inner
	}
	
	#[inline]
	pub fn inner_mut( &mut self ) -> &mut T {
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
	/// 2. new `Component` created + `EntityId` stored in `Component`,
	/// 3. `Component` registered in ECS -> `CompId`,
	/// 4. `CompId` stored in `Entity`.
	#[inline]
	pub const fn new ( value: usize ) -> Self {
		Self {
			id: value,
			marker: std::marker::PhantomData::< E >,
		}
	}
	
	#[inline]
	pub const fn id ( &self ) -> usize {
		self.id
	}
}

impl< E > From< EntityId< E > > for usize {
	#[inline]
	fn from( value: EntityId< E > ) -> Self {
		value.id
	}
}
impl< E > From< usize > for EntityId< E > {
	#[inline]
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
	comp_id: usize,
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
	#[inline]
	pub const fn new ( id: usize ) -> Self {
		Self {
			comp_id: id,
			comp_marker: std::marker::PhantomData::< T >,
			entity_marker: std::marker::PhantomData::< E >,
		}
	}
}

impl< T, E > From< CompId< T, E > > for usize {
	#[inline]
	fn from( value: CompId< T, E > ) -> Self {
		value.comp_id
	}
}
impl< T, E > From< usize > for CompId< T, E > {
	#[inline]
	fn from( value: usize ) -> Self {
		Self::new( value )
	}
}

impl< T: Clone, E: Clone > Copy for CompId< T, E > {}

// struct - CompId
//------------------------------------------------------------------------------
// struct - CompIter

#[derive( Debug )]
pub struct CompIter< 'a, T: Clone > {
	data: core::slice::Iter< 'a, Component< T > >,
}

impl< 'a, T: Clone > From< core::slice::Iter< 'a, Component< T > > > for CompIter< 'a, T > {
	#[inline]
	fn from( value: core::slice::Iter< 'a, Component< T > > ) -> Self {
		Self {
			data: value,
		}
	}
}

impl< 'a, T: Clone > Iterator for CompIter< 'a, T > {
	type Item = &'a T;
	
	#[inline]
	fn next( &mut self ) -> Option< Self::Item > {
		self.data.next().map( Component::inner )
	}
}

// struct - CompIter
//------------------------------------------------------------------------------
// struct - CompIterMut

#[derive( Debug )]
pub struct CompIterMut< 'a, T: Clone > {
	data: core::slice::IterMut< 'a, Component< T > >,
}

impl< 'a, T: Clone > From< core::slice::IterMut< 'a, Component< T > > > for CompIterMut< 'a, T > {
	#[inline]
	fn from( value: core::slice::IterMut< 'a, Component< T > > ) -> Self {
		Self {
			data: value,
		}
	}
}

impl< 'a, T: Clone > Iterator for CompIterMut< 'a, T > {
	type Item = &'a mut T;
	
	#[inline]
	fn next( &mut self ) -> Option< Self::Item > {
		self.data.next().map( Component::inner_mut )
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

#[allow( clippy::min_ident_chars )]
impl std::fmt::Display for EcsErr {
	#[inline]
	fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result {
		match *self {
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
