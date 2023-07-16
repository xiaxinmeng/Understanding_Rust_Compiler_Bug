rust
pub trait Foo {
    const CONST: bool;
}

impl <'a> Foo for () where &'a Self: Foo {
    const CONST: bool = <&Self>::CONST;
}
