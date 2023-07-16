 rust
trait Tag<'a> {
    fn get_name(&'a self) -> &'a str;
}

struct FooTag {
    _name: ~str
}
impl<'a> Tag<'a> for FooTag {
    fn get_name(&'a self) -> &'a str { self._name.as_slice() }
}

fn main() {
    let tag : ~Tag = ~FooTag {_name: ~"hello" } as ~Tag;
    let s = tag.get_name();

}
