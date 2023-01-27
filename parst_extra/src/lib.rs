pub mod var;

use parst::{Deparsable, Parsable};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Never;

impl<'a, Src, Ctx> Parsable<'a, Src, Ctx> for Never
where
	Src: ?Sized,
{
	fn read(_source: &'a Src, _context: Ctx) -> parst::PResult<Self, Src> {
		Err(parst::error::Error::InvalidInput)
	}

	fn read_counted(
		_source: &'a Src,
		_context: Ctx,
		_index: usize,
	) -> parst::PResultCounted<Self, Src> {
		Err(parst::error::Error::InvalidInput)
	}
}

impl Deparsable for Never {
	fn write(&self, _w: &mut impl std::io::Write) -> std::io::Result<()> {
		Err(std::io::Error::new(
			std::io::ErrorKind::PermissionDenied,
			"Can not deparse a Never",
		))
	}
}
