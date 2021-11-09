use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("invalid input")]
	InvalidInput,
}
