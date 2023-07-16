 Rust
fn id<T>(t: T) -> T { t }
fn main() {
    let x = id(&2); //~ ERROR borrowed value does not live long enough
    println!("{}", x);    
    let y = &2;
    println!("{}", y); // no error
}
