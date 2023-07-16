rust
mod outer {
    macro_rules! foo {
        () => {
            "foo"
        };
    }

    macro_rules! bar {
        () => {
            concat!(foo!(), "bar")
        };
    }

    pub fn baz() {
        format!(bar!());
    }
}
