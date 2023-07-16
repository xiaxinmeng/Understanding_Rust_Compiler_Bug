rust
fn foo() {  // WARN: unused
  bar();
}

fn bar() {  // WARN: unused
  println!("hello");
}

fn main() {
}
