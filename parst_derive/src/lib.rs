mod deparsable;
mod parsable;

use crate::{
	deparsable::generate::generate_expression_deparsable,
	parsable::{attributes::parse_outer_attributes, generate::generate_expression_parsable},
};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
	parse2, parse_macro_input, DeriveInput, GenericParam, Generics, WhereClause, WherePredicate,
};

#[proc_macro_derive(
	Parsable,
	attributes(parst, assert_eq, assert_ne, with_context, with_field_context)
)]
pub fn derive_parsable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let derive_input = parse_macro_input!(input as DeriveInput);
	proc_macro::TokenStream::from(process_input_parsable(&derive_input))
}

fn process_input_parsable(input: &DeriveInput) -> TokenStream {
	let ident = &input.ident;
	let generics = &input.generics;

	let outer_attributes = parse_outer_attributes(&input.attrs);

	let (impl_generics, type_generics, where_clause) =
		process_generics_parsable(generics, outer_attributes.context.is_none());

	let expression = generate_expression_parsable(input);

	match outer_attributes.context {
		None => {
			// Implement for all C
			quote! {
				impl #impl_generics ::parst::Parsable<'a, __CTX> for #ident #type_generics #where_clause {
					fn read(__bytes: &'a [u8], __context: __CTX) -> ::parst::PResult<'a, Self> {
						#![allow(non_snake_case)]
						#expression
					}
				}
			}
		}
		Some(t) => {
			// Implement for some specific C
			quote! {
				impl #impl_generics ::parst::Parsable<'a, #t> for #ident #type_generics #where_clause {
					fn read(__bytes: &'a [u8], __context: #t) -> ::parst::PResult<'a, Self> {
						#![allow(non_snake_case)]
						#expression
					}
				}
			}
		}
	}
}

#[proc_macro_derive(Deparsable)]
pub fn derive_deparsable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let derive_input = parse_macro_input!(input as DeriveInput);
	proc_macro::TokenStream::from(process_input_deparsable(&derive_input))
}

fn process_input_deparsable(input: &DeriveInput) -> TokenStream {
	let ident = &input.ident;
	let generics = &input.generics;
	let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

	let expression = generate_expression_deparsable(input);
	quote! {
		impl #impl_generics ::parst::Deparsable for #ident #type_generics #where_clause
		{
			fn write(&self, __w: &mut impl ::std::io::Write) -> ::std::io::Result<()> {
				#![allow(non_snake_case)]
				#expression
			}
		}
	}
}

fn process_generics_parsable(
	generics: &Generics,
	generic_context: bool,
) -> (TokenStream, TokenStream, WhereClause) {
	let a_param = parse2::<GenericParam>(quote! { 'a }).unwrap();

	let params = generics.params.iter().cloned().collect::<Vec<_>>();

	let mut where_clause = generics
		.where_clause
		.clone()
		.unwrap_or_else(|| parse2(quote! { where }).unwrap());

	if generic_context {
		let context_constraint = parse2::<WherePredicate>(quote! { __CTX: Clone }).unwrap();
		where_clause.predicates.push(context_constraint);
	}

	// is 'a lifetime already present in generics?
	let is_already_a = params.iter().any(|x| x == &a_param);

	let mut impl_params = Vec::new();
	if !is_already_a {
		impl_params.push(a_param.into_token_stream())
	}
	params.iter().for_each(|p| {
		impl_params.push(quote! { #p });
	});
	if generic_context {
		impl_params.push(quote! { __CTX });
	}

	let type_params = params
		.iter()
		.map(|p| match p {
			GenericParam::Type(t) => {
				let i = &t.ident;
				quote! { #i }
			}
			GenericParam::Lifetime(l) => {
				quote! { #l }
			}
			GenericParam::Const(c) => {
				let i = &c.ident;
				quote! { #i }
			}
		})
		.collect::<Vec<_>>();

	let impl_params = quote! { < #( #impl_params ),* > };
	let type_params = quote! { < #( #type_params ),* > };

	(impl_params, type_params, where_clause)
}
