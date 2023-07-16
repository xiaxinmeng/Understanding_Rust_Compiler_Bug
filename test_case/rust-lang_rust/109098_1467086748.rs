`rust
#![crate_type = "lib"]

trait Visit {
    fn visit() {}
}

trait Array<'a> {}

impl Visit for () where
    (): for<'a> Array<'a, >,
{}
