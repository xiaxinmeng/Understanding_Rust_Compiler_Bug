rs
fn f<F>()
where
    F: FnOnce() -> Box<FnOnce(&())>,
{
}
