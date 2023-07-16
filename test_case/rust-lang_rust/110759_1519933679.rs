rust
fn main() {
    let mut a = 1;

    let n = {
        let c = &mut a;
        let n = &*c; // you are still holding a ref to c
        drop(c); // can't drop here, we are borrowed
        n
    };  // c is not dropped here, it was moved into `drop`
    
    println!("{}", n);  // use-after-free: n points to a dropped value
}
