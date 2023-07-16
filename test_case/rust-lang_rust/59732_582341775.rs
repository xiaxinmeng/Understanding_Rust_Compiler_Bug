rust
use std::borrow::Cow;
use std::collections::BTreeMap;

struct MyMap(BTreeMap<Cow<'static, str>, Vec<u8>>);

impl MyMap {
    fn get<'a, 'b: 'a>(&'a self, c: &'b Cow<str>) -> Option<&'a Vec<u8>> {
        self.0.get(c)
    }

    fn dummy(&self) {
        let y: &Vec<u8> = {
            let s = String::from("Hey");
            self.0.get(&s.into() as &Cow<str>).unwrap()
        };
        println!("{:?}", y);
    }
}

fn main() {
    let mut m = BTreeMap::new();
    m.insert("Hey".into(), vec![1, 2, 3]);
    let m = MyMap(m);
    m.dummy();
}
