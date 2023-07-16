rust
#![feature(test)]

extern crate serde;
extern crate serde_json;
extern crate test;

#[derive(Clone, Default)]
struct Area(u32);

impl serde::Serialize for Area {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        let mut state = try!(serializer.serialize_struct("Area", 1));
        try!(serializer.serialize_struct_elt(&mut state, "blockIds", &[(); 0]));
        serializer.serialize_struct_end(state)
    }
}

#[bench]
fn bench(b: &mut test::Bencher) {
    let areas = vec![Area::default(); 65536];
    let len = serde_json::to_string(&areas).unwrap().len();
    let mut buf = Vec::with_capacity(len);
    b.iter(|| {
        buf.clear();
        serde_json::to_writer(&mut buf, &areas).unwrap();
    });
}
