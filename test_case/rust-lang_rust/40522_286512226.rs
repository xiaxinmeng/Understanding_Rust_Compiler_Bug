rust
static FOO: usize = 0;

fn bar() -> &'static usize {
    &FOO
}

fn main() {
    bar();
}
