mod deparsable;
mod helpers;
mod parsable;

use crate::{
	deparsable::generate::generate_expression_deparsable,
	parsable::generate::generate_expression_parsable,
};
use helpers::combine_generics;
use parsable::attributes::{LocalContext, OuterAttributes};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput};

#[proc_macro_derive(
	Parsable,
	attributes(parst, matches, assert_eq, assert_ne, with_context, with_field_context)
)]
pub fn derive_parsable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let derive_input = parse_macro_input!(input as DeriveInput);
	proc_macro::TokenStream::from(process_input_parsable(&derive_input))
}

fn process_input_parsable(input: &DeriveInput) -> TokenStream {
	let ident = &input.ident;
	let generics = &input.generics;

	let outer_attributes = OuterAttributes::from_attributes(&input.attrs);
	let local_context = LocalContext::from(outer_attributes);

	let expression = generate_expression_parsable(input, &local_context);

	let mut combined_generics = combine_generics(generics, &local_context.new_generics());
	let src_lifetime = local_context.src_lifetime;
	let src_type = local_context.src_type;
	let ctx_pat = local_context.ctx_pat;
	let ctx_type = local_context.ctx_type;

	let predicates = &mut combined_generics.make_where_clause().predicates;
	if local_context.src_is_generic {
		predicates.push(parse_quote! { #src_type: ?Sized });
	}

	let (combined_impl_generics, _, combined_where) = combined_generics.split_for_impl();

	quote! {
		#[automatically_derived]
		impl #combined_impl_generics ::parst::Parsable<#src_lifetime, #src_type, #ctx_type> for #ident #generics #combined_where {
			fn read(__source: &#src_lifetime #src_type, #ctx_pat: #ctx_type) -> ::parst::PResult<Self, #src_type> {
				#![allow(non_snake_case)]
				#expression
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
		#[automatically_derived]
		impl ::parst::Deparsable for #ident #generics
		{
			fn write(&self, mut __w: impl ::std::io::Write) -> ::std::io::Result<()> {
				#![allow(non_snake_case)]
				#expression
			}
		}
	}
}
