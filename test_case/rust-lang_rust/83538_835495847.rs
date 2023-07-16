rust
struct First {
  b : Vec< First >
} pub struct Second {
  d : Vec< First >
} struct Third< f > {
  g : Vec< f >
} enum Ty { j(Fourth, Fifth, Sixth) } struct Fourth {
  o : Vec< Ty >
} struct Fifth {
  bounds : First
} struct Sixth {
  p : Box< Ty >
} pub fn q() {
  Third::< Ty >{g : Vec::new ()};
}
pub fn s(t : &mut Second) {}
