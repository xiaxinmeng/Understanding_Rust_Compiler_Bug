plain
Z:\test> rustc --version --verbose
rustc 1.68.0 (2c8cc3432 2023-03-06)
binary: rustc
commit-hash: 2c8cc343237b8f7d5a3c3703e3a87f2eb2c54a74
commit-date: 2023-03-06
host: x86_64-pc-windows-msvc
release: 1.68.0
LLVM version: 15.0.6
Z:\test> cat whitespace.rs
fn main() {
    "\
£";
}
Z:\test> rustc whitespace.rs
warning: non-ASCII whitespace symbol '\u{a3}' is not skipped
 --> whitespace.rs:2:6
  |
2 |       "\
  |  ______^
3 | | £";
  | | ^ non-ASCII whitespace symbol '\u{a3}' is not skipped
  | |_|
  |

warning: 1 warning emitted

