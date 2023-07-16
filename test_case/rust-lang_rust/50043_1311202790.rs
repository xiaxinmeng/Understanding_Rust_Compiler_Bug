rust
fn fold<F:FnMut(usize, u8) -> usize>(recurse: bool, mut init: usize, mut f: F) -> usize {
    if recurse {
        for i in 0..2 {
            init = f(init, i);
            init = fold(false, init, &mut f)
        }
        init
    } else {
        init
    }
}

fn main() {
    fold(true, 0, |a, b| a + (b as usize));
}
