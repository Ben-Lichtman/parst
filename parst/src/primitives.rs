use crate::{error::Error, PResult, Parsable};
use std::{array::try_from_fn, marker::PhantomData};

fn try_split_at(input: &[u8], at: usize) -> PResult<&[u8]> {
	(input.len() >= at)
		.then(|| input.split_at(at))
		.ok_or(Error::NotEnoughBytes)
}

impl<C> Parsable<'_, C> for u8
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, bytes @ ..] => (*a, bytes),
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((head, bytes))
	}
}

impl<C> Parsable<'_, C> for i8
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, bytes @ ..] => (*a as _, bytes),
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((head, bytes))
	}
}

impl<C> Parsable<'_, C> for u16
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, bytes @ ..] => (Self::from_le_bytes([*a, *b]), bytes),
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((head, bytes))
	}
}

impl<C> Parsable<'_, C> for i16
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, bytes @ ..] => (Self::from_le_bytes([*a, *b]), bytes),
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((head, bytes))
	}
}

impl<C> Parsable<'_, C> for u32
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, c, d, bytes @ ..] => (Self::from_le_bytes([*a, *b, *c, *d]), bytes),
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((head, bytes))
	}
}

impl<C> Parsable<'_, C> for i32
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, c, d, bytes @ ..] => (Self::from_le_bytes([*a, *b, *c, *d]), bytes),
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((head, bytes))
	}
}

impl<C> Parsable<'_, C> for u64
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, c, d, e, f, g, h, bytes @ ..] => {
				(Self::from_le_bytes([*a, *b, *c, *d, *e, *f, *g, *h]), bytes)
			}
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((head, bytes))
	}
}

impl<C> Parsable<'_, C> for i64
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, c, d, e, f, g, h, bytes @ ..] => {
				(Self::from_le_bytes([*a, *b, *c, *d, *e, *f, *g, *h]), bytes)
			}
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((head, bytes))
	}
}

impl<C> Parsable<'_, C> for f32
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, c, d, bytes @ ..] => (Self::from_le_bytes([*a, *b, *c, *d]), bytes),
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((head, bytes))
	}
}

impl<C> Parsable<'_, C> for f64
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, c, d, e, f, g, h, bytes @ ..] => {
				(Self::from_le_bytes([*a, *b, *c, *d, *e, *f, *g, *h]), bytes)
			}
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((head, bytes))
	}
}

impl<'a, C> Parsable<'a, C> for &'a [u8]
where
	C: Copy,
{
	fn read(bytes: &'a [u8], _context: C) -> PResult<Self> { Ok((bytes, &[])) }
}

impl<'a, C, const N: usize> Parsable<'a, C> for &'a [u8; N]
where
	C: Copy,
{
	fn read(bytes: &'a [u8], _context: C) -> PResult<Self> {
		let (head, bytes) = try_split_at(bytes, N)?;
		// SAFETY: at this point we know that the slice is large enough
		let arry_ref = unsafe { &*head.as_ptr().cast() };
		Ok((arry_ref, bytes))
	}
}

impl<'a, C, T> Parsable<'a, C> for Vec<T>
where
	C: Copy,
	T: Parsable<'a, C>,
{
	fn read(mut bytes: &'a [u8], context: C) -> PResult<Self> {
		let mut v = Vec::new();
		while let Ok((element, remainder)) = Parsable::read(bytes, context) {
			v.push(element);
			bytes = remainder;
		}
		Ok((v, bytes))
	}
}

impl<'a, C, T> Parsable<'a, C> for Box<T>
where
	C: Copy,
	T: Parsable<'a, C>,
{
	fn read(bytes: &'a [u8], context: C) -> PResult<Self> {
		let (boxed, bytes) = Parsable::read(bytes, context)?;
		Ok((Box::new(boxed), bytes))
	}
}

impl<'a, C, T> Parsable<'a, C> for Option<T>
where
	C: Copy,
	T: Parsable<'a, C>,
{
	fn read(bytes: &'a [u8], context: C) -> PResult<Self> {
		match Parsable::read(bytes, context) {
			Ok((inner, bytes)) => Ok((Some(inner), bytes)),
			Err(_) => Ok((None, bytes)),
		}
	}
}

impl<C, T> Parsable<'_, C> for PhantomData<T>
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> { Ok((PhantomData, bytes)) }
}

impl<'a, C, T, const N: usize> Parsable<'a, C> for [T; N]
where
	C: Copy,
	T: Parsable<'a, C>,
{
	fn read(mut bytes: &'a [u8], context: C) -> PResult<Self> {
		try_from_fn(|_| {
			let (element, this_bytes) = Parsable::read(bytes, context)?;
			bytes = this_bytes;
			Ok(element)
		})
		.map(|array| (array, bytes))
	}
}

impl<C> Parsable<'_, C> for ()
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> { Ok(((), bytes)) }
}

impl<'a, Ctx, A> Parsable<'a, Ctx> for (A,)
where
	Ctx: Copy,
	A: Parsable<'a, Ctx>,
{
	fn read(bytes: &'a [u8], context: Ctx) -> PResult<Self> {
		let (a, bytes) = Parsable::read(bytes, context)?;
		Ok(((a,), bytes))
	}
}

impl<'a, Ctx, A, B> Parsable<'a, Ctx> for (A, B)
where
	Ctx: Copy,
	A: Parsable<'a, Ctx>,
	B: Parsable<'a, Ctx>,
{
	fn read(bytes: &'a [u8], context: Ctx) -> PResult<Self> {
		let (a, bytes) = Parsable::read(bytes, context)?;
		let (b, bytes) = Parsable::read(bytes, context)?;
		Ok(((a, b), bytes))
	}
}

impl<'a, Ctx, A, B, C> Parsable<'a, Ctx> for (A, B, C)
where
	Ctx: Copy,
	A: Parsable<'a, Ctx>,
	B: Parsable<'a, Ctx>,
	C: Parsable<'a, Ctx>,
{
	fn read(bytes: &'a [u8], context: Ctx) -> PResult<Self> {
		let (a, bytes) = Parsable::read(bytes, context)?;
		let (b, bytes) = Parsable::read(bytes, context)?;
		let (c, bytes) = Parsable::read(bytes, context)?;
		Ok(((a, b, c), bytes))
	}
}

impl<'a, Ctx, A, B, C, D> Parsable<'a, Ctx> for (A, B, C, D)
where
	Ctx: Copy,
	A: Parsable<'a, Ctx>,
	B: Parsable<'a, Ctx>,
	C: Parsable<'a, Ctx>,
	D: Parsable<'a, Ctx>,
{
	fn read(bytes: &'a [u8], context: Ctx) -> PResult<Self> {
		let (a, bytes) = Parsable::read(bytes, context)?;
		let (b, bytes) = Parsable::read(bytes, context)?;
		let (c, bytes) = Parsable::read(bytes, context)?;
		let (d, bytes) = Parsable::read(bytes, context)?;
		Ok(((a, b, c, d), bytes))
	}
}

impl<'a, Ctx, A, B, C, D, E> Parsable<'a, Ctx> for (A, B, C, D, E)
where
	Ctx: Copy,
	A: Parsable<'a, Ctx>,
	B: Parsable<'a, Ctx>,
	C: Parsable<'a, Ctx>,
	D: Parsable<'a, Ctx>,
	E: Parsable<'a, Ctx>,
{
	fn read(bytes: &'a [u8], context: Ctx) -> PResult<Self> {
		let (a, bytes) = Parsable::read(bytes, context)?;
		let (b, bytes) = Parsable::read(bytes, context)?;
		let (c, bytes) = Parsable::read(bytes, context)?;
		let (d, bytes) = Parsable::read(bytes, context)?;
		let (e, bytes) = Parsable::read(bytes, context)?;
		Ok(((a, b, c, d, e), bytes))
	}
}

impl<'a, Ctx, A, B, C, D, E, F> Parsable<'a, Ctx> for (A, B, C, D, E, F)
where
	Ctx: Copy,
	A: Parsable<'a, Ctx>,
	B: Parsable<'a, Ctx>,
	C: Parsable<'a, Ctx>,
	D: Parsable<'a, Ctx>,
	E: Parsable<'a, Ctx>,
	F: Parsable<'a, Ctx>,
{
	fn read(bytes: &'a [u8], context: Ctx) -> PResult<Self> {
		let (a, bytes) = Parsable::read(bytes, context)?;
		let (b, bytes) = Parsable::read(bytes, context)?;
		let (c, bytes) = Parsable::read(bytes, context)?;
		let (d, bytes) = Parsable::read(bytes, context)?;
		let (e, bytes) = Parsable::read(bytes, context)?;
		let (f, bytes) = Parsable::read(bytes, context)?;
		Ok(((a, b, c, d, e, f), bytes))
	}
}

impl<'a, Ctx, A, B, C, D, E, F, G> Parsable<'a, Ctx> for (A, B, C, D, E, F, G)
where
	Ctx: Copy,
	A: Parsable<'a, Ctx>,
	B: Parsable<'a, Ctx>,
	C: Parsable<'a, Ctx>,
	D: Parsable<'a, Ctx>,
	E: Parsable<'a, Ctx>,
	F: Parsable<'a, Ctx>,
	G: Parsable<'a, Ctx>,
{
	fn read(bytes: &'a [u8], context: Ctx) -> PResult<Self> {
		let (a, bytes) = Parsable::read(bytes, context)?;
		let (b, bytes) = Parsable::read(bytes, context)?;
		let (c, bytes) = Parsable::read(bytes, context)?;
		let (d, bytes) = Parsable::read(bytes, context)?;
		let (e, bytes) = Parsable::read(bytes, context)?;
		let (f, bytes) = Parsable::read(bytes, context)?;
		let (g, bytes) = Parsable::read(bytes, context)?;
		Ok(((a, b, c, d, e, f, g), bytes))
	}
}

impl<'a, Ctx, A, B, C, D, E, F, G, H> Parsable<'a, Ctx> for (A, B, C, D, E, F, G, H)
where
	Ctx: Copy,
	A: Parsable<'a, Ctx>,
	B: Parsable<'a, Ctx>,
	C: Parsable<'a, Ctx>,
	D: Parsable<'a, Ctx>,
	E: Parsable<'a, Ctx>,
	F: Parsable<'a, Ctx>,
	G: Parsable<'a, Ctx>,
	H: Parsable<'a, Ctx>,
{
	fn read(bytes: &'a [u8], context: Ctx) -> PResult<Self> {
		let (a, bytes) = Parsable::read(bytes, context)?;
		let (b, bytes) = Parsable::read(bytes, context)?;
		let (c, bytes) = Parsable::read(bytes, context)?;
		let (d, bytes) = Parsable::read(bytes, context)?;
		let (e, bytes) = Parsable::read(bytes, context)?;
		let (f, bytes) = Parsable::read(bytes, context)?;
		let (g, bytes) = Parsable::read(bytes, context)?;
		let (h, bytes) = Parsable::read(bytes, context)?;
		Ok(((a, b, c, d, e, f, g, h), bytes))
	}
}
