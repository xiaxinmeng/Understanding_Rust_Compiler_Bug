rust
const EXAMPLE: &[u8] = include_bytes!("/proc/cpuinfo");
const EX_LEN: usize = EXAMPLE.len();
static BUF: [u8; EX_LEN] = {
    let mut arr = [0u8; EX_LEN];
    let mut idx = 0;
    while idx < EX_LEN {
        arr[idx] = EXAMPLE[idx];
        idx += 1;
    }
    arr
};

fn main() {
    println!("{}", BUF.len());
}
