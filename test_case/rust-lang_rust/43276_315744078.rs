rust
fn foo<'a>(x:  fn(&u8, &u8), mut y: Vec<&u8>, z: &'a u8) {
  y.push(z); 
}

fn main() { }
