 rust
macro_rules! foo { //~ NODE in expansion of
     () => {
          1.fake(); //~ ERROR does not implement any method
     }
}

fn main() {
     foo!(); //~ NOTE expansion site
}
