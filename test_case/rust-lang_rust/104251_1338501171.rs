rust
#![feature(inherent_associated_types)]

struct S<T>(T);

impl S<u8> {
    type P = ();
}

impl S<String> {
    type P = bool;
}

fn main() {
    let _: S<String>::P = false;
    //                    ^^^^^ expected `()`, found `bool` // <- this is incorrect!
    //                                                            the code should pass tycheck
}
