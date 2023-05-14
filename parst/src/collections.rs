use crate::{Deparsable, PResult, Parsable};
use std::{array::try_from_fn, marker::PhantomData, ops::Deref};

impl<'a, T, Src, Ctx, const N: usize> Parsable<'a, Src, Ctx> for [T; N]
where
	Src: ?Sized,
	Ctx: Copy,
	T: Parsable<'a, Src, Ctx>,
{
	fn read(mut source: &'a Src, context: Ctx, mut index: usize) -> PResult<Self, Src> {
		try_from_fn(|_| {
			let (element, this_bytes, new_index) = Parsable::read(source, context, index)?;
			source = this_bytes;
			index = new_index;
			Ok(element)
		})
		.map(|array| (array, source, index))
	}
}

impl<T, const N: usize> Deparsable for [T; N]
where
	T: Deparsable,
{
	fn write(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
		self.iter().try_for_each(|element| element.write(&mut *w))
	}
}

macro_rules! impl_tuple {
	($name:ident $ty:ident) => {
        impl_tuple!(@impl $name $ty);
    };
    ($name:ident $ty:ident $( $N:ident $T:ident )+) => {
        impl_tuple!($( $N $T )+);
        impl_tuple!(@impl $name $ty $( $N $T )+);
    };
	(@impl $( $N:ident $T:ident )+) => {
		impl<'a, Src, Ctx, $( $T ),+> Parsable<'a, Src, Ctx> for ($( $T, )+)
		where
			Src: ?Sized,
            Ctx: Copy,
            $(
                $T: Parsable<'a, Src, Ctx>,
            )+
		{

			fn read(source: &'a Src, context: Ctx, index: usize) -> PResult<Self, Src> {
                $(
                    let ($N, source, index) = Parsable::read(source, context, index)?;
                )+
                Ok((($( $N, )+), source, index))
			}
		}

		impl<$( $T ),+> Deparsable for ($( $T, )+)
		where
			$(
				$T: Deparsable,
			)+
		{
			fn write(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
				let ( $( $N, )+ ) = self;
				$(
					$N.write(&mut *w)?;
				)+
				Ok(())
			}
		}
	};
}

impl_tuple!(a A b B c C d D e E f F g G h H);

impl<'a, Src, Ctx, T> Parsable<'a, Src, Ctx> for Vec<T>
where
	Src: ?Sized,
	Ctx: Copy,
	T: Parsable<'a, Src, Ctx>,
{
	fn read(mut source: &'a Src, context: Ctx, mut index: usize) -> PResult<Self, Src> {
		let mut v = Vec::new();
		while let Ok((element, remainder, new_index)) = Parsable::read(source, context, index) {
			v.push(element);
			source = remainder;
			index = new_index;
		}
		Ok((v, source, index))
	}
}

impl<T> Deparsable for Vec<T>
where
	T: Deparsable,
{
	fn write(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
		for element in self {
			element.write(&mut *w)?;
		}
		Ok(())
	}
}

impl<'a, Src, Ctx, T> Parsable<'a, Src, Ctx> for Box<T>
where
	Src: ?Sized,
	T: Parsable<'a, Src, Ctx>,
{
	fn read(source: &'a Src, context: Ctx, index: usize) -> PResult<Self, Src> {
		let (boxed, source, index) = Parsable::read(source, context, index)?;
		Ok((Box::new(boxed), source, index))
	}
}

impl<T> Deparsable for Box<T>
where
	T: Deparsable,
{
	fn write(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
		self.deref().write(&mut *w)
	}
}

impl<'a, Src, Ctx, T> Parsable<'a, Src, Ctx> for Option<T>
where
	Src: ?Sized,
	T: Parsable<'a, Src, Ctx>,
{
	fn read(source: &'a Src, context: Ctx, index: usize) -> PResult<Self, Src> {
		match Parsable::read(source, context, index) {
			Ok((inner, source, index)) => Ok((Some(inner), source, index)),
			Err(_) => Ok((None, source, index)),
		}
	}
}

impl<T> Deparsable for Option<T>
where
	T: Deparsable,
{
	fn write(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
		match self {
			Some(inner) => inner.write(&mut *w),
			None => Ok(()),
		}
	}
}

impl<Src, Ctx, T> Parsable<'_, Src, Ctx> for PhantomData<T>
where
	Src: ?Sized,
{
	fn read(source: &Src, _context: Ctx, index: usize) -> PResult<Self, Src> {
		Ok((PhantomData, source, index))
	}
}
