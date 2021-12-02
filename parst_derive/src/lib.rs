mod attributes;
mod generate;

use crate::{attributes::parse_outer_attributes, generate::generate_expression};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(
	Parsable,
	attributes(parst, assert_eq, assert_ne, with_context, with_field_context)
)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let derive_input = parse_macro_input!(input as DeriveInput);
	proc_macro::TokenStream::from(process_input(&derive_input))
}

fn process_input(input: &DeriveInput) -> TokenStream {
	let ident = &input.ident;
	let generics = &input.generics;

	let outer_attributes = parse_outer_attributes(&input.attrs);

	let expression = generate_expression(input);

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
