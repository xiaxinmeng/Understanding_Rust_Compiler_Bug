rust
mod my {
    pub struct Foo(&'static str);
}

impl AsRef<str> for my::Foo {
    fn as_ref(&self) -> &str {
        let Self(s) = self; // previously compiled, now errors correctly
        s
    }
}
