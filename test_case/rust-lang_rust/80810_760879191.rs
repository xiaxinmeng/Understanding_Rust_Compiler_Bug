
$ rustc --version --verbose
rustc 1.51.0-nightly (455a0e1d9 2021-01-05)
binary: rustc
commit-hash: 455a0e1d91d3f56c4752a9f035e3622c614b7240
commit-date: 2021-01-05
host: s390x-unknown-linux-gnu
release: 1.51.0-nightly
$ cat demo.rs
trait T1: {}
trait T2: {
        type Foo: T1; 
}
trait T3<T>: {
        fn f(&self) -> T::Foo where T: T2; 
}
fn main() {}
$ rustc demo.rs && echo "no complaints"
no complaints
