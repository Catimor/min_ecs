use std::fmt::Write;

use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::Span;
use syn::{ Token, Type, parse::{ Parse, ParseStream } };
use quote::{ ToTokens, quote };

pub struct World {
	name: syn::Ident,
	comp_vec: CompVec,
	entity_list: EntityList,
	entity_vec: Vec< Entity >,
	derives: Vec< DeriveType >,
}

impl Parse for World {
	fn parse( input: ParseStream ) -> syn::Result<Self> {
		//--- optional Derives
		
		let mut derives = DeriveType::new_vec();
		
		if input.parse::< Token![#] >().is_ok() {
			let inner;
			_ = syn::bracketed!( inner in input );
			
			_ = kw::derive::parse( &inner )?;
			
			let very_inner;
			_ = syn::parenthesized!( very_inner in inner );
			let mut vec: Vec< DeriveType > = very_inner.parse_terminated( DeriveType::parse, syn::Token![,])?.into_iter().collect();
			derives.append( &mut vec );
		};
		
		// ignore trailing comma
		_ = input.parse::< Token![,] >();
		
		//--- World name
		
		_ = kw::world::parse( input )?;
		let world_name = syn::Ident::parse( input )?;
		_ = input.parse::< Token![;] >()?;
		
		//--- CompVec name
		
		let mut s_ident = world_name.to_string();
		_ = write!( &mut s_ident, "Components" );
		
		let comp_vec_name = syn::Ident::new( &s_ident, Span::mixed_site() );
		
		s_ident = world_name.to_string();
		_ = write!( &mut s_ident, "Entities" );
		let entity_vec_name = syn::Ident::new( &s_ident, Span::mixed_site() );
		
		//--- Entities
		
		let mut entity_name;
		let mut inner_named;
		let mut inner_generic;
		
		let mut entity_vec = Vec::new();
		let mut all_types = Vec::< Type >::new();
		
		let mut generic_comps = Vec::new();
		let mut named_comps = Vec::< IdentTypePair >::new();
		
		let mut has_paren;
		let mut has_brace;
		
		while kw::entity::parse( input ).is_ok() {
			named_comps.clear();
			generic_comps.clear();
			
			entity_name = syn::Ident::parse( input )?;
			
			if input.peek( syn::token::Paren ) {
				_ = syn::parenthesized!( inner_generic in input );
				generic_comps = inner_generic.parse_terminated( syn::Type::parse, syn::Token![,])?.into_iter().collect();
				
				has_paren = true;
			} else {
				has_paren = false;
			}
			
			if input.peek( syn::token::Brace ) {
				_ = syn::braced!( inner_named in input );
				named_comps = inner_named.parse_terminated( IdentTypePair::parse, syn::Token![,])?.into_iter().collect();
				
				has_brace = true;
			} else {
				has_brace = false;
			}
			
			if has_paren && !has_brace {
				_ = input.parse::< Token![;] >()?;
			}
			
			entity_vec.push( Entity::new( entity_name, generic_comps.clone(), named_comps.clone(), derives.clone() ) );
			
			//--- Type filtering
			
			if generic_comps.is_empty() && named_comps.is_empty() {
				return Err( syn::Error::new( Span::mixed_site(), "each entity must have at least one type" ) );
			}
			
			for ty in generic_comps.drain( .. ) {
				if !all_types.contains( &ty ) {
					all_types.push( ty );
				}
			}
			for ty in named_comps.drain( .. ).map( syn::Type::from ) {
				if !all_types.contains( &ty ) {
					all_types.push( ty );
				}
			}
		}
		
		if entity_vec.is_empty() {
			return Err( syn::Error::new( Span::mixed_site(), "at least one `entity` is required" ) );
		}
		
		//--- CompVec, return Ok( World )
		
		let comp_vec = CompVec::new( comp_vec_name, all_types, entity_vec.clone(), derives.clone() );
		let entity_list = EntityList::new( entity_vec_name, entity_vec.clone(), derives.clone() );
		
		Ok( Self {
			name: world_name,
			comp_vec,
			entity_list,
			entity_vec,
			derives,
		})
	}
}

impl ToTokens for World {
	fn to_tokens( &self, tokens: &mut TokenStream2 ) {
		let World {
			name,
			comp_vec,
			entity_list,
			entity_vec,
			derives,
		} = self;
		
		let comp_vec_name = &comp_vec.name;
		let entity_list_name = &entity_list.name;
		
		tokens.extend( quote! {
			#[derive( #( #derives , )* )]
			pub struct #name {
				entities: #entity_list_name,
				components: #comp_vec_name,
			}
			
			impl #name {
				#[inline]
				pub fn new () -> Self {
					Self {
						entities: #entity_list_name::new(),
						components: #comp_vec_name::new(),
					}
				}
			}
			
			impl Default for #name {
				#[inline]
				fn default () -> Self { Self::new() }
			}
			
			impl Ecs< #entity_list_name > for #name {
				#[inline]
				fn shrink ( &mut self ) {
					self.components.shrink();
				}
				
				#[inline]
				fn new_entity< E: Entity > ( &mut self ) -> EntityId< E > where
					E: Entity,
					#entity_list_name: EntityListE< E >,
				{
					self.entities.new_entity()
				}
			}
		});
		
		let mut entity;
		for ent in entity_vec {
			entity = ent.name();
			
			tokens.extend( quote! {
				impl EcsEL< #entity, #entity_list_name > for #name where
					EntityId< #entity >: Copy,
					Self: Sized,
					#entity_list_name: EntityListE< #entity >,
				{
					
					//#[inline]
					//fn new_entity ( &mut self ) -> EntityId< #entity > {
					//	let idx = self.entities.#idx.len();
					//	self.entities.#idx.push( #entity::new() );
					//	EntityId::from( idx )
					//}
					
					#[inline]
					fn entity ( &self, id: EntityId< #entity > ) -> Option< &#entity > {
						self.entities.get( id )
					}
					
					#[inline]
					fn entity_mut ( &mut self, id: EntityId< #entity > ) -> Option< &mut #entity > {
						self.entities.get_mut( id )
					}
					
					#[inline]
					fn has_component< T > ( &self, id: EntityId< #entity > ) -> bool where
						#entity: EntityFn< T >
					{
						if let Some( entity ) = self.entities.get( id ) {
							entity.get().is_some()
						} else {
							false
						}
					}
				}
				
				impl< T > EcsTE< T, #entity > for #name where
					#comp_vec_name: CompVec + CompVecTE< T, #entity >,
					#entity: Entity + EntityFn< T >,
					EntityId< #entity >: Copy,
					CompId< T, #entity >: Copy,
					T: Clone,
				{
					#[inline]
					fn get ( &self, id: CompId< T, #entity > ) -> Option< &T > {
						self.components.get( id ).map( Component::inner )
					}
					
					#[inline]
					fn get_mut ( &mut self, id: CompId< T, #entity > ) -> Option< &mut T > {
						self.components.get_mut( id ).map( Component::inner_mut )
					}
					
					#[inline]
					fn call< F: Fn( &T ) -> U, U > ( &self, id: CompId< T, #entity >, fcn: F ) -> Option< U > {
						if let Some( comp ) = self.get( id ) {
							Some( fcn( comp ) )
						} else {
							None
						}
					}
					
					#[inline]
					fn call_mut< F: FnMut( &mut T ) -> U, U > ( &mut self, id: CompId< T, #entity >, mut fcn: F ) -> Option< U > {
						if let Some( comp ) = self.get_mut( id ) {
							Some( fcn( comp ) )
						} else {
							None
						}
					}
					
					#[inline]
					fn insert ( &mut self, id: EntityId< #entity >, item: T ) -> Option< CompId< T, #entity >> {
						if let Some( entity ) = self.entities.get_mut( id ) {
							let comp = Component::new( id, item );
							
							let comp_id = self.components.insert( comp );
							entity.set( comp_id );
							
							Some( comp_id )
						} else {
							None
						}
					}
					
				}
				
				impl EcsVE< #comp_vec_name, #entity > for #name where
					#comp_vec_name: CompVec,
					#entity: Entity,
				{
					#[inline]
					fn remove<T> ( &mut self, e_id: EntityId< #entity > ) -> Result< (), EcsErr >
					where
						#comp_vec_name: CompVec + CompVecTE< T, #entity >,
						#entity: EntityFn< T > + Clone,
						T: Clone,
					{
						if let Some( entity ) = self.entities.get_mut( e_id ) {
							let comp_id = entity.get();
							//let comp_id = entity.remove();
							if let Some( cid ) = comp_id {
								let out = self.components.remove( cid );
								
								if out.is_ok() {
									entity.remove();
									Ok(())
								} else {
									Err( EcsErr::CompVecMissingComponent( usize::from( e_id ), usize::from( cid ) ))
								}
								
							} else {
								Err( EcsErr::EntityMissingComponent( usize::from( e_id ) ))
							}
						} else {
							Err( EcsErr::NoSuchEntityId( usize::from( e_id ) ))
						}
					}
					
					
					#[inline]
					fn insert_fn< T, F > ( &mut self, mut fcn: F, id: EntityId< #entity >, item: T ) -> Option< CompId< T, #entity >> where
						#comp_vec_name: CompVecTE< T, #entity >,
						#entity: Clone,
						T: Clone,
						F: FnMut( &mut #entity, CompId< T, #entity > ) -> Option< CompId< T, #entity >>
					{
						if let Some( mut entity ) = self.entities.get_mut( id ) {
							let comp = Component::new( id, item );
							
							let comp_id = self.components.insert( comp );
							fcn( &mut entity, comp_id );
							
							Some( comp_id )
						} else {
							None
						}
					}
					
					#[inline]
					fn remove_fn< T, F > ( &mut self, mut fcn: F, e_id: EntityId< #entity > ) -> Result< (), EcsErr > where
						#comp_vec_name: CompVecTE< T, #entity >,
						#entity: Clone,
						T: Clone,
						F: FnMut( &mut #entity ) -> Option< CompId< T, #entity >>
					{
						if let Some( mut entity ) = self.entities.get_mut( e_id ) {
							let opt = fcn( &mut entity );
							if let Some( cid ) = opt {
								let out = self.components.remove( cid );
								
								if out.is_ok() {
									Ok(())
								} else {
									Err( EcsErr::CompVecMissingComponent( usize::from( e_id ), usize::from( cid ) ))
								}
								
							} else {
								Err( EcsErr::EntityMissingComponent( usize::from( e_id ) ))
							}
						} else {
							Err( EcsErr::NoSuchEntityId( usize::from( e_id ) ))
						}
					}
					
					
					#[inline]
					fn run_system< F> ( &mut self, mut system_fn: F ) where
						F: FnMut( &mut #comp_vec_name, &#entity )
					{
						for ent in self.entities.iter() {
							system_fn( &mut self.components, ent );
						}
					}
					
					#[inline]
					fn run_system_mut< F> ( &mut self, mut system_fn: F ) where
						F: FnMut( &mut #comp_vec_name, &mut #entity )
					{
						for ent in self.entities.iter_mut() {
							system_fn( &mut self.components, ent );
						}
					}
					
				}
			});
		}
		
		tokens.extend( quote! {
			impl EcsV< #comp_vec_name > for #name {
				#[inline]
				fn iter< T > ( &self ) -> CompIter< T > where
					#comp_vec_name: CompVecT< T >,
					T: Clone,
				{
					self.components.iter()
				}
				
				#[inline]
				fn iter_mut< T > ( &mut self ) -> CompIterMut< T > where
					#comp_vec_name: CompVecT< T >,
					T: Clone,
				{
					self.components.iter_mut()
				}
			}
			
			#comp_vec
			#entity_list
		});
		
		tokens.extend( quote! {
			#(
				#entity_vec
			)*
		});
		
	}
}

//---

pub struct CompVec {
	name: syn::Ident,
	type_vec: Vec< Type >,
	entity_vec: Vec< Entity >,
	derives: Vec< DeriveType >,
}

impl CompVec {
	pub fn new( name: syn::Ident, type_vec: Vec< Type >, entity_vec: Vec< Entity >, derives: Vec< DeriveType >, ) -> Self {
		CompVec {
			name,
			type_vec,
			entity_vec,
			derives,
		}
	}
}

impl ToTokens for CompVec {
	fn to_tokens( &self, tokens: &mut TokenStream2 ) {
		let CompVec {
			name,
			type_vec,
			entity_vec,
			derives,
		} = self;
		
		let mut fld_idx = 0;
		let mut s_ident = String::new();
		
		let mut ty_iter;
		let mut cv_fields = Vec::new();
		
		let mut comp_names = Vec::new();
		let mut recycle_names = Vec::new();
		let mut ty_vec = Vec::new();
		
		let mut entity_type;
		let mut ident;
		let mut ident_recycle;
		for entity in entity_vec {
			entity_type = entity.name();
			
			ty_iter = entity.type_list();
			
			for ty in ty_iter {
				if let Some( idx ) = cv_fields.iter().position( |elem: &CompVecFld| elem == ty ) {
					cv_fields[ idx ].append_entity( entity_type.clone() );
				} else {
					_ = write!( &mut s_ident, "type_{fld_idx}" );
					ident = syn::Ident::new( &s_ident, Span::mixed_site() );
					s_ident.clear();
					
					_ = write!( &mut s_ident, "recycle_{fld_idx}" );
					ident_recycle = syn::Ident::new( &s_ident, Span::mixed_site() );
					s_ident.clear();
					
					comp_names.push( ident.clone() );
					recycle_names.push( ident_recycle.clone() );
					ty_vec.push( ty.clone() );
					
					cv_fields.push( CompVecFld::new( ty.clone(), ident, ident_recycle, vec![ entity_type.clone() ] ) );
					
					fld_idx += 1;
				}
			}
		}
		
		if cv_fields.len() != type_vec.len() {
			panic!( "internal macro error: cv_fields.len() != type_vec.len()" )
		}
		
		tokens.extend( quote! {
			#[derive( #( #derives , )* )]
			pub struct #name {
				#(
					#comp_names: Vec< Component< #ty_vec >>,
				)*
				#(
					#recycle_names: Vec< usize >,
				)*
			}
			
			impl CompVec for #name {
				#[inline]
				fn new () -> Self {
					Self {
						#(
							#comp_names: Vec::new(),
						)*
						#(
							#recycle_names: Vec::new(),
						)*
					}
				}
				
				#[inline]
				fn shrink ( &mut self ) {
					#(
						self.#comp_names.shrink_to_fit();
					)*
					#(
						self.#recycle_names.shrink_to_fit();
					)*
				}
			}
			
			impl Default for #name {
				#[inline]
				fn default() -> Self { Self::new() }
			}
			
		});
		
		for fld in cv_fields {
			let CompVecFld {
				ty,
				ident,
				ident_recycle,
				entity_vec,
			} = fld;
			
			for entity_name in &entity_vec {
				tokens.extend( quote! {
					impl CompVecTE< #ty, #entity_name > for #name {
						#[inline]
						fn insert ( &mut self, item: Component< #ty > ) -> CompId< #ty, #entity_name > {
							if let Some( idx ) = self.#ident_recycle.pop() {
								if let Some( comp ) = self.#ident.get_mut( idx ) {
									comp.overwrite( item );
									
									return idx.into()
								}
								
								self.#ident_recycle.push( idx );
							}
							
							let idx = self.#ident.len();
							self.#ident.push( item );
							idx.into()
						}
						
						#[inline]
						fn remove ( &mut self, id: CompId< #ty, #entity_name > ) -> Result< (), EcsErr > {
							let idx = usize::from( id );
							if self.#ident.len() < idx {
								return Err( EcsErr::NoSuchCompId( idx ) )
							} else if !self.#ident_recycle.contains( &idx ) {
								self.#ident_recycle.push( idx );
							}
							Ok(())
						}
						
						#[inline]
						fn get ( &self, id: CompId< #ty, #entity_name > ) -> Option< &Component< #ty > > {
							let idx = usize::from( id );
							if self.#ident_recycle.contains( &idx ) {
								None
							} else {
								self.#ident.get( idx )
							}
						}
						
						#[inline]
						fn get_mut ( &mut self, id: CompId< #ty, #entity_name > ) -> Option< &mut Component< #ty > > {
							let idx = usize::from( id );
							if self.#ident_recycle.contains( &idx ) {
								None
							} else {
								self.#ident.get_mut( idx )
							}
						}
						
					}
					
				});
			}
			
			tokens.extend( quote! {
				impl CompVecT< #ty > for #name {
					#[inline]
					fn len ( &self, _: std::marker::PhantomData< #ty > ) -> usize {
						self.#ident.len()
					}
					
					#[inline]
					fn iter ( &self ) -> CompIter< #ty > {
						CompIter::< #ty >::from( self.#ident.iter() )
					}
					
					#[inline]
					fn iter_mut ( &mut self ) -> CompIterMut< #ty > {
						//self.#ident.iter_mut().into()
						CompIterMut::< #ty >::from( self.#ident.iter_mut() )
					}
					
				}
				
			});
		}
	}
}

//---

#[derive( Clone )]
pub struct Entity {
	name: syn::Ident,
	generic_comps: Vec< Type >,
	named_comps: Vec< IdentTypePair >,
	type_list: Vec< Type >,
	derives: Vec< DeriveType >,
}

impl Entity {
	pub fn new( name: syn::Ident, generic_comps: Vec< Type >, named_comps: Vec< IdentTypePair >, derives: Vec< DeriveType >, ) -> Self {
		let mut type_list = generic_comps.clone();
		for ty in named_comps.iter().map( syn::Type::from ) {
			if !type_list.contains( &ty ) {
				type_list.push( ty );
			}
		}
		
		Entity {
			name,
			generic_comps,
			named_comps,
			type_list,
			derives,
		}
	}
	
	pub const fn name ( &self ) -> &syn::Ident {
		&self.name
	}
	
	pub const fn type_list( &self ) -> &Vec< syn::Type > {
		&self.type_list
	}
	
}

impl ToTokens for Entity {
	fn to_tokens( &self, tokens: &mut TokenStream2 ) {
		let &Entity {
			name,
			generic_comps,
			named_comps,
			derives,
			..
		} = &self;
		
		let mut s_ident = String::new();
		let mut generic_fld_idents = Vec::new();
		let mut generic_fld_types = Vec::new();
		let mut named_fld_idents = Vec::new();
		let mut named_fld_types = Vec::new();
		
		for ( idx, ty ) in generic_comps.iter().enumerate() {
			_ = write!( &mut s_ident, "type_{idx}" );
			
			generic_fld_types.push( ty );
			generic_fld_idents.push( syn::Ident::new( &s_ident, Span::mixed_site() ));
			s_ident.clear();
		}
		
		for fld in named_comps {
			named_fld_idents.push( syn::Ident::from( fld ) );
			named_fld_types.push( syn::Type::from( fld ) );
		}
		
		tokens.extend( quote! {
			#[derive( #( #derives , )* )]
			pub struct #name {
				#(
					#generic_fld_idents: Option< CompId< #generic_fld_types, Self > >,
				)*
				#(
					#named_fld_idents: Option< CompId< #named_fld_types, Self > >,
				)*
			}
			
			impl Entity for #name {
				#[inline]
				fn new() -> Self {
					Self {
						#(
							#generic_fld_idents: None,
						)*
						#(
							#named_fld_idents: None,
						)*
					}
				}
			}
			
			impl Default for #name {
				#[inline]
				fn default() -> Self { Self::new() }
			}
		});
		
		for ( ident, t ) in generic_fld_idents.iter().zip( generic_fld_types ) {
			tokens.extend( quote! {
				impl EntityFn< #t > for #name {
					#[inline]
					fn set ( &mut self, item: CompId< #t, Self > ) -> Option< CompId< #t, Self >> {
						self.#ident.replace( item )
					}
					
					#[inline]
					fn try_set ( &mut self, item: Option< CompId< #t, Self >> ) -> Option< CompId< #t, Self >> {
						if let Some( inner ) = item {
							self.#ident.replace( inner )
						} else {
							None
						}
					}
					
					#[inline]
					fn get ( &self ) -> Option< CompId< #t, Self > > {
						self.#ident.clone()
					}
					
					#[inline]
					fn remove ( &mut self ) -> Option< CompId< #t, Self > > {
						self.#ident.take()
					}
				}
			});
		}
		
		let mut set_ident;
		let mut try_set_ident;
		let mut remove_ident;
		
		let mut ident;
		let mut t;
		for idx in 0 .. named_fld_idents.len() {
			ident = &named_fld_idents[ idx ];
			t = &named_fld_types[ idx ];
			
			_ = write!( &mut s_ident, "set_{ident}" );
			set_ident = syn::Ident::new( &s_ident, Span::mixed_site() );
			s_ident.clear();
			
			_ = write!( &mut s_ident, "try_set_{ident}" );
			try_set_ident = syn::Ident::new( &s_ident, Span::mixed_site() );
			s_ident.clear();
			
			_ = write!( &mut s_ident, "remove_{ident}" );
			remove_ident = syn::Ident::new( &s_ident, Span::mixed_site() );
			s_ident.clear();
			
			tokens.extend( quote! {
				impl #name {
					#[inline]
					pub fn #set_ident ( &mut self, item: CompId< #t, Self > ) -> Option< CompId< #t, Self >> {
						self.#ident.replace( item )
					}
					
					#[inline]
					pub fn #try_set_ident ( &mut self, item: Option< CompId< #t, Self >> ) -> Option< CompId< #t, Self >> {
						if let Some( inner ) = item {
							self.#ident.replace( inner )
						} else {
							None
						}
					}
					
					#[inline]
					pub fn #remove_ident ( &mut self ) -> Option< CompId< #t, Self >> {
						self.#ident.take()
					}
					
					#[inline]
					pub fn #ident ( &self ) -> Option< CompId< #t, Self >> {
						self.#ident.clone()
					}
				}
			});
		}
	}
}

//---

pub struct EntityList {
	name: syn::Ident,
	entity_vec: Vec< Entity >,
	derives: Vec< DeriveType >,
}

impl EntityList {
	pub fn new ( name: syn::Ident, entity_vec: Vec< Entity >, derives: Vec< DeriveType >, ) -> Self {
		EntityList {
			name,
			entity_vec,
			derives,
		}
	}
}

impl ToTokens for EntityList {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		let EntityList {
			name,
			entity_vec,
			derives,
		} = &self;
		
		let mut s_ident = String::with_capacity( 4 );
		let mut ident;
		
		let mut ident_vec = Vec::new();
		let mut type_vec = Vec::new();
		
		let len = entity_vec.len();
		
		for ( idx , ent ) in entity_vec.iter().enumerate() {
			s_ident.clear();
			_ = write!( &mut s_ident, "e{idx}" );
			
			ident = syn::Ident::new( &s_ident, Span::mixed_site() );
			ident_vec.push( ident );
			type_vec.push( ent.name() );
		}
		
		tokens.extend( quote!{
			#[derive( #( #derives , )* )]
			pub struct #name {
				#(
					#ident_vec: Vec< #type_vec >,
				)*
			}
			
			impl #name {
				#[inline]
				pub fn new () -> Self {
					Self {
						#(
							#ident_vec: Vec::new(),
						)*
					}
				}
			}
			
			impl Default for #name {
				#[inline]
				fn default () -> Self { Self::new() }
			}
		});
		
		let mut e_fld;
		let mut e_name;
		for idx in 0 .. len {
			e_fld = &ident_vec[ idx ];
			e_name = &type_vec[ idx ];
			
			tokens.extend( quote!{
				impl EntityListE< #e_name > for #name where
					#e_name: Entity
				{
					#[inline]
					fn new_entity ( &mut self ) -> EntityId< #e_name > {
						self.#e_fld.push( #e_name::new() );
						EntityId::from( self.#e_fld.len() - 1 )
					}
					
					
					#[inline]
					fn get ( &self, id: EntityId< #e_name > ) -> Option< &#e_name > {
						self.#e_fld.get( usize::from( id ))
					}
					
					#[inline]
					fn get_mut ( &mut self, id: EntityId< #e_name > ) -> Option< &mut #e_name > {
						self.#e_fld.get_mut( usize::from( id ))
					}
					
					
					#[inline]
					fn iter ( &self ) -> std::slice::Iter< #e_name > {
						self.#e_fld.iter()
					}
					
					#[inline]
					fn iter_mut ( &mut self ) -> std::slice::IterMut< #e_name > {
						self.#e_fld.iter_mut()
					}
				}
				
				impl< T > EntityListTE< T, #e_name > for #name where
					#e_name: Entity + EntityFn< T >
				{
					#[inline]
					fn comp_id ( &self, id: EntityId< #e_name > ) -> Option< CompId< T, #e_name > > {
						if let Some( entity ) = self.#e_fld.get( usize::from( id ) ) {
							entity.get()
						} else {
							None
						}
					}
				}
			});
		}
		
	}
}

//------------------------------------------------------------------------------

#[ derive( Clone ) ]
pub struct IdentTypePair {
	ident: syn::Ident,
	of_type: syn::Type,
}

impl IdentTypePair {
	pub const fn new( ident: syn::Ident, of_type: syn::Type, ) -> Self {
		IdentTypePair {
			ident,
			of_type,
		}
	}
	
	pub fn set_name ( &mut self, ident: syn::Ident ) {
		self.ident = ident;
	}
	
}

impl Parse for IdentTypePair {
	fn parse( input: ParseStream ) -> syn::Result<Self> {
		let ident = syn::Ident::parse( input )?;
		_ = input.parse::< Token![:] >()?;
		let of_type = syn::Type::parse( input )?;
		
		Ok(
			Self {
				ident,
				of_type,
			}
		)
	}
}

impl From< IdentTypePair > for syn::Ident {
	fn from( value: IdentTypePair ) -> Self {
		value.ident
	}
}
impl From< &IdentTypePair > for syn::Ident {
	fn from( value: &IdentTypePair ) -> Self {
		value.ident.clone()
	}
}
impl From< &mut IdentTypePair > for syn::Ident {
	fn from( value: &mut IdentTypePair ) -> Self {
		value.ident.clone()
	}
}

impl From< IdentTypePair > for syn::Type {
	fn from( value: IdentTypePair ) -> Self {
		value.of_type
	}
}
impl From< &IdentTypePair > for syn::Type {
	fn from( value: &IdentTypePair ) -> Self {
		value.of_type.clone()
	}
}
impl From< &mut IdentTypePair > for syn::Type {
	fn from( value: &mut IdentTypePair ) -> Self {
		value.of_type.clone()
	}
}

//------------------------------------------------------------------------------

pub struct EntityArg {
	tokens: TokenStream2,
}

impl From< &IdentTypePair > for EntityArg {
	fn from( value: &IdentTypePair ) -> Self {
		let ident = value.ident.clone();
		let ty = value.of_type.clone();
		
		let tokens = quote! {
			#ident: Option< CompId< #ty, Self >>,
		};
		
		Self {
			tokens,
		}
	}
}

impl ToTokens for EntityArg {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		tokens.extend( self.tokens.clone() );
	}
}

//------------------------------------------------------------------------------

#[derive( Clone )]
pub enum DeriveType {
	Ident ( syn::Ident ),
	Path ( syn::Path ),
}

impl DeriveType {
	pub fn new_vec () -> Vec< DeriveType > {
		let debug = Self::Ident( syn::Ident::new( "Debug", Span::mixed_site()));
		let clone = Self::Ident( syn::Ident::new( "Clone", Span::mixed_site()));
		let pareq = Self::Ident( syn::Ident::new( "PartialEq", Span::mixed_site()));
		
		vec![ debug, clone, pareq ]
	}
}

impl Parse for DeriveType {
	fn parse( input: ParseStream ) -> syn::Result<Self> {
		if let Ok( path ) = syn::Path::parse_mod_style( input ) {
			Ok( Self::Path( path ) )
		} else {
			let ident = input.parse()?;
			Ok( Self::Ident( ident ) )
		}
	}
}

impl ToTokens for DeriveType {
	fn to_tokens( &self, tokens: &mut proc_macro2::TokenStream ) {
		tokens.extend( match self {
			Self::Ident( val ) => quote! { #val },
			Self::Path( val ) => quote! { #val },
		});
	}
}

//------------------------------------------------------------------------------

pub struct CompVecFld {
	ty: syn::Type,
	ident: syn::Ident,
	ident_recycle: syn::Ident,
	entity_vec: Vec< syn::Ident >,
}

impl CompVecFld {
	pub fn new ( ty: syn::Type, ident: syn::Ident, ident_recycle: syn::Ident, entity_vec: Vec< syn::Ident >,  ) -> Self {
		CompVecFld {
			ty,
			ident,
			ident_recycle,
			entity_vec,
		}
	}
	
	pub fn append_entity ( &mut self, entity: syn::Ident ) {
		self.entity_vec.push( entity );
	}
}

impl PartialEq< syn::Type > for CompVecFld {
	fn eq( &self, other: &syn::Type ) -> bool {
		&self.ty == other
	}
}

//------------------------------------------------------------------------------

mod kw {
	syn::custom_keyword!( derive );
	syn::custom_keyword!( world );
	syn::custom_keyword!( entity );
}
