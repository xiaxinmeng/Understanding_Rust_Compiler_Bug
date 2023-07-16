rust
struct F;
struct T;

impl F {
    const V: i32 = 0;
}

impl T {
    const V: i32 = 0;
}

macro_rules! mac {
    ($( $v: ident = $s: ident,)*) => {
        enum E {
            $( $v = $s::V, )*
        }
    }
}

mac! {
    A = F,
    B = T,
}
