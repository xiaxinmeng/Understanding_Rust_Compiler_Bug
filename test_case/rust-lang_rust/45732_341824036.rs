rust
#![feature(decl_macro)]

pub macro once($mac:ident) {
    macro $mac() {}
    // If we just put `macro once() {}` here like as in the original example, it wouldn't be usable outside the macro definition due to hygiene and wouldn't be an error.
}

once!(once); // error
