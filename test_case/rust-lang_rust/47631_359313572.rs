rust
// Inlined
pub fn generic<T>(t: T) -> T { t }

// Inlined
#[inline]
pub fn generic_inline<T>(t: T) -> T { t }

// Not inlined
pub fn non_generic(i: i32) -> i32 { i + 10 }

// Inlined
#[inline]
pub fn non_generic_inline(i: i32) -> i32 { i + 10 }

impl GenericTrait<i32> for i32 {
    // Not inlined
    fn generic(self) -> i32 { self }

    // Inlined
    #[inline]
    fn generic_inline(self) -> i32 { self }

    // Inlined
    fn generic_method<U>(self, u: U) -> (i32, U) { (self, u) }

    // Inlined
    #[inline]
    fn generic_method_inline<U>(self, u: U) -> (i32, U) { (self, u) }
}

pub trait GenericTrait<T> {
    fn generic(self) -> T;
    fn generic_inline(self) -> T;
    fn generic_method<U>(self, u: U) -> (T, U);
    fn generic_method_inline<U>(self, u: U) -> (T, U);
}
