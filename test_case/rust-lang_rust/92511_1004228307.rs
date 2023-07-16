rust
#![feature(const_eval_limit)]
#![const_eval_limit = "10000000"]

const MB: usize = 1024 * 1024;
const L: usize = MB;
const DATA: [u8; L] = [3; L];
const TEST: [u8; L] = test(&DATA);

const fn test(data: &[u8]) -> [u8; L] {
    let mut ret = [0u8; L];
    let mut i = 0;
    let len = data.len();
    while i < len {
        ret[i] = data[i];
        i += 1;
    }
    ret
}

fn main() {
    println!("{}", TEST[1024]);
}
