rust
fn main() {
    let mut x = 5;
    { let y = &mut x;
      *y += 1;
    }
    take(&x);
}
fn take(v: &i32){
    println!("v is {}", v);
}
