rust
fn main() {
    let mut a = String::from("fa");

    let n = {
        let c = &mut a;
        let n = &*c; // shadows c
        drop(n); // unshadows c
        c
    };
    println!("{}", n);
}
