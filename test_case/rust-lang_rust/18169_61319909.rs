 rust
let x = Value(5);
let y = Missing;

match x {
    Value(n) => println!("x is {}", n),
    Missing  => println!("x is missing!"),
}
