 rust
fn main() {
    let i = 1u32 << 32u32;
    println(fmt!("shift by literal 32, result: %d", i as int));
    let mut s = 0u32;
    while s < 40 {
        let i = 1u32 << s;
        println(fmt!("shift by variable %d, result: %d", s as int, i as int));
        s = s + 1;
    }
}
