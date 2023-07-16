rust
fn unwrap_if_io_error<'a>(mut e: &'a (dyn Error + 'static)) -> &'a (dyn Error + 'static) {
    while e.is::<io::Error>() {
        e = e.downcast_ref::<io::Error>()
            .and_then(|e| e.get_ref().map(|e| e as _))
            .unwrap();
    }
    e
}
