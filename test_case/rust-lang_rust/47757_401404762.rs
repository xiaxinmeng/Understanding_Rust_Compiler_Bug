rust
trait FromStr<'a = 'static> {
    fn from_str<'b>(s: &'b str) -> Self where 'a: 'b;
}

struct Foo<'b>(&'b str);
struct Bar(String);

impl FromStr for Bar {
    fn from_str(s: &str) -> Self {
        Bar(s.to_owned())
    }
}

impl<'c> FromStr<'c> for Foo<'c> {
    fn from_str(s: &'c) -> Self {
        Foo(s)
    }
}
