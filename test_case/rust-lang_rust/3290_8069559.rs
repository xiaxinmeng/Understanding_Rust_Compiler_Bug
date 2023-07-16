
fn main() {
   let mut x = ~3;
   x = x;
   assert *x == 3;
}
