 rust
  for x in range(0i, 101i) {

    match (x % 3, x % 5) {
      (0, 0) => println!("fizzbuzz"),
      (0, _) => println!("fizz"),
      (_, 0) => println!("buzz"),
           _ => println!("{}", x),
    }

  }
