rust
#![feature(nll)]

struct FancyNum {
    num: u8,
}

fn main() {
    let mut fancy_num = FancyNum { num: 5 };
    let fancy_ref = &fancy_num;

    let mut x = move || {
        fancy_num.num += 1;
    };

    x();
    
    drop(fancy_ref);
}
