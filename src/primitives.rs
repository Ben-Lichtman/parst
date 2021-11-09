use crate::{error::Error, PResult, Parsable};
use std::array::try_from_fn;

impl Parsable<'_> for u8 {
	fn read(bytes: &[u8]) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, bytes @ ..] => (*a, bytes),
			_ => return Err(Error::InvalidInput),
		};

		Ok((head, bytes))
	}
}

impl Parsable<'_> for i8 {
	fn read(bytes: &[u8]) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, bytes @ ..] => (*a as _, bytes),
			_ => return Err(Error::InvalidInput),
		};

		Ok((head, bytes))
	}
}

impl Parsable<'_> for u16 {
	fn read(bytes: &[u8]) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, bytes @ ..] => (Self::from_le_bytes([*a, *b]), bytes),
			_ => return Err(Error::InvalidInput),
		};

		Ok((head, bytes))
	}
}

impl Parsable<'_> for i16 {
	fn read(bytes: &[u8]) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, bytes @ ..] => (Self::from_le_bytes([*a, *b]), bytes),
			_ => return Err(Error::InvalidInput),
		};

		Ok((head, bytes))
	}
}

impl Parsable<'_> for u32 {
	fn read(bytes: &[u8]) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, c, d, bytes @ ..] => (Self::from_le_bytes([*a, *b, *c, *d]), bytes),
			_ => return Err(Error::InvalidInput),
		};

		Ok((head, bytes))
	}
}

impl Parsable<'_> for i32 {
	fn read(bytes: &[u8]) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, c, d, bytes @ ..] => (Self::from_le_bytes([*a, *b, *c, *d]), bytes),
			_ => return Err(Error::InvalidInput),
		};

		Ok((head, bytes))
	}
}

impl Parsable<'_> for u64 {
	fn read(bytes: &[u8]) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, c, d, e, f, g, h, bytes @ ..] => {
				(Self::from_le_bytes([*a, *b, *c, *d, *e, *f, *g, *h]), bytes)
			}
			_ => return Err(Error::InvalidInput),
		};

		Ok((head, bytes))
	}
}

impl Parsable<'_> for i64 {
	fn read(bytes: &[u8]) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, c, d, e, f, g, h, bytes @ ..] => {
				(Self::from_le_bytes([*a, *b, *c, *d, *e, *f, *g, *h]), bytes)
			}
			_ => return Err(Error::InvalidInput),
		};

		Ok((head, bytes))
	}
}

impl Parsable<'_> for f32 {
	fn read(bytes: &[u8]) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, c, d, bytes @ ..] => (Self::from_le_bytes([*a, *b, *c, *d]), bytes),
			_ => return Err(Error::InvalidInput),
		};

		Ok((head, bytes))
	}
}

impl Parsable<'_> for f64 {
	fn read(bytes: &[u8]) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, c, d, e, f, g, h, bytes @ ..] => {
				(Self::from_le_bytes([*a, *b, *c, *d, *e, *f, *g, *h]), bytes)
			}
			_ => return Err(Error::InvalidInput),
		};

		Ok((head, bytes))
	}
}

impl<'a> Parsable<'a> for &'a [u8] {
	fn read(bytes: &'a [u8]) -> PResult<Self> { Ok((bytes, &[])) }
}

impl<'a, T> Parsable<'a> for Vec<T>
where
	T: Parsable<'a>,
{
	fn read(mut bytes: &'a [u8]) -> PResult<Self> {
		let mut v = Vec::new();
		while let Ok((element, remainder)) = T::read(bytes) {
			v.push(element);
			bytes = remainder;
		}
		Ok((v, bytes))
	}
}

impl<'a, T> Parsable<'a> for Box<T>
where
	T: Parsable<'a>,
{
	fn read(bytes: &'a [u8]) -> PResult<Self> {
		let (boxed, bytes) = T::read(bytes)?;
		Ok((Box::new(boxed), bytes))
	}
}

impl<'a, T> Parsable<'a> for Option<T>
where
	T: Parsable<'a>,
{
	fn read(bytes: &'a [u8]) -> PResult<Self> {
		match T::read(bytes) {
			Ok((inner, bytes)) => Ok((Some(inner), bytes)),
			Err(_) => Ok((None, bytes)),
		}
	}
}

impl<'a, T, const N: usize> Parsable<'a> for [T; N]
where
	T: Parsable<'a>,
{
	fn read(mut bytes: &'a [u8]) -> PResult<Self> {
		try_from_fn(|_| {
			let (element, this_bytes) = Parsable::read(bytes)?;
			bytes = this_bytes;
			Ok(element)
		})
		.map(|array| (array, bytes))
	}
}

impl<'a, A> Parsable<'a> for (A,)
where
	A: Parsable<'a>,
{
	fn read(bytes: &'a [u8]) -> PResult<Self> {
		let (a, bytes) = Parsable::read(bytes)?;
		Ok(((a,), bytes))
	}
}

impl<'a, A, B> Parsable<'a> for (A, B)
where
	A: Parsable<'a>,
	B: Parsable<'a>,
{
	fn read(bytes: &'a [u8]) -> PResult<Self> {
		let (a, bytes) = Parsable::read(bytes)?;
		let (b, bytes) = Parsable::read(bytes)?;
		Ok(((a, b), bytes))
	}
}

impl<'a, A, B, C> Parsable<'a> for (A, B, C)
where
	A: Parsable<'a>,
	B: Parsable<'a>,
	C: Parsable<'a>,
{
	fn read(bytes: &'a [u8]) -> PResult<Self> {
		let (a, bytes) = Parsable::read(bytes)?;
		let (b, bytes) = Parsable::read(bytes)?;
		let (c, bytes) = Parsable::read(bytes)?;
		Ok(((a, b, c), bytes))
	}
}

impl<'a, A, B, C, D> Parsable<'a> for (A, B, C, D)
where
	A: Parsable<'a>,
	B: Parsable<'a>,
	C: Parsable<'a>,
	D: Parsable<'a>,
{
	fn read(bytes: &'a [u8]) -> PResult<Self> {
		let (a, bytes) = Parsable::read(bytes)?;
		let (b, bytes) = Parsable::read(bytes)?;
		let (c, bytes) = Parsable::read(bytes)?;
		let (d, bytes) = Parsable::read(bytes)?;
		Ok(((a, b, c, d), bytes))
	}
}

impl<'a, A, B, C, D, E> Parsable<'a> for (A, B, C, D, E)
where
	A: Parsable<'a>,
	B: Parsable<'a>,
	C: Parsable<'a>,
	D: Parsable<'a>,
	E: Parsable<'a>,
{
	fn read(bytes: &'a [u8]) -> PResult<Self> {
		let (a, bytes) = Parsable::read(bytes)?;
		let (b, bytes) = Parsable::read(bytes)?;
		let (c, bytes) = Parsable::read(bytes)?;
		let (d, bytes) = Parsable::read(bytes)?;
		let (e, bytes) = Parsable::read(bytes)?;
		Ok(((a, b, c, d, e), bytes))
	}
}

impl<'a, A, B, C, D, E, F> Parsable<'a> for (A, B, C, D, E, F)
where
	A: Parsable<'a>,
	B: Parsable<'a>,
	C: Parsable<'a>,
	D: Parsable<'a>,
	E: Parsable<'a>,
	F: Parsable<'a>,
{
	fn read(bytes: &'a [u8]) -> PResult<Self> {
		let (a, bytes) = Parsable::read(bytes)?;
		let (b, bytes) = Parsable::read(bytes)?;
		let (c, bytes) = Parsable::read(bytes)?;
		let (d, bytes) = Parsable::read(bytes)?;
		let (e, bytes) = Parsable::read(bytes)?;
		let (f, bytes) = Parsable::read(bytes)?;
		Ok(((a, b, c, d, e, f), bytes))
	}
}

impl<'a, A, B, C, D, E, F, G> Parsable<'a> for (A, B, C, D, E, F, G)
where
	A: Parsable<'a>,
	B: Parsable<'a>,
	C: Parsable<'a>,
	D: Parsable<'a>,
	E: Parsable<'a>,
	F: Parsable<'a>,
	G: Parsable<'a>,
{
	fn read(bytes: &'a [u8]) -> PResult<Self> {
		let (a, bytes) = Parsable::read(bytes)?;
		let (b, bytes) = Parsable::read(bytes)?;
		let (c, bytes) = Parsable::read(bytes)?;
		let (d, bytes) = Parsable::read(bytes)?;
		let (e, bytes) = Parsable::read(bytes)?;
		let (f, bytes) = Parsable::read(bytes)?;
		let (g, bytes) = Parsable::read(bytes)?;
		Ok(((a, b, c, d, e, f, g), bytes))
	}
}

impl<'a, A, B, C, D, E, F, G, H> Parsable<'a> for (A, B, C, D, E, F, G, H)
where
	A: Parsable<'a>,
	B: Parsable<'a>,
	C: Parsable<'a>,
	D: Parsable<'a>,
	E: Parsable<'a>,
	F: Parsable<'a>,
	G: Parsable<'a>,
	H: Parsable<'a>,
{
	fn read(bytes: &'a [u8]) -> PResult<Self> {
		let (a, bytes) = Parsable::read(bytes)?;
		let (b, bytes) = Parsable::read(bytes)?;
		let (c, bytes) = Parsable::read(bytes)?;
		let (d, bytes) = Parsable::read(bytes)?;
		let (e, bytes) = Parsable::read(bytes)?;
		let (f, bytes) = Parsable::read(bytes)?;
		let (g, bytes) = Parsable::read(bytes)?;
		let (h, bytes) = Parsable::read(bytes)?;
		Ok(((a, b, c, d, e, f, g, h), bytes))
	}
}
