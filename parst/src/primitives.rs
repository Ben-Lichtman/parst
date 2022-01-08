use crate::{error::Error, Deparsable, PResult, Parsable};
use std::{array::try_from_fn, marker::PhantomData, ops::Deref};

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

impl Deparsable for u8 {
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> { w.write_all(&[*self]) }
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

impl Deparsable for i8 {
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		w.write_all(&[*self as _])
	}
}

impl<C> Parsable<'_, C> for u16
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, bytes @ ..] => (Self::from_ne_bytes([*a, *b]), bytes),
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((head, bytes))
	}
}

impl Deparsable for u16 {
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		w.write_all(&self.to_ne_bytes())
	}
}

impl<C> Parsable<'_, C> for i16
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, bytes @ ..] => (Self::from_ne_bytes([*a, *b]), bytes),
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((head, bytes))
	}
}

impl Deparsable for i16 {
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		w.write_all(&self.to_ne_bytes())
	}
}

impl<C> Parsable<'_, C> for u32
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, c, d, bytes @ ..] => (Self::from_ne_bytes([*a, *b, *c, *d]), bytes),
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((head, bytes))
	}
}

impl Deparsable for u32 {
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		w.write_all(&self.to_ne_bytes())
	}
}

impl<C> Parsable<'_, C> for i32
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, c, d, bytes @ ..] => (Self::from_ne_bytes([*a, *b, *c, *d]), bytes),
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((head, bytes))
	}
}

impl Deparsable for i32 {
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		w.write_all(&self.to_ne_bytes())
	}
}

impl<C> Parsable<'_, C> for u64
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, c, d, e, f, g, h, bytes @ ..] => {
				(Self::from_ne_bytes([*a, *b, *c, *d, *e, *f, *g, *h]), bytes)
			}
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((head, bytes))
	}
}

impl Deparsable for u64 {
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		w.write_all(&self.to_ne_bytes())
	}
}

impl<C> Parsable<'_, C> for i64
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, c, d, e, f, g, h, bytes @ ..] => {
				(Self::from_ne_bytes([*a, *b, *c, *d, *e, *f, *g, *h]), bytes)
			}
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((head, bytes))
	}
}

impl Deparsable for i64 {
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		w.write_all(&self.to_ne_bytes())
	}
}

impl<C> Parsable<'_, C> for f32
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, c, d, bytes @ ..] => (Self::from_ne_bytes([*a, *b, *c, *d]), bytes),
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((head, bytes))
	}
}

impl Deparsable for f32 {
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		w.write_all(&self.to_ne_bytes())
	}
}

impl<C> Parsable<'_, C> for f64
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, c, d, e, f, g, h, bytes @ ..] => {
				(Self::from_ne_bytes([*a, *b, *c, *d, *e, *f, *g, *h]), bytes)
			}
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((head, bytes))
	}
}

impl Deparsable for f64 {
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		w.write_all(&self.to_ne_bytes())
	}
}

impl<'a, C> Parsable<'a, C> for &'a [u8]
where
	C: Copy,
{
	fn read(bytes: &'a [u8], _context: C) -> PResult<Self> { Ok((bytes, &[])) }
}

impl Deparsable for &[u8] {
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> { w.write_all(self) }
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

impl<const N: usize> Deparsable for &[u8; N] {
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> { w.write_all(*self) }
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

impl<T> Deparsable for Vec<T>
where
	T: Deparsable,
{
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		self.iter().try_for_each(|element| element.write(&mut w))
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

impl<T> Deparsable for Box<T>
where
	T: Deparsable,
{
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		self.deref().write(&mut w)
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

impl<T> Deparsable for Option<T>
where
	T: Deparsable,
{
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		if let Some(inner) = self {
			inner.write(&mut w)?;
		}
		Ok(())
	}
}

impl<C, T> Parsable<'_, C> for PhantomData<T>
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> { Ok((PhantomData, bytes)) }
}

impl<T> Deparsable for PhantomData<T>
where
	T: Deparsable,
{
	fn write(&self, _w: impl std::io::Write) -> std::io::Result<()> { Ok(()) }
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

impl<T, const N: usize> Deparsable for [T; N]
where
	T: Deparsable,
{
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		self.iter().try_for_each(|element| element.write(&mut w))
	}
}

impl<C> Parsable<'_, C> for ()
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> { Ok(((), bytes)) }
}

impl Deparsable for () {
	fn write(&self, _w: impl std::io::Write) -> std::io::Result<()> { Ok(()) }
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

impl<A> Deparsable for (A,)
where
	A: Deparsable,
{
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		self.0.write(&mut w)?;
		Ok(())
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

impl<A, B> Deparsable for (A, B)
where
	A: Deparsable,
	B: Deparsable,
{
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		self.0.write(&mut w)?;
		self.1.write(&mut w)?;
		Ok(())
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

impl<A, B, C> Deparsable for (A, B, C)
where
	A: Deparsable,
	B: Deparsable,
	C: Deparsable,
{
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		self.0.write(&mut w)?;
		self.1.write(&mut w)?;
		self.2.write(&mut w)?;
		Ok(())
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

impl<A, B, C, D> Deparsable for (A, B, C, D)
where
	A: Deparsable,
	B: Deparsable,
	C: Deparsable,
	D: Deparsable,
{
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		self.0.write(&mut w)?;
		self.1.write(&mut w)?;
		self.2.write(&mut w)?;
		self.3.write(&mut w)?;
		Ok(())
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

impl<A, B, C, D, E> Deparsable for (A, B, C, D, E)
where
	A: Deparsable,
	B: Deparsable,
	C: Deparsable,
	D: Deparsable,
	E: Deparsable,
{
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		self.0.write(&mut w)?;
		self.1.write(&mut w)?;
		self.2.write(&mut w)?;
		self.3.write(&mut w)?;
		self.4.write(&mut w)?;
		Ok(())
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

impl<A, B, C, D, E, F> Deparsable for (A, B, C, D, E, F)
where
	A: Deparsable,
	B: Deparsable,
	C: Deparsable,
	D: Deparsable,
	E: Deparsable,
	F: Deparsable,
{
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		self.0.write(&mut w)?;
		self.1.write(&mut w)?;
		self.2.write(&mut w)?;
		self.3.write(&mut w)?;
		self.4.write(&mut w)?;
		self.5.write(&mut w)?;
		Ok(())
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

impl<A, B, C, D, E, F, G> Deparsable for (A, B, C, D, E, F, G)
where
	A: Deparsable,
	B: Deparsable,
	C: Deparsable,
	D: Deparsable,
	E: Deparsable,
	F: Deparsable,
	G: Deparsable,
{
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		self.0.write(&mut w)?;
		self.1.write(&mut w)?;
		self.2.write(&mut w)?;
		self.3.write(&mut w)?;
		self.4.write(&mut w)?;
		self.5.write(&mut w)?;
		self.6.write(&mut w)?;
		Ok(())
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

impl<A, B, C, D, E, F, G, H> Deparsable for (A, B, C, D, E, F, G, H)
where
	A: Deparsable,
	B: Deparsable,
	C: Deparsable,
	D: Deparsable,
	E: Deparsable,
	F: Deparsable,
	G: Deparsable,
	H: Deparsable,
{
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		self.0.write(&mut w)?;
		self.1.write(&mut w)?;
		self.2.write(&mut w)?;
		self.3.write(&mut w)?;
		self.4.write(&mut w)?;
		self.5.write(&mut w)?;
		self.6.write(&mut w)?;
		self.7.write(&mut w)?;
		Ok(())
	}
}
