rust
fn set_callback<F>(_: Option<F>)
where
    F: FnOnce(&mut [u8])
{ }

fn foo() {
    set_callback(Some(|_| ()));
}
