rust
fn indirect_get_slice() -> &'static [usize] {
    &[]
}

#[inline(always)]
fn get_slice() -> &'static [usize] {
    let ret = indirect_get_slice();
    ret
}

fn main() {
    let output = get_slice().len();
    println!("{}", output);
}
