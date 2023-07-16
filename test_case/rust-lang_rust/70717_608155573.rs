rust
fn main(){
    let a = (String::new(),0);
    let ref _b = {a.0};
    // Can't print a partially moved value
    println!("{:?}",a);
}
