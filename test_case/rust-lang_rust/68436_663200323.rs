rust
fn foo<const C: u8>() {
  if C > 0 {
    println!("Foo gives {}", 25 / C);
  }
  else {
    println!("Foo gives 0");
  }
}

fn bar() {
  foo::<0>();
}
