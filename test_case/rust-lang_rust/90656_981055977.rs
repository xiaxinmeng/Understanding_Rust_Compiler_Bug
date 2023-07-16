rust
fn symbol<'a>(s: &'a str) -> impl FnMut(&'a str) -> Result<&'a str, &'a str> {
    |_| unimplemented!()
}

static FOO: &(dyn FnMut(&str) -> Result<&str, &str> + Sync) = &symbol("*");

fn main() {}
