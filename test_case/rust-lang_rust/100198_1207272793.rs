
Z:\test> rustc +nightly-gnu --version --verbose
rustc 1.64.0-nightly (affe0d3a0 2022-08-05)
binary: rustc
commit-hash: affe0d3a00e92fa7885e3f5d2c5073fde432d154
commit-date: 2022-08-05
host: x86_64-pc-windows-gnu
release: 1.64.0-nightly
LLVM version: 14.0.6
Z:\test> cat main.rs
fn main() {
    let _ =
    "0"
    .lines()
    .map(|r| r
             .split_whitespace()
             .map(|s| s.parse::<u32>().unwrap())
             .collect::<Vec<_>>())
    .reduce(|x, y| y.iter().zip(x)
                   .map(|(a, b)| a + b)
                   .collect());
}

Z:\test> rustc +nightly-gnu main.rs
Z:\test>
