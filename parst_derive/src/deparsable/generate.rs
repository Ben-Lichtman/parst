use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DataEnum, DataStruct, DeriveInput, Field, Fields, Pat};

use crate::attributes::{parse_field_attributes, InnerContext, LocalContext};

pub fn generate_expression_deparsable(input: &DeriveInput, ctx: &LocalContext) -> TokenStream {
	match &input.data {
		Data::Struct(s) => generate_struct(s, ctx),
		Data::Enum(e) => generate_enum(e, ctx),
		_ => panic!("Can not derive deparsable for union"),
	}
}

fn field_name((index, field): (usize, &Field)) -> TokenStream {
	match &field.ident {
		Some(ident) => quote! { #ident },
		None => {
			let ident = format_ident!("__field_{}", index);
			quote! { #ident }
		}
	}
}

fn field_name_borrow((index, field): (usize, &Field)) -> TokenStream {
	match &field.ident {
		Some(ident) => quote! { #ident },
		None => quote! { #index },
	}
}

fn generate_struct(input: &DataStruct, ctx: &LocalContext) -> TokenStream {
	let field_names = input
		.fields
		.iter()
		.enumerate()
		.map(field_name_borrow)
		.collect::<Vec<_>>();

	let writes = input
		.fields
		.iter()
		.zip(field_names.iter())
		.map(|(field, name)| gen_write(field, name, &ctx.ctx_pat))
		.collect::<Vec<_>>();

	quote! {
		let Self { #( #field_names ),* } = self;
		#( #writes )*
		Ok(())
	}
}

fn generate_enum(input: &DataEnum, ctx: &LocalContext) -> TokenStream {
	let matches = input
		.variants
		.iter()
		.map(|variant| {
			let name = &variant.ident;

			let field_names = variant
				.fields
				.iter()
				.enumerate()
				.map(field_name)
				.collect::<Vec<_>>();

			let writes = variant
				.fields
				.iter()
				.zip(field_names.iter())
				.map(|(field, name)| gen_write(field, name, &ctx.ctx_pat))
				.collect::<Vec<_>>();

			let pattern = match variant.fields {
				Fields::Named(_) => quote! { { #(#field_names),* } },
				Fields::Unnamed(_) => quote! { ( #(#field_names),* ) },
				Fields::Unit => quote! {},
			};

			quote! {
				Self::#name #pattern => {
					#( #writes )*
				}
			}
		})
		.collect::<TokenStream>();

	quote! {
		match self {
			#matches
		}
		Ok(())
	}
}

fn gen_write(Field { attrs, ty, .. }: &Field, name: &TokenStream, ctx_pat: &Pat) -> TokenStream {
	let field_attributes = parse_field_attributes(attrs);

	let mut tokens = Vec::new();

	let assignment = match field_attributes.context {
		InnerContext::None => quote! {
			<#ty as ::parst::Deparsable<_>>::write(#name, __w, ())?;
		},
		InnerContext::Inherit => quote! {
			<#ty as ::parst::Deparsable<_>>::write(#name, __w, #ctx_pat)?;
		},
		InnerContext::Expr(e) => quote! {
			<#ty as ::parst::Deparsable<_>>::write(#name, __w, { #e })?;
		},
	};
	tokens.push(assignment);

	quote! {
		#( #tokens )*
	}
}
