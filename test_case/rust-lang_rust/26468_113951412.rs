 rust
#[inline(never)]
fn broken(mode: &Mode) -> u32 {
    for _ in 0..1 {
        if let Mode::Bar(BarMode::Bar1) = *mode { return 17 }
        if let Mode::Foo(5) = *mode { return 19 }
    }
    return 42;
}
