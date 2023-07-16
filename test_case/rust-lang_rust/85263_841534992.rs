rust
#[derive(Copy, Clone)]
struct Pie {
    slices: u8,
    size: u8,
}

union Foo {
    bar: i8,
    baz: Pie
}

// safe with THIR and MIR checker:
match u {
    Foo { baz: _ } => {},
}

// unsafe to THIR, safe to MIR:
match u {
    Foo { baz: Pie { .. } } => {},
}
match u {
    Foo { baz: Pie { slices: _, size: _ } } => {},
}
