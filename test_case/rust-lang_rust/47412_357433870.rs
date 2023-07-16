rust
fn main() {
    let x = Box::new(22);
    drop(x);
    match x { 
        _ => { }
    }
}
