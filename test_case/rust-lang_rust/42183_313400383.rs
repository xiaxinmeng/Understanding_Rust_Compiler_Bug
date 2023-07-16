
enum Foo {
   Vec(Vec<u8>),
   Set(HashSet<u8>,
}

fn mkiter(f: Foo) -> impl Iterator<Item=&u8> {
    match f {
      Foo::Vec(ref v) => v.iter(),
      Foo::Set(ref s) => s.iter(),
    }
}
