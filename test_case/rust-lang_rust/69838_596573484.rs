rust
// exec-env:RUSTC_LOG=rustc_expand=debug
// rustc-env:RUST_BACKTRACE=1
// rustc-env:RUSTC_LOG=rustc_expand=debug
// compile-flags:-Ztreat-err-as-bug
// pp-exact

// The next line should not be expanded

#[path = "issue-12590-b.rs"]
mod issue_12590_b;

fn main() { }
