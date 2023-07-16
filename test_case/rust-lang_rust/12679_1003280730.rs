rust
fn set_callback<F>(_: F)
where
    F: FnOnce(&mut [u8])
{ }

fn foo() {
    set_callback(|_| ());
}
