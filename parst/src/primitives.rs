use crate::{
	helpers::try_split_array, Deparsable, PResult, PResultBytes, PResultBytesCounted,
	PResultCounted, PResultStr, Parsable,
};

impl<'a, Src> Parsable<'a, Src> for () {
	fn read(source: &'a Src, _context: ()) -> PResult<Self, Src> { Ok(((), source)) }

	fn read_counted(source: &'a Src, _context: (), index: usize) -> PResultCounted<Self, Src> {
		Ok(((), source, index))
	}
}

impl Deparsable for () {
	fn write(&self, _w: &mut impl std::io::Write) -> std::io::Result<()> { Ok(()) }
}

impl<'a> Parsable<'a, [u8]> for &'a [u8] {
	fn read(source: &'a [u8], _context: ()) -> PResultBytes<'a, Self> { Ok((source, &[])) }

	fn read_counted(source: &'a [u8], _context: (), index: usize) -> PResultCounted<Self, [u8]> {
		Ok((source, &[], index))
	}
}

impl Deparsable for &[u8] {
	fn write(&self, w: &mut impl std::io::Write) -> std::io::Result<()> { w.write_all(self) }
}

impl<'a> Parsable<'a, str> for &'a str {
	fn read(source: &'a str, _context: ()) -> PResultStr<'_, Self> { Ok((source, "")) }

	fn read_counted(source: &'a str, _context: (), index: usize) -> PResultCounted<Self, str> {
		Ok((source, "", index))
	}
}

impl Deparsable for &str {
	fn write(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
		w.write_all(self.as_bytes())
	}
}

impl<'a, const N: usize> Parsable<'a, [u8]> for &'a [u8; N] {
	fn read(source: &'a [u8], _context: ()) -> PResultBytes<Self> {
		let (output, source) = try_split_array(source)?;
		Ok((output, source))
	}

	fn read_counted(source: &'a [u8], _context: (), index: usize) -> PResultCounted<Self, [u8]> {
		let (output, source) = try_split_array(source)?;
		Ok((output, source, index + N))
	}
}

impl<const N: usize> Deparsable for &[u8; N] {
	fn write(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
		w.write_all(self.as_ref())
	}
}

macro_rules! impl_prim {
	($ty:ident $size:literal) => {
		impl Parsable<'_, [u8]> for $ty {
			fn read(source: &[u8], _context: ()) -> PResultBytes<Self> {
				let (head, source) = try_split_array::<_, $size>(source)?;
				let prim = $ty::from_ne_bytes(*head);
				Ok((prim, source))
			}

			fn read_counted(
				source: &[u8],
				_context: (),
				index: usize,
			) -> PResultBytesCounted<Self> {
				let (head, source) = try_split_array::<_, $size>(source)?;
				let prim = $ty::from_ne_bytes(*head);
				Ok((prim, source, index + $size))
			}
		}

		impl Deparsable for $ty {
			fn write(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
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
