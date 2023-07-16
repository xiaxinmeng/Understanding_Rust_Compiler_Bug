 rust
fn main() {
    let v = vec![1, 2, 3];
    let v2 = v;
    {   let v3 = v2; drop(v3); } // explicit drop for clarity, 
        // but even without it v3 would be dropped at end of this inner scope
    println!("{:?}", v);
}
