rust
#![feature(decl_macro)]

macro a($b:ident) {
    macro $b() {}
}

b!();
a!(b);
