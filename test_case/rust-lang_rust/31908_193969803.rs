 rust
mod foo {
    pub type Result = ();
}

mod bar {
    // use super::*; // uncommenting this line would import `Result` from `super`'s prelude ...
    use foo::*;

    fn f() -> Result { () } // ... making this an ambiguity error.
}
