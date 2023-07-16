rust
struct S;

impl S {
    fn f() {}
}

macro_rules! impl_add {
    ($($n:ident)*) => {
        $(
            fn $n() {
                S::f::<i64>();
            }
        )*
    }
}

impl_add!(a b);
