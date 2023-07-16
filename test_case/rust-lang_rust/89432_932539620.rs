rust
const FOO: fn() -> String = || "foo".into();

pub fn bar() -> fn() -> String {
    || "bar".into()
}
