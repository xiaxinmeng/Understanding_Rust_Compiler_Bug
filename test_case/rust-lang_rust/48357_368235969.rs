rust
#![feature(nll)]

fn bar(a: u8) {
    let mut first = 5;
    let mut second = 6;
    let temp;
    
    match a {
        10 => {
            temp = &mut first;
        },
        20 => {
            temp = &mut second;
        }
        _ => panic!()
    }
    
    println!("Variables: {:?} {:?}", first, second);
    *temp = 7;
}

fn main() {}
