rust
use std::marker::PhantomData;
struct a {
  b : PhantomData< a >
}
pub struct c {
  d : PhantomData< a >
}
struct e< f > {
  g : Vec< f >
}
struct Ty(h, i, j);
struct h {
  k : Vec< Ty >
}
struct i {
  bounds : a
}
struct j {
  l : Box< Ty >
}
pub fn m() {
  e::< Ty >{g : Vec::new ()};
}
pub fn n(t : &mut c) {}
