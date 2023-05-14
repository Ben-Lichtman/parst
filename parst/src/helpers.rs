#[inline]
pub fn try_split_array<S, const N: usize>(input: &[S]) -> Option<(&[S; N], &[S])> {
	(input.len() >= N).then(|| input.split_array_ref::<N>())
}
