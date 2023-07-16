
enum X { X }
fn x() -> X { unimplemented!() }
match x() {
    X => println!("yo"),
    _ => println!("sup")
}
