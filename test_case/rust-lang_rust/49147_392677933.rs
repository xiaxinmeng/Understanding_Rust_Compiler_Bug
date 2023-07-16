rust
enum Wrap { ... }
struct MyStruct<'a> {
  arr: &'a mut [Option<Wrap>]
}
impl<'a> MyStruct<'a> {
  pub fn new(list: &'a mut [Option<Wrap>]) -> Self { Self{ arr: list} } }
}
// works fine, and is very ergonomic
let mut z = MyStruct{ arr: &mut [] }; 
// however the logical follow-up and equally ergonomic &mut [None;N] fails...

// all fine, Copy-trait not needed (*1)
let mut empty1: [_;11] = Default::default();
let mut a = MyStruct{ arr: &mut empty1 };

// need type ascription, Copy-trait not needed (*2)
let mut empty2 = <[_;11]>::default();
let mut b = MyStruct::new(&mut empty2);

// need type ascription, Copy-trait for Wrap
let mut empty3 = [Default::default();11]; // (*3)
let mut empty4 = [Option::default();11]; // (*4)
let mut empty5 = [None;11]; // (*5)

// need type ascription, parenthesized expression ... but alas, lifetime fails when (expr)
let mut c = MyStruct{ arr: &mut [Default::default();11] }; // (*6) , also parens expr
let mut d = MyStruct::new( &mut [Default::default();11] ); // (*6b) , also parens expr

// need Copy-trait for Wrap
let mut e = MyStruct{ arr: &mut [Default::default();11] }; // (*7)
let mut f = MyStruct{ arr: &mut [None;11] }; // (*8)
let mut g = MyStruct::new(&mut [None;11]); // (*9)
