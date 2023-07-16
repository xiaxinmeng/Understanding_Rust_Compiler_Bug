rust
fn make_ne_enum() -> NeEnum {
    // works
    NeEnum::Variant
}

fn make_ne_enum_2() -> NeEnum {
    // fails
    NeEnum::Variant{}
}
