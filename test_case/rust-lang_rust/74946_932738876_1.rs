console
$ cat a.rs 
fn main() {
    if std::env::var("a").is_ok() {
        println!("b");
    }
}
$ rustc -Zmir-opt-level=3 a.rs -C incremental=incremental
$ rustc -Zmir-opt-level=3 a.rs -C incremental=incremental
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_metadata/src/rmeta/decoder.rs:425:33
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (c02371c44 2021-10-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z mir-opt-level=3 -C incremental

query stack during panic:
#0 [optimized_mir] optimizing MIR for `main`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
