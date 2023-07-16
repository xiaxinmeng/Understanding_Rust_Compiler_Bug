rs
trait Trait<Dummy> {
    type Type;
}

impl<T, Dummy> Trait<Dummy> for T {
    type Type = T;
}

struct StrRef<'a>(&'a str);

impl<'a, 'b> From<&'a str> for StrRef<'b> {
    fn from(s: <&'a str as Trait<&'b &'a str>>::Type) -> StrRef<'b> {
        Self(s)
    }
}

fn main() {
    let x = String::from("Hello World!");
    let y = StrRef::from(x.as_str()).0;
    drop(x);
    println!("{}", y);
}
