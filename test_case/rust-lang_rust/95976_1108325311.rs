
struct Outer<'a, B: 'a + ?Sized> {
  first: u32,
  ref_unsized: &'a Foo<B>,
  second: &'a str,
}

struct Foo<B: ?Sized> {
  a: u32,
  b: B
}

struct Bar {
  c: u32,
  d: u32,
}

const ARR: Foo<[Bar; 2]> = Foo {
       a: 0,
       b: [Bar { c: 1, d: 2 }, Bar { c: 3, d: 4 }]
   };  

const SIZED: Outer<[Bar]> = Outer { first: 1, ref_unsized: &ARR, second: "Outer"};

fn main() {}
