rust
macro_rules! foo {
    () => {
        "foo"
    };
}

macro_rules! bar {
    () => {
        foo!()
    };
}

fn main() {
    format_args!(bar!()); // ERROR, `foo` is not found, happens because `format_args` is opaque now
}
