 rust
fn main() {
    let head = [(1,2), (3,4)];
    let tail = [(4,5), (6,7)];

    // missing .cloned()
    let _:Vec<_> = head.iter().chain(tail.iter().map(|x|{(x.0+1,x.1+1)})).collect(); 
}
