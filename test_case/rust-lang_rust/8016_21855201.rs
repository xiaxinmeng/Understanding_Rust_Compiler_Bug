
macro_rules! ok (($T:ident) => ($T))
macro_rules! bad (($T:ty)   => ($T))
//ok!(int);  // compiles
bad!(int); // crashes
fn main() { }
