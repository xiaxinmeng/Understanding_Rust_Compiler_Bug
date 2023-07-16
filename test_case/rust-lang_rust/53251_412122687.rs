Rust
pub trait JavaAdd<T> {
    type Target;

    fn add(self, other: T) -> Self::Target;
}

macro_rules! impl_add {
    ($($primitive:ident),*) => {
        $(impl<T: Into<i64>> JavaAdd<T> for $primitive {
            type Target = i64;

            fn add(self, other: T) -> Self::Target {
                self.into::<i64>() + other.into::<i64>()
            }
        })*
    }
}

impl_add!(i8, i16);
