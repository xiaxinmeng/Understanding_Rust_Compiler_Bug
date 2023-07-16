rust
use glob::Pattern;

struct Foo<'a>(&'a str);

struct Bar(Vec<String>);

impl Bar {
    fn use_pattern(&self, pattern: &Pattern) -> impl Iterator<Item = Foo<'_>> {
        self.0
            .iter()
            .filter(|toolchain| pattern.matches(toolchain))
            .map(|toolchain| Foo(&toolchain))
    }
}
