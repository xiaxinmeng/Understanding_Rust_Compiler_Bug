rust
pub enum Repro {
    A(i64),
    B(i64),
}

fn make(a: bool, data: i64) -> Repro {
    let enum_variant_constructor_fn_ptr: fn(i64) -> Repro = match a {
        true => Repro::A,
        false => Repro::B,
    };
    enum_variant_constructor_fn_ptr(data)
}
