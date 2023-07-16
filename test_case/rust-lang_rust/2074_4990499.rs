
fn main(args: [str]) {
    let one = fn@() -> uint {
      fn f() { fail; }
      ret 42u;
    };
    let two = fn@() -> uint {
      fn f() { fail; }
      ret 42u;
    };
    one(); two();
}
