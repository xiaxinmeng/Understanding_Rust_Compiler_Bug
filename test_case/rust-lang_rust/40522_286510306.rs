
const FOO_: usize = 0;
const FOO: &'static usize = &FOO_;

fn bar() -> &'static usize {
    FOO
}

fn main() {
    bar();
}
