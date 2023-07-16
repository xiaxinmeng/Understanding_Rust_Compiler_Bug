rust
#![feature(never_type)]
#![feature(exhaustive_patterns)]

fn foo() ->  Result<(), !> {
    return Ok(());
}

fn main() {
    match foo() {
        Ok(_) => println!("yay!")
    }
}
