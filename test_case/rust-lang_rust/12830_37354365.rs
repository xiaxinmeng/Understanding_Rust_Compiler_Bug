 rust
let a;
let b = Some(1);
match b {
  Some(c) if {
    a = c+1;
    a == 2
  } => println!("{}", a),
  Some(_) => println!("Some"),
  None => println!("None")
}
