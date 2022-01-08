use crate::{error::Error, Deparsable, PResult, Parsable};

pub struct LE<T>(T);

impl<T> AsRef<T> for LE<T> {
	fn as_ref(&self) -> &T { &self.0 }
}

impl<C> Parsable<'_, C> for LE<u8>
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, bytes @ ..] => (*a, bytes),
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((LE(head), bytes))
	}
}

impl Deparsable for LE<u8> {
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> { w.write_all(&[self.0]) }
}

impl<C> Parsable<'_, C> for LE<i8>
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, bytes @ ..] => (*a as _, bytes),
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((LE(head), bytes))
	}
}

impl Deparsable for LE<i8> {
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		w.write_all(&[self.0 as _])
	}
}

impl<C> Parsable<'_, C> for LE<u16>
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, bytes @ ..] => (u16::from_le_bytes([*a, *b]), bytes),
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((LE(head), bytes))
	}
}

impl Deparsable for LE<u16> {
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		w.write_all(&self.0.to_le_bytes())
	}
}

impl<C> Parsable<'_, C> for LE<i16>
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, bytes @ ..] => (i16::from_le_bytes([*a, *b]), bytes),
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((LE(head), bytes))
	}
}

impl Deparsable for LE<i16> {
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		w.write_all(&self.0.to_le_bytes())
	}
}

impl<C> Parsable<'_, C> for LE<u32>
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, c, d, bytes @ ..] => (u32::from_le_bytes([*a, *b, *c, *d]), bytes),
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((LE(head), bytes))
	}
}

impl Deparsable for LE<u32> {
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		w.write_all(&self.0.to_le_bytes())
	}
}

impl<C> Parsable<'_, C> for LE<i32>
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, c, d, bytes @ ..] => (i32::from_le_bytes([*a, *b, *c, *d]), bytes),
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((LE(head), bytes))
	}
}

impl Deparsable for LE<i32> {
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		w.write_all(&self.0.to_le_bytes())
	}
}

impl<C> Parsable<'_, C> for LE<u64>
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, c, d, e, f, g, h, bytes @ ..] => {
				(u64::from_le_bytes([*a, *b, *c, *d, *e, *f, *g, *h]), bytes)
			}
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((LE(head), bytes))
	}
}

impl Deparsable for LE<u64> {
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		w.write_all(&self.0.to_le_bytes())
	}
}

impl<C> Parsable<'_, C> for LE<i64>
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, c, d, e, f, g, h, bytes @ ..] => {
				(i64::from_le_bytes([*a, *b, *c, *d, *e, *f, *g, *h]), bytes)
			}
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((LE(head), bytes))
	}
}

impl Deparsable for LE<i64> {
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		w.write_all(&self.0.to_le_bytes())
	}
}

pub struct BE<T>(T);

impl<T> AsRef<T> for BE<T> {
	fn as_ref(&self) -> &T { &self.0 }
}

impl<C> Parsable<'_, C> for BE<u8>
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, bytes @ ..] => (*a, bytes),
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((BE(head), bytes))
	}
}

impl Deparsable for BE<u8> {
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> { w.write_all(&[self.0]) }
}

impl<C> Parsable<'_, C> for BE<i8>
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, bytes @ ..] => (*a as _, bytes),
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((BE(head), bytes))
	}
}

impl Deparsable for BE<i8> {
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		w.write_all(&[self.0 as _])
	}
}

impl<C> Parsable<'_, C> for BE<u16>
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, bytes @ ..] => (u16::from_be_bytes([*a, *b]), bytes),
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((BE(head), bytes))
	}
}

impl Deparsable for BE<u16> {
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		w.write_all(&self.0.to_be_bytes())
	}
}

impl<C> Parsable<'_, C> for BE<i16>
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, bytes @ ..] => (i16::from_be_bytes([*a, *b]), bytes),
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((BE(head), bytes))
	}
}

impl Deparsable for BE<i16> {
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		w.write_all(&self.0.to_be_bytes())
	}
}

impl<C> Parsable<'_, C> for BE<u32>
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, c, d, bytes @ ..] => (u32::from_be_bytes([*a, *b, *c, *d]), bytes),
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((BE(head), bytes))
	}
}

impl Deparsable for BE<u32> {
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		w.write_all(&self.0.to_be_bytes())
	}
}

impl<C> Parsable<'_, C> for BE<i32>
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, c, d, bytes @ ..] => (i32::from_be_bytes([*a, *b, *c, *d]), bytes),
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((BE(head), bytes))
	}
}

impl Deparsable for BE<i32> {
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		w.write_all(&self.0.to_be_bytes())
	}
}

impl<C> Parsable<'_, C> for BE<u64>
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, c, d, e, f, g, h, bytes @ ..] => {
				(u64::from_be_bytes([*a, *b, *c, *d, *e, *f, *g, *h]), bytes)
			}
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((BE(head), bytes))
	}
}

impl Deparsable for BE<u64> {
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		w.write_all(&self.0.to_be_bytes())
	}
}

impl<C> Parsable<'_, C> for BE<i64>
where
	C: Copy,
{
	fn read(bytes: &[u8], _context: C) -> PResult<Self> {
		let (head, bytes) = match bytes {
			[a, b, c, d, e, f, g, h, bytes @ ..] => {
				(i64::from_be_bytes([*a, *b, *c, *d, *e, *f, *g, *h]), bytes)
			}
			_ => return Err(Error::NotEnoughBytes),
		};

		Ok((BE(head), bytes))
	}
}

impl Deparsable for BE<i64> {
	fn write(&self, mut w: impl std::io::Write) -> std::io::Result<()> {
		w.write_all(&self.0.to_be_bytes())
	}
}
