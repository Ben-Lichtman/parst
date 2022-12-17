use crate::{error::Error, PResult};

pub fn try_split_array<S, const N: usize>(input: &[S]) -> PResult<&[S; N], [S]> {
	(input.len() >= N)
		.then(|| input.split_array_ref::<N>())
		.ok_or(Error::NotEnoughBytes)
}
