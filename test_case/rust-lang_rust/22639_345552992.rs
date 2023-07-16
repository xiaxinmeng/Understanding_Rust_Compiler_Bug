rust
use std::num::Saturating;

fn main() {
    let s = "-999";
    let n = match s.parse::<Saturating<i8>>()?.0 {
        std::i8::MIN ... -11 => { println!("saturating"); -10 }
        11 ... std::i8::MAX => { println!("saturating"); 10 }
        n => { println!("in range"); n }
    };
    println!("n={}", n);
}
