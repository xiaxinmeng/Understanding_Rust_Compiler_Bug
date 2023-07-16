rust
use std::borrow::Cow;
use std::collections::BTreeMap;

struct MyMap(BTreeMap<Cow<'static,str>, Vec<u8>>);

impl MyMap {
    fn get<'a, 'b:'a>(&'a self, c: &'b Cow<str>) -> Option<&'a Vec<u8>> {
        self.0.get(c)
    }
}
