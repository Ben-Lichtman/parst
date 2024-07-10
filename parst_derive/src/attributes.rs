use quote::ToTokens;
use syn::{
	parse_quote, Attribute, Expr, FnArg, GenericParam, Generics, Lifetime, LitStr, Pat, PatType,
	Type,
};

#[derive(Debug, Default)]
pub struct OuterAttributes {
	pub lifetime: Option<Lifetime>,
	pub src: Option<Type>,
	pub ctx: Option<PatType>,
	pub dis: Option<Type>,
}

impl OuterAttributes {
	pub fn from_attributes(input: &[Attribute]) -> Self {
		let mut outer_attributes = Self::default();

		input
			.iter()
			.filter(|a| a.path().is_ident("parst"))
			.for_each(|a| {
				a.parse_nested_meta(|meta| {
					if let Some(ident) = meta.path.get_ident() {
						let ident_string = ident.into_token_stream().to_string();
						match ident_string.as_ref() {
							"lifetime" => {
								let value = meta.value().unwrap();
								let litstring = value.parse::<LitStr>().unwrap();
								let value = litstring.parse::<Lifetime>().unwrap();
								outer_attributes.lifetime = Some(value);
							}
							"src" => {
								let value = meta.value().unwrap();
								let litstring = value.parse::<LitStr>().unwrap();
								let value = litstring.parse::<Type>().unwrap();
								outer_attributes.src = Some(value);
							}
							"ctx" => {
								let value = meta.value().unwrap();
								let litstring = value.parse::<LitStr>().unwrap();
								let value = match litstring.parse::<FnArg>().unwrap() {
									FnArg::Receiver(_) => panic!(),
									FnArg::Typed(t) => t,
								};
								outer_attributes.ctx = Some(value);
							}
							"dis" => {
								let value = meta.value().unwrap();
								let litstring = value.parse::<LitStr>().unwrap();
								let value = litstring.parse::<Type>().unwrap();
								outer_attributes.dis = Some(value);
							}
							x => panic!("unknown attribute {}", x),
						}
					}

					Ok(())
				})
				.unwrap();
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
	pub dis_type: Option<Type>,
}

impl LocalContext {
	pub fn new_generics_for_parsable(&self) -> Generics {
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
		parse_quote! { <#( #new_generics ),*> }
	}

	pub fn new_generics_for_deparsable(&self) -> Generics {
		let mut new_generics: Vec<GenericParam> = vec![];
		if self.ctx_is_generic {
			let ctx_type = &self.ctx_type;
			new_generics.push(parse_quote! { #ctx_type });
		}
		let new_generics = new_generics.into_iter();
		parse_quote! { <#( #new_generics ),*> }
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
			dis_type: value.dis,
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
		.filter(|a| a.path().is_ident("parst"))
		.for_each(|a| {
			a.parse_nested_meta(|meta| {
				if let Some(ident) = meta.path.get_ident() {
					let ident_string = ident.into_token_stream().to_string();
					match ident_string.as_ref() {
						"ctx" => {
							let value = meta.value().unwrap();
							let litstring = value.parse::<LitStr>().unwrap();
							let value = litstring.parse::<Expr>().unwrap();
							field_attributes.context = InnerContext::Expr(value);
						}
						"matches" => {
							let value = meta.value().unwrap();
							let litstring = value.parse::<LitStr>().unwrap();
							let value = litstring
								.parse_with(Pat::parse_multi_with_leading_vert)
								.unwrap();
							field_attributes.matches = Some(value);
						}
						"assert_eq" => {
							let value = meta.value().unwrap();
							let litstring = value.parse::<LitStr>().unwrap();
							let value = litstring.parse::<Expr>().unwrap();
							field_attributes.assert_eq = Some(value);
						}
						"assert_ne" => {
							let value = meta.value().unwrap();
							let litstring = value.parse::<LitStr>().unwrap();
							let value = litstring.parse::<Expr>().unwrap();
							field_attributes.assert_ne = Some(value);
						}
						"with_context" => {
							let value = meta.value().unwrap();
							let litstring = value.parse::<LitStr>().unwrap();
							let value = litstring.parse::<Expr>().unwrap();
							field_attributes.context = InnerContext::Expr(value);
						}
						x => panic!("unknown attribute {}", x),
					}
				}

				Ok(())
			})
			.unwrap();
		});

	field_attributes
}

#[derive(Debug, Default)]
pub struct VariantAttributes {
	pub dis: Option<Expr>,
}

pub fn parse_variant_attributes(input: &[Attribute]) -> VariantAttributes {
	let mut variant_attributes = VariantAttributes::default();

	input
		.iter()
		.filter(|a| a.path().is_ident("parst"))
		.for_each(|a| {
			a.parse_nested_meta(|meta| {
				if let Some(ident) = meta.path.get_ident() {
					let ident_string = ident.into_token_stream().to_string();
					match ident_string.as_ref() {
						"dis" => {
							let value = meta.value().unwrap();
							let litstring = value.parse::<LitStr>().unwrap();
							let value = litstring.parse::<Expr>().unwrap();
							variant_attributes.dis = Some(value);
						}
						x => panic!("unknown attribute {}", x),
					}
				}

				Ok(())
			})
			.unwrap();
		});

	variant_attributes
}
