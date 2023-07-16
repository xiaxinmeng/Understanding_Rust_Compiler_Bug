 rust
#![feature(never_type)]

fn main() {
    let result: Result<_, !> = Ok(1);
    match result {
//        ^^^^^^ pattern `Err(_)` not covered
        Ok(i) => println!("{}", i),
    }
}
