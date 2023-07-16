
$ cat ./hygienic_macro_test.rs
use std;

macro_rules! refer_to_x (
  ( $i:ident ) => (
    if true {
      let mut x = 2;
      x = x - 1;
      $i = $i - x;
    }
  )
)

fn foo() -> (int, int) {
  let mut x = 10;
  let mut y = 20;

  refer_to_x!(x);
  refer_to_x!(y);

  return (x, y);
}


#[test]
fn test_foo() {
  assert foo() == (9, 19);
}
