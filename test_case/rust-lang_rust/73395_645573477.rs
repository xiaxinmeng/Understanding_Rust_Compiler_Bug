rust
enum Whatever {
    Variant(Foo, Bar, StructWithWierdInvariants),
    OtherVariant,
}

const WHATEVER_KIND_DISCR: Discriminant<Whatever> =
    mem::discriminant(&Whatever::Kind(...));

fn this_is_a_function(input: Whatever) {
    other_function_requiring_discriminant(input, WHATEVER_KIND_DISCR);
}
