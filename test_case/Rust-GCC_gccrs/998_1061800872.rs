rust
macro_rules! define_vars {
    ($($v:ident)*) => { $(let $v = 15;)* }
}

// This is valid and should parse items
fn main() {
    define_vars!(f g h i);
}
