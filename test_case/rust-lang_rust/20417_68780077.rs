 rust
fn main() {
    let d: Result<Dog, AnimalError> = Animal::new(&123);
    let d1 = d.unwrap();
    println!("{}",d1);
}
