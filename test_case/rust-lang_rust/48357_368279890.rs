rust
#![feature(nll)]

fn bar(a: bool) {
    let mut first = 5;
    let temp;
    
    if a {
        temp = &mut first;
    } else {
        temp = &mut first;
    }
    
    *temp = 20;
}

fn main() {
    bar(true)
}
