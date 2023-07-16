 rust
#[feature(macro_rules)];

macro_rules! bind {
    ($( $p:pat $(: $t:ty)* = $val:expr );* in $rhs:expr) => {
        {
            $( let $p $(: $t)* = $val; )*
            $rhs
        }
    }
}

fn main() {
    let v = bind!(a: int = 1; b = 2 in a + b);
    println!("{}", v);
}
