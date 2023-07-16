
[00:53:34] error: variable does not need to be mutable
[00:53:34]     --> src/tools/cargo/src/cargo/core/resolver/mod.rs:1112:17
[00:53:34]      |
[00:53:34] 1112 |             let mut set = self.resolve_features.entry(pkgid.clone())
[00:53:34]      |                 ^^^^^^^
[00:53:34]      |
[00:53:34] 
[00:53:34] error: variable does not need to be mutable
[00:53:34]    --> src/tools/cargo/src/cargo/util/config.rs:484:13
[00:53:34]     |
[00:53:34] 484 |         let mut cfg = match *cfg {
[00:53:34]     |             ^^^^^^^
[00:53:34] 
[00:53:34] error: variable does not need to be mutable
[00:53:34]   --> src/tools/cargo/src/cargo/util/read2.rs:15:18
[00:53:34]    |
[00:53:34] 15 |                  mut data: &mut FnMut(bool, &mut Vec<u8>, bool)) -> io::Result<()> {
[00:53:34]    |                  ^^^^^^^^
[00:53:34] 
[00:53:34] error: aborting due to 3 previous errors
[00:53:34] 
[00:53:34] error: Could not compile `cargo`.
