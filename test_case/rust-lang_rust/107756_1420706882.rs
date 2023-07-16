rust
const FOO: () = {
    foo(10);
};

const fn foo(x: usize) {
    if x != 0 {
        let mut array = [0_u8; usize::MAX / 4];
        foo(x - 1);
        array[array.len() - 1] = 42;
    }
}

fn main() {
    FOO;
}
