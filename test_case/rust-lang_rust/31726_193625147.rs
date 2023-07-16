 rust
mod m {
    macro_rules! m { macro_rules! n { } }
}

mod n {
    m::m!; // Replacing this macro invocation with its expansion breaks the example ...
}

mod foo {
    use n::*;
    m::m!;
}

mod bar {
    foo::n!; // ... by making this a time travel error
}
