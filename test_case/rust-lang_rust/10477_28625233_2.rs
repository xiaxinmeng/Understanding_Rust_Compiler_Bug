 rust
fn dead_fn() {
}

#[main]
fn dead_fn2() {
} 

fn used_fn() {
}

#[start]
fn start(_: int, _: **u8) -> int {
    used_fn();
    0
}

// this is not main
fn main() {
    dead_fn();
    dead_fn2();
}
