use crate::{
	error::Error, helpers::try_split_array, Deparsable, PResult, PResultBytes, PResultStr, Parsable,
};

impl<'a, Src> Parsable<'a, Src> for () {
	#[inline]
	fn read(source: &'a Src, _context: ()) -> PResult<Self, Src> { Ok(((), source)) }
}

impl Deparsable for () {
	#[inline]
	fn write(&mut self, _w: &mut impl std::io::Write, _context: ()) -> std::io::Result<()> {
		Ok(())
	}
}

impl<'a> Parsable<'a, [u8]> for &'a [u8] {
	#[inline]
	fn read(source: &'a [u8], _context: ()) -> PResultBytes<Self> { Ok((source, &[])) }
}

impl Deparsable for &[u8] {
	#[inline]
	fn write(&mut self, w: &mut impl std::io::Write, _context: ()) -> std::io::Result<()> {
		w.write_all(self)
	}
}

impl<'a> Parsable<'a, str> for &'a str {
	#[inline]
	fn read(source: &'a str, _context: ()) -> PResultStr<Self> { Ok((source, "")) }
}

impl Deparsable for &str {
	#[inline]
	fn write(&mut self, w: &mut impl std::io::Write, _context: ()) -> std::io::Result<()> {
		w.write_all(self.as_bytes())
	}
}

impl<'a, const N: usize> Parsable<'a, [u8]> for &'a [u8; N] {
	#[inline]
	fn read(source: &'a [u8], _context: ()) -> PResultBytes<Self> {
		let (output, source) = try_split_array(source).ok_or((Error::NotEnoughBytes, source))?;
		Ok((output, source))
	}
}

impl<const N: usize> Deparsable for &[u8; N] {
	#[inline]
	fn write(&mut self, w: &mut impl std::io::Write, _context: ()) -> std::io::Result<()> {
		w.write_all(self.as_ref())
	}
}

macro_rules! impl_prim {
	($ty:ident $size:literal) => {
		impl Parsable<'_, [u8]> for $ty {
			#[inline]
			fn read(source: &[u8], _context: ()) -> PResultBytes<Self> {
				let (head, source) =
					try_split_array::<_, $size>(source).ok_or((Error::NotEnoughBytes, source))?;
				let prim = $ty::from_ne_bytes(*head);
				Ok((prim, source))
			}
		}

		impl Deparsable for $ty {
			#[inline]
			fn write(&mut self, w: &mut impl std::io::Write, _context: ()) -> std::io::Result<()> {
				w.write_all(&self.to_ne_bytes())
			}
		}
	};
}

macro_rules! impl_prims {
	($ty:ident $size:literal) => {
		impl_prim!($ty $size);
	};
	($ty:ident $size:literal $( $other_ty:ident $other_size:literal )*) => {
		impl_prim!($ty $size);
        impl_prims!($( $other_ty $other_size )*);
	};
}

impl_prims!(u8 1 i8 1 u16 2 i16 2 u32 4 i32 4 u64 8 i64 8 f32 4 f64 8);
