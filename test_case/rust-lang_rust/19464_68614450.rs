 rust
fn take_any(_: ||) {
}

fn take_const_owned(_: ||:Sync+Send) {
}

fn give_any(f: ||) {
    take_any(f);
}

fn give_owned(f: ||:Send) {
    take_any(f);
    take_const_owned(f);
}

fn main() {}
