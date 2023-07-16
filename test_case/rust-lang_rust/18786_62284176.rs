 rust
fn main() { 
  for d in range(0u, 1) { 
    let u: uint = (d + 1) % 2;
    let mut x = [0i,..2];
    let mut mask = false;
    while x[u] < 2 { 
      mask = x[1] > 0;
      x[u] += 1;
    } 
    println!("This should be true: {}" , mask);
    if !mask { 
      panic!("Failed!");
    } 
  } 
}
