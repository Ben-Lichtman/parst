use std::collections::HashSet;
use syn::{
	punctuated::Punctuated,
	token::{Comma, Where},
	GenericParam, Generics, WhereClause, WherePredicate,
};

pub fn make_where_clause(i: impl Iterator<Item = WherePredicate>) -> Option<WhereClause> {
	let where_clauses = i.collect::<Punctuated<_, Comma>>();
	(!where_clauses.is_empty()).then_some(WhereClause {
		where_token: Where::default(),
		predicates: where_clauses,
	})
}

pub fn combine_where_clauses(
	a: &Option<WhereClause>,
	b: &Option<WhereClause>,
) -> Option<WhereClause> {
	let where_clauses = a
		.iter()
		.flat_map(|a_w| a_w.predicates.iter())
		.chain(b.iter().flat_map(|a_w| a_w.predicates.iter()));
	make_where_clause(where_clauses.cloned())
}

pub fn combine_generics(a: &Generics, b: &Generics) -> Generics {
	let mut c = Generics::default();

	let mut found = HashSet::new();
	let mut ordered = Vec::new();

	a.lifetimes()
		.cloned()
		.map(GenericParam::Lifetime)
		.chain(b.lifetimes().cloned().map(GenericParam::Lifetime))
		.chain(a.type_params().cloned().map(GenericParam::Type))
		.chain(b.type_params().cloned().map(GenericParam::Type))
		.chain(a.const_params().cloned().map(GenericParam::Const))
		.chain(b.const_params().cloned().map(GenericParam::Const))
		.for_each(|param| {
			if !found.contains(&param) {
				found.insert(param.clone());
				ordered.push(param);
			}
		});

	c.params.extend(ordered);
	c.where_clause = combine_where_clauses(&a.where_clause, &b.where_clause);
	c
}
