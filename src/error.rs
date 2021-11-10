use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("invalid input")]
	InvalidInput,
	#[error("not enough bytes")]
	NotEnoughBytes,
	#[error("assertion failed")]
	AssertionFailed,
}
