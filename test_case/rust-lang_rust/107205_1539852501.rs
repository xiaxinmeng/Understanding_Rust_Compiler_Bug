rust
// rustc src/main.rs -Zmir-opt-level=0
const X: for<'b> fn(&'b ()) = |&()| ();
fn main() {
    let dyn_debug = &X as &fn(&'static ()) as &dyn Send;
    drop(dyn_debug)
}
