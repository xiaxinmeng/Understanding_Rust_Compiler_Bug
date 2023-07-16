rust
fn nest<F: FnOnce() -> ()>(f: F) -> impl FnOnce() -> () {
    || f()
}

fn inner(val: &'static str) -> impl FnOnce() -> () {
    move || {
        val;
    }
}

fn main() {
    nest(nest(nest(nest(nest(
        nest(nest(nest(nest(nest(
            nest(nest(nest(nest(nest(
                nest(nest(nest(nest(nest(
                    inner("")
                )))))
            )))))
        )))))
    )))));
}
