rust
pub trait Trait<T> {
    type Type;
}

impl<T> Trait<T> for i32 {
    type Type = i32;
}

pub trait Trait2 {
    fn foo<T>(v: Self::Type)
    where
        Self: Trait<T>;
}

impl Trait2 for i32 {
    fn foo<T>(v: <Self as Trait<T>>::Type)
    where
        Self: Trait<T>,
    {
        let _: i32 = unsafe { std::mem::transmute(v) };
    }
}
