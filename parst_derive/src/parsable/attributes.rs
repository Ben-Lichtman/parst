use quote::ToTokens;
use std::ops::Deref;
use syn::{
	parse_quote, Attribute, Expr, FnArg, GenericParam, Generics, Lifetime, Lit, Meta,
	MetaNameValue, NestedMeta, Pat, PatType, Type,
};

#[derive(Debug, Default)]
pub struct OuterAttributes {
	pub lifetime: Option<Lifetime>,
	pub src: Option<Type>,
	pub ctx: Option<PatType>,
}

impl OuterAttributes {
	pub fn from_attributes(input: &[Attribute]) -> Self {
		let mut outer_attributes = Self::default();

		input
			.iter()
			.filter(|a| a.path == parse_quote! { parst })
			.filter_map(|a| a.parse_meta().ok())
			.filter_map(|meta| match meta {
				Meta::List(l) => Some(l.nested.into_iter()),
				_ => None,
			})
			.flatten()
			.for_each(|nested| match nested {
				NestedMeta::Meta(Meta::NameValue(MetaNameValue {
					path,
					lit: Lit::Str(litstr),
					..
				})) => {
					let path = path.into_token_stream().to_string();
					match path.deref() {
						"lifetime" => {
							if let Ok(x) = litstr.parse::<Lifetime>() {
								outer_attributes.lifetime = Some(x)
							}
						}
						"src" => {
							if let Ok(x) = litstr.parse::<Type>() {
								outer_attributes.src = Some(x)
							}
						}
						"ctx" => {
							if let Ok(FnArg::Typed(x)) = litstr.parse::<FnArg>() {
								outer_attributes.ctx = Some(x);
							}
						}
						x => panic!("{}", x),
					}
				}
				NestedMeta::Lit(_) => unimplemented!(),
				_ => unimplemented!(),
			});

		outer_attributes
	}
}

#[derive(Debug)]
pub struct LocalContext {
	pub src_lifetime: Lifetime,
	pub src_type: Type,
	pub src_is_generic: bool,
	pub ctx_pat: Pat,
	pub ctx_type: Type,
	pub ctx_is_generic: bool,
}

impl LocalContext {
	pub fn new_generics(&self) -> Generics {
		let src_lifetime = &self.src_lifetime;
		let mut new_generics: Vec<GenericParam> = vec![parse_quote! { #src_lifetime }];
		if self.src_is_generic {
			let src_type = &self.src_type;
			new_generics.push(parse_quote! { #src_type })
		}
		if self.ctx_is_generic {
			let ctx_type = &self.ctx_type;
			new_generics.push(parse_quote! { #ctx_type });
		}
		let new_generics = new_generics.into_iter();
		parse_quote! { <#(#new_generics),*> }
	}
}

impl From<OuterAttributes> for LocalContext {
	fn from(value: OuterAttributes) -> Self {
		let src_lifetime = value.lifetime.unwrap_or_else(|| parse_quote! { '__src });
		let (src_type, src_is_generic) = value
			.src
			.map(|x| (x, false))
			.unwrap_or_else(|| (parse_quote! { __Src }, true));
		let (ctx_pat, ctx_type, ctx_is_generic) = value
			.ctx
			.map(|x| (*x.pat, *x.ty, false))
			.unwrap_or_else(|| (parse_quote! { __context }, parse_quote! { __Ctx }, true));

		Self {
			src_lifetime,
			src_type,
			src_is_generic,
			ctx_pat,
			ctx_type,
			ctx_is_generic,
		}
	}
}

#[derive(Debug, Default)]
pub struct FieldAttributes {
	pub context: InnerContext,
	pub matches: Option<Pat>,
	pub assert_eq: Option<Expr>,
	pub assert_ne: Option<Expr>,
}

#[derive(Debug, Default)]
pub enum InnerContext {
	#[default]
	None,
	Inherit,
	Expr(Expr),
}

pub fn parse_field_attributes(input: &[Attribute]) -> FieldAttributes {
	let mut field_attributes = FieldAttributes::default();

	input
		.iter()
		.filter(|a| a.path == parse_quote! { parst })
		.filter_map(|a| a.parse_meta().ok())
		.filter_map(|meta| match meta {
			Meta::List(l) => Some(l.nested.into_iter()),
			_ => None,
		})
		.flatten()
		.for_each(|nested| match nested {
			NestedMeta::Meta(Meta::NameValue(MetaNameValue {
				path,
				lit: Lit::Str(l),
				..
			})) => match path.to_token_stream().to_string().as_str() {
				"ctx" => field_attributes.context = InnerContext::Expr(l.parse().unwrap()),
				"matches" => field_attributes.matches = Some(l.parse().unwrap()),
				"assert_eq" => field_attributes.assert_eq = Some(l.parse().unwrap()),
				"assert_ne" => field_attributes.assert_ne = Some(l.parse().unwrap()),
				x => panic!("{:#?}", x),
			},
			NestedMeta::Lit(Lit::Str(s)) => match s.value().as_str() {
				"inherit" => field_attributes.context = InnerContext::Inherit,
				_ => unimplemented!(),
			},
			_ => unimplemented!(),
		});

	field_attributes
}
