
struct Struct;

trait Trait {
    type Type;
}

fn foo<'a>(_arg: <&'a Struct as Trait>::Type) where &'a Struct: Trait {}
