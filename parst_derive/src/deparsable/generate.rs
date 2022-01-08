use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DataEnum, DataStruct, DeriveInput, Field, Fields};

pub fn generate_expression_deparsable(input: &DeriveInput) -> TokenStream {
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

fn field_name_borrow((index, field): (usize, &Field)) -> TokenStream {
	match &field.ident {
		Some(ident) => quote! { #ident },
		None => quote! { #index },
	}
}

fn generate_struct(input: &DataStruct) -> TokenStream {
	let field_names = input
		.fields
		.iter()
		.enumerate()
		.map(field_name_borrow)
		.collect::<Vec<_>>();

	let writes = quote! {
		#(::parst::Deparsable::write(&self.#field_names, &mut __w)?;)*
	};

	quote! {
		#writes
		Ok(())
	}
}

fn generate_enum(input: &DataEnum) -> TokenStream {
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

			let pattern = match variant.fields {
				Fields::Named(_) => quote! { { #(#field_names),* } },
				Fields::Unnamed(_) => quote! { ( #(#field_names),* ) },
				Fields::Unit => quote! {},
			};

			quote! {
				Self::#name #pattern => {
					#(::parst::Deparsable::write(#field_names, &mut __w)?;)*
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
