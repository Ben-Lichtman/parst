use crate::{error::Error, PResult, Parsable};

impl Parsable<'_> for u8 {
	fn read(i: &[u8]) -> PResult<Self> {
		let (head, tail) = match i {
			[a, tail @ ..] => (*a, tail),
			_ => return Err(Error::InvalidInput),
		};

		Ok((head, tail))
	}
}

impl Parsable<'_> for i8 {
	fn read(i: &[u8]) -> PResult<Self> {
		let (head, tail) = match i {
			[a, tail @ ..] => (*a as _, tail),
			_ => return Err(Error::InvalidInput),
		};

		Ok((head, tail))
	}
}

impl Parsable<'_> for u16 {
	fn read(i: &[u8]) -> PResult<Self> {
		let (head, tail) = match i {
			[a, b, tail @ ..] => (Self::from_le_bytes([*a, *b]), tail),
			_ => return Err(Error::InvalidInput),
		};

		Ok((head, tail))
	}
}

impl Parsable<'_> for i16 {
	fn read(i: &[u8]) -> PResult<Self> {
		let (head, tail) = match i {
			[a, b, tail @ ..] => (Self::from_le_bytes([*a, *b]), tail),
			_ => return Err(Error::InvalidInput),
		};

		Ok((head, tail))
	}
}

impl Parsable<'_> for u32 {
	fn read(i: &[u8]) -> PResult<Self> {
		let (head, tail) = match i {
			[a, b, c, d, tail @ ..] => (Self::from_le_bytes([*a, *b, *c, *d]), tail),
			_ => return Err(Error::InvalidInput),
		};

		Ok((head, tail))
	}
}

impl Parsable<'_> for i32 {
	fn read(i: &[u8]) -> PResult<Self> {
		let (head, tail) = match i {
			[a, b, c, d, tail @ ..] => (Self::from_le_bytes([*a, *b, *c, *d]), tail),
			_ => return Err(Error::InvalidInput),
		};

		Ok((head, tail))
	}
}

impl Parsable<'_> for u64 {
	fn read(i: &[u8]) -> PResult<Self> {
		let (head, tail) = match i {
			[a, b, c, d, e, f, g, h, tail @ ..] => {
				(Self::from_le_bytes([*a, *b, *c, *d, *e, *f, *g, *h]), tail)
			}
			_ => return Err(Error::InvalidInput),
		};

		Ok((head, tail))
	}
}

impl Parsable<'_> for i64 {
	fn read(i: &[u8]) -> PResult<Self> {
		let (head, tail) = match i {
			[a, b, c, d, e, f, g, h, tail @ ..] => {
				(Self::from_le_bytes([*a, *b, *c, *d, *e, *f, *g, *h]), tail)
			}
			_ => return Err(Error::InvalidInput),
		};

		Ok((head, tail))
	}
}

impl Parsable<'_> for f32 {
	fn read(i: &[u8]) -> PResult<Self> {
		let (head, tail) = match i {
			[a, b, c, d, tail @ ..] => (Self::from_le_bytes([*a, *b, *c, *d]), tail),
			_ => return Err(Error::InvalidInput),
		};

		Ok((head, tail))
	}
}

impl Parsable<'_> for f64 {
	fn read(i: &[u8]) -> PResult<Self> {
		let (head, tail) = match i {
			[a, b, c, d, e, f, g, h, tail @ ..] => {
				(Self::from_le_bytes([*a, *b, *c, *d, *e, *f, *g, *h]), tail)
			}
			_ => return Err(Error::InvalidInput),
		};

		Ok((head, tail))
	}
}

impl<'a> Parsable<'a> for &'a [u8] {
	fn read(i: &'a [u8]) -> PResult<Self> { Ok((i, &[])) }
}
