rust
#[derive(Debug)]
struct V(u32);

fn main() {
    let a = Some(456);
    let v = V(123);
    
    a.map(|x| {
        let _ = x + v.0;
        drop(v);
    });
    
    println!("v: {:?}", v);
}
