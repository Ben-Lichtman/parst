use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::iter::once;
use syn::{
	Data, DataEnum, DataStruct, DeriveInput, Field, Fields, FieldsNamed, FieldsUnnamed, Ident,
};

use crate::attributes::parse_inner_attributes;

pub fn generate_expression(input: &DeriveInput) -> TokenStream {
	match &input.data {
		Data::Struct(s) => generate_struct(s),
		Data::Enum(e) => generate_enum(e),
		_ => panic!("Can not derive parsable for union"),
	}
}

fn generate_struct(input: &DataStruct) -> TokenStream {
	let (assignments, finished) = gen_field_reads(&input.fields, quote! { Self });
	quote! {
		#assignments
		Ok((#finished, bytes))
	}
}

fn generate_enum(input: &DataEnum) -> TokenStream {
	input
		.variants
		.iter()
		.map(|variant| {
			let name = &variant.ident;
			let (assignments, finished) = gen_field_reads(&variant.fields, quote! { Self::#name });
			let fn_name = format_ident!("decode_{}", name);
			quote! {
				let #fn_name = || -> ::parst::PResult<Self> {
					#assignments
					Ok((#finished, bytes))
				};
				if let Ok(x) = #fn_name() {
					return Ok(x);
				}
			}
		})
		.chain(once(quote! {
			Err(::parst::error::Error::InvalidInput)
		}))
		.collect()
}

fn gen_field_reads(fields: &Fields, name: TokenStream) -> (TokenStream, TokenStream) {
	match fields {
		Fields::Named(FieldsNamed { named, .. }) => {
			let assignments = named
				.iter()
				.flat_map(|field| {
					let name = field.ident.clone().unwrap();
					gen_single_field(field, name)
				})
				.collect::<TokenStream>();

			let names = named
				.iter()
				.map(|Field { ident, .. }| {
					let name = ident.as_ref().unwrap();
					quote! { #name, }
				})
				.collect::<TokenStream>();

			(assignments, quote! { #name { #names } })
		}
		Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
			let assignments = unnamed
				.iter()
				.enumerate()
				.flat_map(|(n, field)| {
					let name = format_ident!("field_{}", n);
					gen_single_field(field, name)
				})
				.collect::<TokenStream>();

			let names = (0..unnamed.len())
				.map(|n| {
					let name = format_ident!("field_{}", n);
					quote! { #name, }
				})
				.collect::<TokenStream>();

			(assignments, quote! { #name ( #names ) })
		}
		Fields::Unit => (quote! {}, quote! { #name }),
	}
}

fn gen_single_field(Field { attrs, .. }: &Field, name: Ident) -> TokenStream {
	let inner_attributes = parse_inner_attributes(attrs);

	let mut tokens = TokenStream::new();

	match inner_attributes.with_context {
		false => tokens.extend(quote! {
			let (#name, bytes) = ::parst::Parsable::read(bytes, ())?;
		}),
		true => tokens.extend(quote! {
			let (#name, bytes) = ::parst::Parsable::read(bytes, _context)?;
		}),
	}
	if let Some(l) = inner_attributes.assert_eq {
		tokens.extend(quote! {
			if #name != #l {
				return Err(::parst::error::Error::AssertionFailed);
			}
		});
	}
	if let Some(l) = inner_attributes.assert_ne {
		tokens.extend(quote! {
			if #name == #l {
				return Err(::parst::error::Error::AssertionFailed);
			}
		});
	}

	tokens
}
