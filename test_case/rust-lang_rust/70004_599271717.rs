rust
loop {
  let x = ();
  println!("{:?}", &x as *const _); // prints the same thing each round
}
