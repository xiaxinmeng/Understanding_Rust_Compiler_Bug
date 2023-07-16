rust
const fn double(v: usize) -> usize {
    v * 2
}

pub const FOO: usize = double(usize::MAX);
