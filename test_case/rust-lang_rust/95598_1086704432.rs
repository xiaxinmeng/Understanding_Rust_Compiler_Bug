rust
fn cmp<P:AsRef<Path>>(name:P) -> impl Fn(&dyn AsRef<Path>)->bool
{
    move |s| {
        return name.as_ref().ends_with(s)
    }
}
