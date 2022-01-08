mod deparsable;
mod parsable;

use crate::{
	deparsable::generate::generate_expression_deparsable,
	parsable::{attributes::parse_outer_attributes, generate::generate_expression_parsable},
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

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

	let expression = generate_expression_parsable(input);

	match outer_attributes.context {
		None => {
			// Implement for all C
			quote! {
				impl<'a, C> ::parst::Parsable<'a, C> for #ident #generics
					where C: Copy,
				{
					fn read(__bytes: &'a [u8], __context: C) -> ::parst::PResult<'a, Self> {
						#![allow(non_snake_case)]
						#expression
					}
				}
			}
		}
		Some(t) => {
			// Implement for some specific C
			quote! {
				impl<'a> ::parst::Parsable<'a, #t> for #ident #generics {
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

	let expression = generate_expression_deparsable(input);
	quote! {
		impl ::parst::Deparsable for #ident #generics
		{
			fn write(&self, mut __w: impl ::std::io::Write) -> ::std::io::Result<()> {
				#![allow(non_snake_case)]
				#expression
			}
		}
	}
}
