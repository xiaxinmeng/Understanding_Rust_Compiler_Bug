rust
#![feature(nll)]

fn foo<'a>(x: &'a mut u32) -> u32 {
    let mut x = 22;
    let y = &x;
    if false {
        return x;
    }

    x += 1;
    println!("{}", y);
    return 0;
}

fn main() { }
