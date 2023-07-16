
➜  cat a.rs
fn main() {
    println!("{}", -9.223372036854776e18f64 as i64);
    println!("{}", -9.223372036854776e18f64 as i64 == 0);
}
➜  rustc a.rs
➜  ./a
-9223372036854775808
false
➜  rustc -Vv
rustc 1.1.0-nightly (e5394240a 2015-05-14) (built 2015-05-13)
binary: rustc
commit-hash: e5394240a295650b567aa406b4a0e1e3a6749a5f
commit-date: 2015-05-14
build-date: 2015-05-13
host: x86_64-apple-darwin
release: 1.1.0-nightly
