 rust
fn main() {
    set_alloc_error_hook(|| abort()); // OOM = abort, maybe

    dependency::function(); // this is totally free to change the OOM handler

    // ..
}
