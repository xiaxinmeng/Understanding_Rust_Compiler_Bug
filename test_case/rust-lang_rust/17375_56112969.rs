 rust
#![feature(macro_rules)]

trait ImmutablePrimitiveSlice<'a, U, S> {
    fn as_unsigned(self) -> &'a [U];
    fn as_signed(self) -> &'a [S];
}

trait MutablePrimitiveSlice<'a, U, S>: ImmutablePrimitiveSlice<'a, U, S> {
    fn as_unsigned_mut(self) -> &'a mut [U];
    fn as_signed_mut(self) -> &'a mut [S];
}

macro_rules! prim_immut {
    ($u:ty, $s:ty, $t:ty) => {
        impl<'a> ImmutablePrimitiveSlice<'a, $u, $s> for $t {
            #[inline] fn as_unsigned(self) -> &'a [$u] { unsafe { std::mem::transmute(self) } }
            #[inline] fn as_signed(self) -> &'a [$s] { unsafe { std::mem::transmute(self) } }
        }
    }
}
macro_rules! prim_mut {
    ($u:ty, $s:ty, $t:ty) => {
        impl<'a> MutablePrimitiveSlice<'a, $u, $s> for $t {
            #[inline]
            fn as_unsigned_mut(self) -> &'a mut [$u] { unsafe { std::mem::transmute(self) } }
            #[inline]
            fn as_signed_mut(self) -> &'a mut [$s] { unsafe { std::mem::transmute(self) } }
        }
    }
}

macro_rules! prim {
    ($u:ty, $s:ty) => {
        prim_immut!($u, $s, &'a [$u])
        prim_immut!($u, $s, &'a [$s])
        prim_immut!($u, $s, &'a mut [$u])
        prim_immut!($u, $s, &'a mut [$s])
        prim_mut!($u, $s, &'a mut [$u])
        prim_mut!($u, $s, &'a mut [$s])
    }
}

prim!(u8,   i8)
prim!(u16,  i16)
prim!(u32,  i32)
prim!(u64,  i64)
prim!(uint, int)

fn main() {
    let x: &[i8] = [97, 98, 99];
    println!("{}", std::str::from_utf8(x.as_unsigned()));
}
