use crate::parsable::attributes::{parse_inner_attributes, InnerContext};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DataEnum, DataStruct, DeriveInput, Field, Fields};

pub fn generate_expression_parsable(input: &DeriveInput) -> TokenStream {
	match &input.data {
		Data::Struct(s) => generate_struct(s),
		Data::Enum(e) => generate_enum(e),
		_ => panic!("Can not derive parsable for union"),
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

fn generate_struct(input: &DataStruct) -> TokenStream {
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
		.map(|(field, name)| gen_assign(field, name))
		.collect::<Vec<_>>();

	match input.fields {
		Fields::Named(_) => quote! {
			#( #assignments )*
			Ok((Self { #( #field_names ),* }, __bytes))
		},
		Fields::Unnamed(_) => quote! {
			#( #assignments )*
			Ok((Self ( #( #field_names ),* ), __bytes))
		},
		Fields::Unit => quote! {
			#( #assignments )*
			Ok((Self, __bytes))
		},
	}
}

fn generate_enum(input: &DataEnum) -> TokenStream {
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
				.map(|(field, name)| gen_assign(field, name))
				.collect::<Vec<_>>();

			let return_expr = match variant.fields {
				Fields::Named(_) => quote! {
					Ok((Self::#name { #( #field_names ),* }, __bytes))
				},
				Fields::Unnamed(_) => quote! {
					Ok((Self::#name ( #( #field_names ),* ), __bytes))
				},
				Fields::Unit => quote! {
					Ok((Self::#name, __bytes))
				},
			};

			let function_def = quote! {
				let #fn_name = || -> ::parst::PResult<Self> {
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
		Err(::parst::error::Error::InvalidInput)
	}
}

fn gen_assign(Field { attrs, ty, .. }: &Field, name: &TokenStream) -> TokenStream {
	let inner_attributes = parse_inner_attributes(attrs);

	let mut tokens = Vec::new();

	let assignment = match inner_attributes.with_context {
		InnerContext::None => quote! {
			let (#name, __bytes) = <#ty as ::parst::Parsable<_>>::read(__bytes, ())?;
		},
		InnerContext::Parent => quote! {
			let (#name, __bytes) = <#ty as ::parst::Parsable<_>>::read(__bytes, __context)?;
		},
		InnerContext::Field(f) => quote! {
			let (#name, __bytes) = <#ty as ::parst::Parsable<_>>::read(__bytes, #f)?;
		},
	};
	tokens.push(assignment);

	if let Some(l) = inner_attributes.assert_eq {
		tokens.push(quote! {
			if #name != #l {
				return Err(::parst::error::Error::AssertionFailed);
			}
		});
	}
	if let Some(l) = inner_attributes.assert_ne {
		tokens.push(quote! {
			if #name == #l {
				return Err(::parst::error::Error::AssertionFailed);
			}
		});
	}

	quote! {
		#( #tokens )*
	}
}
