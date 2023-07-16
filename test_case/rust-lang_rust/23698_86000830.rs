 rust
fn main() {
    let mut a = Box::new(1);
    match a {
        mut b => {
            a = Box::new(2);
            println!("{:p} {:p}", &mut a, &mut b);
        }
    }
}
