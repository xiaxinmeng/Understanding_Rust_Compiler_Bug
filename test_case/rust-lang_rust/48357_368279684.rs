rust
#![feature(nll)]

fn bar(a: usize) {
    let mut first = 5;
    let temp;
    
    match a {
        10 => {
            temp = &mut first;
        },
        20 => {
            temp = &mut first;
        }
        _ => panic!()
    }
    
    println!("Temp: {:?}", temp);
}

fn main() {
    bar(10)
}
