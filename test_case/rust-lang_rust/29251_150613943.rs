 rust
#![feature(const_fn)]

const fn f(t: *const u8) {}

extern {
    static A: *const u8;
}

static B: () = f(*&A);

fn main() {}
