rust
fn main() {
    fn dummy() { }
  
    let dummy1 = dummy as fn();
    let dummy2: fn() = dummy;

    dummy1 == dummy1; // OK
    dummy2 == dummy2; // OK
    dummy  == dummy;  // Err
}
