rust
pub fn hello_world<'a>(hello: &'a String, world: &'a String) -> impl Future<Output=Result<(), Error>> + 'a {
    (async move || {
        may_fail(hello, world)?;
        Ok(())
    })()
}
