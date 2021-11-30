use quote::ToTokens;
use syn::{Attribute, Lit, LitInt, Meta, MetaNameValue, NestedMeta, Type};

#[derive(Debug)]
pub struct OuterAttributes {
	pub context: Option<Type>,
}

pub fn parse_outer_attributes(input: &[Attribute]) -> OuterAttributes {
	let mut outer_attributes = OuterAttributes { context: None };

	input
		.into_iter()
		.flat_map(|a| {
			let args = a.parse_args::<NestedMeta>().ok()?;
			Some((a.path.clone(), args))
		})
		.filter(|(path, _)| path.to_token_stream().to_string() == "parst")
		.for_each(|(_, meta)| match meta {
			NestedMeta::Meta(Meta::NameValue(MetaNameValue {
				path,
				lit: Lit::Str(litstr),
				..
			})) => match path.into_token_stream().to_string().as_str() {
				"context" => {
					if let Ok(c) = litstr.parse::<Type>() {
						outer_attributes.context = Some(c);
					}
				}
				_ => (),
			},
			_ => (),
		});

	outer_attributes
}

#[derive(Debug)]
pub struct InnerAttributes {
	pub with_context: bool,
	pub assert_eq: Option<LitInt>,
	pub assert_ne: Option<LitInt>,
}

pub fn parse_inner_attributes(input: &[Attribute]) -> InnerAttributes {
	let mut inner_attributes = InnerAttributes {
		with_context: false,
		assert_eq: None,
		assert_ne: None,
	};

	input
		.into_iter()
		.flat_map(|a| {
			let args = a.parse_args::<NestedMeta>().ok()?;
			Some((a.path.clone(), args))
		})
		.for_each(|(path, meta)| {
			let path_str = path.into_token_stream().to_string();
			match path_str.as_str() {
				"parst" => {
					if let NestedMeta::Meta(Meta::Path(path)) = meta {
						let path_str = path.into_token_stream().to_string();
						if path_str.as_str() == "with_context" {
							inner_attributes.with_context = true;
						}
					}
				}
				"assert_eq" => {
					if let NestedMeta::Lit(Lit::Int(l)) = meta {
						inner_attributes.assert_eq = Some(l);
					}
				}
				"assert_ne" => {
					if let NestedMeta::Lit(Lit::Int(l)) = meta {
						inner_attributes.assert_ne = Some(l);
					}
				}
				x => panic!("{:#?}", x),
			}
		});

	inner_attributes
}
