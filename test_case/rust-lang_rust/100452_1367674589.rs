rust
#[derive(Clone)]
struct Alpha;

fn clone_then_move_cloned() {
    fn foo<F: Fn()>(_: &Alpha, _: F) {}
    let x = Alpha;
    // ok, data is moved while the clone is in use.
    foo(&x, move || {
        let _ = x;
    });
}

fn main() { clone_then_move_cloned() }
