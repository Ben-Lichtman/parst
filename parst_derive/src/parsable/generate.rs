use crate::{
	helpers::field_name,
	parsable::attributes::{parse_field_attributes, InnerContext, LocalContext},
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DataEnum, DataStruct, DeriveInput, Field, Fields, Pat};

pub fn generate_expression_parsable(input: &DeriveInput, ctx: &LocalContext) -> TokenStream {
	match &input.data {
		Data::Struct(s) => generate_struct(s, ctx),
		Data::Enum(e) => generate_enum(e, ctx),
		_ => panic!("Can not derive parsable for union"),
	}
}

fn generate_struct(input: &DataStruct, ctx: &LocalContext) -> TokenStream {
	let field_names = input
		.fields
		.iter()
		.enumerate()
		.map(field_name)
		.collect::<Vec<_>>();

	let assignments = input
		.fields
		.iter()
		.zip(field_names.iter())
		.map(|(field, name)| gen_assign(field, name, &ctx.ctx_pat))
		.collect::<Vec<_>>();

	match input.fields {
		Fields::Named(_) => quote! {
			#( #assignments )*
			Ok((Self { #( #field_names ),* }, __source))
		},
		Fields::Unnamed(_) => quote! {
			#( #assignments )*
			Ok((Self ( #( #field_names ),* ), __source))
		},
		Fields::Unit => quote! {
			#( #assignments )*
			Ok((Self, __source))
		},
	}
}

fn generate_enum(input: &DataEnum, ctx: &LocalContext) -> TokenStream {
	let src_type = &ctx.src_type;
	let function_calls = input
		.variants
		.iter()
		.map(|variant| {
			let field_names = variant
				.fields
				.iter()
				.enumerate()
				.map(|(n, field)| match &field.ident {
					Some(ident) => quote! { #ident },
					None => {
						let ident = format_ident!("__field_{}", n);
						quote! { #ident }
					}
				})
				.collect::<Vec<_>>();

			let name = &variant.ident;
			let fn_name = format_ident!("decode_{}", name);

			let assignments = variant
				.fields
				.iter()
				.zip(field_names.iter())
				.map(|(field, name)| gen_assign(field, name, &ctx.ctx_pat))
				.collect::<Vec<_>>();

			let return_expr = match variant.fields {
				Fields::Named(_) => quote! {
					Ok((Self::#name { #( #field_names ),* }, __source))
				},
				Fields::Unnamed(_) => quote! {
					Ok((Self::#name ( #( #field_names ),* ), __source))
				},
				Fields::Unit => quote! {
					Ok((Self::#name, __source))
				},
			};

			let function_def = quote! {
				let #fn_name = || -> ::parst::PResult<Self, #src_type> {
					#( #assignments )*
					#return_expr
				};
			};

			let function_call = quote! {
				if let Ok(x) = #fn_name() {
					return Ok(x);
				}
			};
			quote! {
				#function_def
				#function_call
			}
		})
		.collect::<Vec<_>>();

	quote! {
		#( #function_calls )*
		Err((::parst::error::Error::InvalidInput, __source))
	}
}

fn gen_assign(Field { attrs, ty, .. }: &Field, name: &TokenStream, ctx_pat: &Pat) -> TokenStream {
	let field_attributes = parse_field_attributes(attrs);

	let mut tokens = Vec::new();

	let assignment = match field_attributes.context {
		InnerContext::None => quote! {
			let (#name, __source) = <#ty as ::parst::Parsable<_>>::read(__source, ())?;
		},
		InnerContext::Inherit => quote! {
			let (#name, __source) = <#ty as ::parst::Parsable<_>>::read(__source, #ctx_pat)?;
		},
		InnerContext::Expr(e) => quote! {
			let (#name, __source) = <#ty as ::parst::Parsable<_>>::read(__source, { #e })?;
		},
	};
	tokens.push(assignment);

	if let Some(pat) = field_attributes.matches {
		tokens.push(quote! {
			if !matches!(#name, #pat) {
				return Err((::parst::error::Error::AssertionFailed, __source));
			}
		})
	}
	if let Some(e) = field_attributes.assert_eq {
		tokens.push(quote! {
			if #name != #e {
				return Err((::parst::error::Error::AssertionFailed, __source));
			}
		});
	}
	if let Some(e) = field_attributes.assert_ne {
		tokens.push(quote! {
			if #name == #e {
				return Err((::parst::error::Error::AssertionFailed, __source));
			}
		});
	}

	quote! {
		#( #tokens )*
	}
}
