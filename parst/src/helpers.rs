#[inline]
pub fn try_split_array<S, const N: usize>(input: &[S]) -> Option<(&[S; N], &[S])> {
	input.split_first_chunk::<N>()
}
