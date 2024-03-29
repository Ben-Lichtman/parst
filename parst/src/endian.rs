use crate::{error::Error, helpers::try_split_array, Deparsable, PResultBytes, Parsable};

#[derive(Debug, Clone)]
pub struct LE<T>(pub T);

impl<T> AsRef<T> for LE<T> {
	fn as_ref(&self) -> &T { &self.0 }
}

impl<T> AsMut<T> for LE<T> {
	fn as_mut(&mut self) -> &mut T { &mut self.0 }
}

#[derive(Debug, Clone)]
pub struct BE<T>(pub T);

impl<T> AsRef<T> for BE<T> {
	fn as_ref(&self) -> &T { &self.0 }
}

impl<T> AsMut<T> for BE<T> {
	fn as_mut(&mut self) -> &mut T { &mut self.0 }
}

macro_rules! impl_prim {
	($ty:ident $size:literal) => {
		impl Parsable<'_, [u8]> for LE<$ty> {
			#[inline]
			fn read(source: &[u8], _context: ()) -> PResultBytes<Self> {
				let (head, source) =
					try_split_array::<_, $size>(source).ok_or((Error::NotEnoughBytes, source))?;
				let prim = $ty::from_le_bytes(*head);
				Ok((Self(prim), source))
			}
		}

		impl Parsable<'_, [u8]> for BE<$ty> {
			#[inline]
			fn read(source: &[u8], _context: ()) -> PResultBytes<Self> {
				let (head, source) =
					try_split_array::<_, $size>(source).ok_or((Error::NotEnoughBytes, source))?;
				let prim = $ty::from_be_bytes(*head);
				Ok((Self(prim), source))
			}
		}

		impl Deparsable for LE<$ty> {
			#[inline]
			fn write(&mut self, w: &mut impl std::io::Write, _context: ()) -> std::io::Result<()> {
				w.write_all(&self.0.to_le_bytes())
			}
		}

		impl Deparsable for BE<$ty> {
			#[inline]
			fn write(&mut self, w: &mut impl std::io::Write, _context: ()) -> std::io::Result<()> {
				w.write_all(&self.0.to_be_bytes())
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
