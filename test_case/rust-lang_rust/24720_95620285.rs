 rust
#![feature(step_by)]
fn main () {
    let mut range = (0..5).step_by(2);

    loop {
        println!("{:?}", range.size_hint());
        match range.next() {
            Some(x) => {
                println!("{}", x);
            },
            None => { break }
        }
    }
}
