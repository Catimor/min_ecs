use std::fmt::Write;

use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::Span;
use syn::{ Token, Type, parse::{ Parse, ParseStream } };
use quote::{ ToTokens, quote };

pub struct CompArray {
	min_ecs_name: syn::Ident,
	ca_name: syn::Ident,
	entity_name: syn::Ident,
	all_types: Vec< Type >,
	generic_types: Vec< Type >,
	named_comps: Vec< IdentTypePair >,
	derives: Vec< DeriveType >,
}

impl Parse for CompArray {
	fn parse( input: ParseStream ) -> syn::Result<Self> {
		let mut derives = DeriveType::new_vec();
		
		if input.parse::< Token![#] >().is_ok() {
			let inner;
			_ = syn::bracketed!( inner in input );
			
			inner.step( |cursor| {
				let rest = *cursor;
				let derive_ident = syn::Ident::new( "derive", Span::mixed_site() );
				
				if let Some(( tt, next )) = rest.token_tree() {
					match tt {
						proc_macro2::TokenTree::Ident( bracketed_ident ) => {
							if bracketed_ident == derive_ident {
								Ok(((), next))
							} else {
								Err( cursor.error( "invalid attribute: expected `derive`" ))
							}
						}
						_ => Err( cursor.error("unexpected token in input") )
					}
				} else {
					Err( cursor.error("unexpected token in input") )
				}
			})?;
			
			let very_inner;
			_ = syn::parenthesized!( very_inner in inner );
			let vec: Vec<DeriveType> = very_inner.parse_terminated( DeriveType::parse, syn::Token![,])?.into_iter().collect();
			derives.extend( vec );
		};
		_ = input.parse::< Token![,] >();// ignore trailing comma
		
		_ = kw::ecs::parse( input )?;
		let min_ecs_name = syn::Ident::parse( input )?;
		
		_ = input.parse::< Token![<] >()?;
		let ca_name = syn::Ident::parse( input )?;
		_ = input.parse::< Token![,] >()?;
		let entity_name = syn::Ident::parse( input )?;
		_ = input.parse::< Token![>] >()?;
		
		let fld_tokens;
		_ = syn::braced!( fld_tokens in input );
		
		let generic_types = if kw::types::parse( &fld_tokens ).is_ok() {
			let inner;
			_ = syn::bracketed!( inner in fld_tokens );
			inner.parse_terminated( syn::Type::parse, syn::Token![,])?.into_iter().collect()
		} else {
			Vec::new()
		};
		
		let named_comps: Vec<IdentTypePair> = fld_tokens.parse_terminated( IdentTypePair::parse, syn::Token![,])?.into_iter().collect();
		_ = fld_tokens.parse::< Token![,] >();// ignore trailing comma
		
		let mut all_types = generic_types.clone();
		if !named_comps.is_empty() {
			let bonus_types: Vec< syn::Type > = named_comps.iter().map( syn::Type::from ).collect();
			
			for bonus_type in &bonus_types {
				if !all_types.contains( bonus_type ) {
					all_types.push( bonus_type.clone() );
				}
			}
		}
		
		
		Ok( Self {
			min_ecs_name, 
			ca_name,
			entity_name,
			all_types,
			generic_types,
			named_comps,
			derives,
		})
	}
}

impl ToTokens for CompArray {
	fn to_tokens( &self, tokens: &mut TokenStream2 ) {
		let CompArray {
			min_ecs_name,
			ca_name,
			entity_name,
			derives,
			..
		} = self;
		
		let entity = MinEcsEntity::from( self );
		let ca = MinEcsCa::from( self );
		
		let macro_args: Vec<_> = self.named_comps.iter().map( MinEcsMacroArg::from ).collect();
		
		tokens.extend( quote! {
			#entity
			
			#ca
			
			new_ecs!( #min_ecs_name, #ca_name, #entity_name #( #macro_args )*; #( #derives , )* );
		});
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

impl From< &IdentTypePair > for syn::Ident {
	fn from( value: &IdentTypePair ) -> Self {
		value.ident.clone()
	}
}

impl From< &IdentTypePair > for syn::Type {
	fn from( value: &IdentTypePair ) -> Self {
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

pub struct MinEcsMacroArg {
	tokens: TokenStream2,
}

impl From< &IdentTypePair > for MinEcsMacroArg {
	fn from( value: &IdentTypePair ) -> Self {
		let ident = value.ident.clone();
		let ty = value.of_type.clone();
		
		let tokens = quote! {
			,#ident: #ty
		};
		
		Self {
			tokens,
		}
	}
}

impl ToTokens for MinEcsMacroArg {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		tokens.extend( self.tokens.clone() );
	}
}

//------------------------------------------------------------------------------

#[derive( Clone )]
enum DeriveType {
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

pub struct MinEcsEntity {
	derives: Vec< DeriveType >,
	entity_name: syn::Ident,
	
	named_pairs: Vec< IdentTypePair >,
	generic_pairs: Vec< IdentTypePair >,
}

#[allow( clippy::min_ident_chars )]
impl From< &CompArray > for MinEcsEntity {
	fn from( value: &CompArray ) -> Self {
		let CompArray {
			entity_name,
			generic_types,
			named_comps,
			derives,
			..
		} = &value;
		
		let mut generic_pairs = Vec::new();
		let mut named_pairs = Vec::new();
		
		let mut s_ident = String::new();
		let mut ident;
		for ( idx, t ) in generic_types.iter().enumerate() {
			_ = write!( &mut s_ident, "type_{idx}" );
			ident = syn::Ident::new( &s_ident, Span::mixed_site() );
			s_ident.clear();
			
			generic_pairs.push( IdentTypePair::new( ident.clone(), t.clone() ) );
		}
		
		for pair in named_comps {
			named_pairs.push( pair.clone() );
		}
		
		MinEcsEntity {
			derives: derives.clone(),
			entity_name: entity_name.clone(),
			
			generic_pairs,
			named_pairs,
		}
	}
}

#[allow( clippy::min_ident_chars )]
impl ToTokens for MinEcsEntity {
	fn to_tokens( &self, tokens: &mut TokenStream2 ) {
		let MinEcsEntity {
			derives,
			entity_name,
			
			named_pairs,
			generic_pairs,
		} = self;
		
		let mut all_pairs = generic_pairs.clone();
		all_pairs.extend( named_pairs.clone() );
		
		let entity_fields: Vec<_> = all_pairs.iter().map( EntityArg::from ).collect();
		let field_names: Vec<_> = all_pairs.iter().map( syn::Ident::from ).collect();
		
		tokens.extend( quote! {
			#[derive( #( #derives , )* )]
			pub struct #entity_name {
				#(
					#entity_fields
				)*
			}
			
			impl Entity for #entity_name {
				fn new() -> Self {
					Self {
						#(
							#field_names: None,
						)*
					}
				}
			}
			
			impl Default for #entity_name {
				fn default() -> Self { Self::new() }
			}
		});
		
		let generic_idents = generic_pairs.iter().map( syn::Ident::from ).collect::<Vec<_>>();
		let generic_types = generic_pairs.iter().map( syn::Type::from ).collect::<Vec<_>>();
		
		for ( ident, t ) in generic_idents.iter().zip( generic_types.iter() ) {
			tokens.extend( quote! {
				impl EntityFn< #t > for #entity_name {
					fn set ( &mut self, item: CompId< #t, Self > ) -> Option< CompId< #t, Self >> {
						self.#ident.replace( item )
					}
					
					fn try_set ( &mut self, item: Option< CompId< #t, Self >> ) -> Option< CompId< #t, Self >> {
						if let Some( inner ) = item {
							self.#ident.replace( inner )
						} else {
							None
						}
					}
					
					fn get ( &self ) -> Option< CompId< #t, Self > > {
						self.#ident.clone()
					}
					
					fn remove ( &mut self ) -> Option< CompId< #t, Self > > {
						self.#ident.take()
					}
				}
			});
		}
		
		let named_idents = named_pairs.iter().map( syn::Ident::from ).collect::<Vec<_>>();
		let named_types = named_pairs.iter().map( syn::Type::from ).collect::<Vec<_>>();
		
		let mut s_ident = String::new();
		let mut set_ident;
		let mut try_set_ident;
		let mut remove_ident;
		
		for ( ident, t ) in named_idents.iter().zip( named_types.iter() ) {
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
				impl #entity_name {
					pub fn #set_ident ( &mut self, item: CompId< #t, Self > ) -> Option< CompId< #t, Self >> {
						self.#ident.replace( item )
					}
					
					pub fn #try_set_ident ( &mut self, item: Option< CompId< #t, Self >> ) -> Option< CompId< #t, Self >> {
						if let Some( inner ) = item {
							self.#ident.replace( inner )
						} else {
							None
						}
					}
					
					pub fn #remove_ident ( &mut self ) -> Option< CompId< #t, Self >> {
						self.#ident.take()
					}
					
					pub fn #ident ( &self ) -> Option< CompId< #t, Self >> {
						self.#ident.clone()
					}
				}
			});
		}
		// */
		
	}
}

//------------------------------------------------------------------------------

pub struct MinEcsCa {
	derives: Vec< DeriveType >,
	ca_name: syn::Ident,
	entity_name: syn::Ident,
	
	component_names: Vec< syn::Ident >,
	recycle_names: Vec< syn::Ident >,
	fld_types: Vec< syn::Type >,
}

impl From< &CompArray > for MinEcsCa {
	fn from( value: &CompArray ) -> Self {
		let CompArray {
			ca_name,
			entity_name,
			all_types,
			derives,
			..
		} = &value;
		
		let mut component_names = Vec::new();
		let mut recycle_names = Vec::new();
		
		let mut s_ident = String::new();
		let mut component_ident;
		let mut recycle_ident;
		for idx in 0 .. all_types.len() {
			_ = write!( &mut s_ident, "type_{idx}" );
			component_ident = syn::Ident::new( &s_ident, Span::mixed_site() );
			s_ident.clear();
			
			_ = write!( &mut s_ident, "recycle_{idx}" );
			recycle_ident = syn::Ident::new( &s_ident, Span::mixed_site() );
			s_ident.clear();
			
			component_names.push( component_ident );
			recycle_names.push( recycle_ident );
		}
		
		MinEcsCa {
			derives: derives.clone(),
			ca_name: ca_name.clone(),
			entity_name: entity_name.clone(),
			
			component_names,
			recycle_names,
			fld_types: all_types.clone(),
		}
	}
}

#[allow( clippy::min_ident_chars )]
impl ToTokens for MinEcsCa {
	fn to_tokens( &self, tokens: &mut TokenStream2 ) {
		let MinEcsCa {
			derives,
			ca_name,
			entity_name,
			
			component_names,
			recycle_names,
			fld_types,
		} = self;
		
		tokens.extend( quote! {
			#[derive( #( #derives , )* )]
			pub struct #ca_name {
				#(
					#component_names: Vec< Component< #fld_types, #entity_name >>,
				)*
				#(
					#recycle_names: Vec< usize >,
				)*
			}
			
			impl CompVec for #ca_name {
				fn new () -> Self {
					Self {
						#(
							#component_names: Vec::new(),
						)*
						#(
							#recycle_names: Vec::new(),
						)*
					}
				}
				
				fn shrink ( &mut self ) {
					#(
						self.#component_names.shrink_to_fit();
					)*
					#(
						self.#recycle_names.shrink_to_fit();
					)*
				}
			}
			
			impl Default for #ca_name {
				fn default() -> Self { Self::new() }
			}
		});
		
		let tmp_iter = component_names.iter()
			.zip( recycle_names.iter() )
			.zip( fld_types.iter() );
		
		for ( (ident, ident_recycle), t ) in tmp_iter {
			tokens.extend( quote! {
				impl CompVecFn< #t, #entity_name > for #ca_name {
					fn insert ( &mut self, item: Component< #t, #entity_name > ) -> CompId< #t, #entity_name > {
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
					
					fn remove ( &mut self, id: CompId< #t, #entity_name > ) -> Result< (), EcsErr > {
						let idx = usize::from( id );
						if self.#ident.len() < idx {
							return Err( EcsErr::NoSuchCompId( idx ) )
						} else if !self.#ident_recycle.contains( &idx ) {
							self.#ident_recycle.push( idx );
						}
						Ok(())
					}
					
					fn get ( &self, id: CompId< #t, #entity_name > ) -> Option< &Component< #t, #entity_name > > {
						let idx = usize::from( id );
						if self.#ident_recycle.contains( &idx ) {
							None
						} else {
							self.#ident.get( idx )
						}
					}
					
					fn get_mut ( &mut self, id: CompId< #t, #entity_name > ) -> Option< &mut Component< #t, #entity_name > > {
						self.#ident.get_mut( usize::from( id ) )
					}
					
					fn len ( &self, _: std::marker::PhantomData< #t > ) -> usize {
						self.#ident.len()
					}
					
					fn iter ( &self ) -> CompIter< #t, #entity_name > {
						self.#ident.iter().into()
					}
					
					fn iter_mut ( &mut self ) -> CompIterMut< #t, #entity_name > {
						self.#ident.iter_mut().into()
					}
				}
			});
		}
	}
}

//------------------------------------------------------------------------------

mod kw {
	syn::custom_keyword!( ecs );
	syn::custom_keyword!( types );
}
