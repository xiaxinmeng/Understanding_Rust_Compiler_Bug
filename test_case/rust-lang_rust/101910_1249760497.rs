plain
   Doc-tests rustc_macros
warning: unexpected `cfg` condition name
 --> /checkout/compiler/rustc_macros/src/lib.rs:2:13
  |
2 | #![cfg_attr(bootstrap, feature(let_else))]
  |
  = note: `#[warn(unexpected_cfgs)]` on by default

warning: 1 warning emitted
---
[TIMING] doc::Standalone { compiler: Compiler { stage: 1, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu } -- 0.521
Documenting book redirect pages (x86_64-unknown-linux-gnu)
[TIMING] doc::TheBook { compiler: Compiler { stage: 1, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu } -- 1.332
Documenting stage1 std (x86_64-unknown-linux-gnu)
Documenting stage1 std (x86_64-unknown-linux-gnu) in HTML format
thread 'main' panicked at 'fs::remove_dir_all(dir) failed with No such file or directory (os error 2)', lib.rs:1566:9
Build completed unsuccessfully in 0:20:06
