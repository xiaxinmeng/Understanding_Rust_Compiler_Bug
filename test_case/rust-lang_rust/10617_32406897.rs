 rust
fn foo() {
  let mut i = 0;
    do task::spawn {
      i += 1; // fails here
      some_func(i);
      println!("spawned {}", i)
    }
    i += 1;
    println!("original {}", i)
}
