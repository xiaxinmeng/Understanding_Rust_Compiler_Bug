rust
#[repr(C)]
struct Pair<A, B> {
   a: A,
   b: B,
}

assert_eq!(Layout::new::<Pair<Foo, Bar>>(),
           Layout::new::<Foo>().extend(Layout::new::<Bar>()))
