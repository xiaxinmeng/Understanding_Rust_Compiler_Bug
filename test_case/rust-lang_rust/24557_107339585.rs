 rust
fn main(){
    let x = 1e-308 / 1e15;
    println!("{}", x == 0.0);
    println!("{:e}", x);
}
