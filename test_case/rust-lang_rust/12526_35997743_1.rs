 rust
use c::int;
extern {
    // ... use `int` a lot ...
}

fn x() -> uint { computation() }

fn y() {
    let i = x() as int; // whoops, meant to be built-in `int`, lost precision on x86-64
    // ...
}
