rust
use jsondocck::{TCrate, main_test}

fn main() {main_test(test);}

fn test(t: TCrate) {
   let foo = t.get_root::<Function>("foo");
   assert_eq!(foo.decl.output, None);
}
