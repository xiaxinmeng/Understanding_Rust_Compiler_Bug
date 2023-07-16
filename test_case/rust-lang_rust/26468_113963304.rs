 rust
#[allow(dead_code)]
#[repr(u16)]
enum FooMode {
    First = 0x1000,
    Check = 0x1001,
    Last  = 0x100f,
}

#[allow(dead_code)]
#[repr(u16)]
enum BarMode {
    First = 0x2000,
    Check = 0x2001,
    Last  = 0x200f,
}

#[allow(dead_code)]
enum Mode {
    Foo(FooMode),
    Bar(BarMode),
}

#[inline(never)]
fn broken(mode: &Mode) -> u32 {
    for _ in 0..1 {
        if let Mode::Foo(FooMode::Check) = *mode { return 17 }
        if let Mode::Bar(BarMode::Check) = *mode { return 19 }
    }
    return 42;
}

fn main() {
    let mode = Mode::Bar(BarMode::Check);
    assert_eq!(broken(&mode), 19);
}
