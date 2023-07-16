rust
macro_rules! mk_struct {
    ($n:ident) => {
        struct $n { field: $n }
    }
}

mk_struct!(Foo);

fn main() { }
