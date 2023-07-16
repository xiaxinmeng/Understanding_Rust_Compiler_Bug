
$ cargo new --bin lldb_test
$ cd lldb_test; cargo build
$ rust-lldb target/debug/lldb_test
(lldb) command script import "/Users/zmitchell/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/etc/lldb_lookup.py"
(lldb) command source -s 0 '/Users/zmitchell/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/etc/lldb_commands'
Executing commands in '/Users/zmitchell/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/etc/lldb_commands'.
(lldb) type synthetic add -l lldb_lookup.synthetic_lookup -x ".*" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(alloc::([a-z_]+::)+)String$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^&(mut )?str$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^&(mut )?\\[.+\\]$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(std::ffi::([a-z_]+::)+)OsString$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(alloc::([a-z_]+::)+)Vec<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(alloc::([a-z_]+::)+)VecDeque<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(alloc::([a-z_]+::)+)BTreeSet<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(alloc::([a-z_]+::)+)BTreeMap<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(std::collections::([a-z_]+::)+)HashMap<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(std::collections::([a-z_]+::)+)HashSet<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(alloc::([a-z_]+::)+)Rc<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(alloc::([a-z_]+::)+)Arc<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(core::([a-z_]+::)+)Cell<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(core::([a-z_]+::)+)Ref<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(core::([a-z_]+::)+)RefMut<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^(core::([a-z_]+::)+)RefCell<.+>$" --category Rust
(lldb) type summary add -F lldb_lookup.summary_lookup  -e -x -h "^core::num::([a-z_]+::)*NonZero.+$" --category Rust
(lldb) type category enable Rust
(lldb) target create "target/debug/lldb_test"
Current executable set to '/Users/zmitchell/code/lldb_test/target/debug/lldb_test' (arm64).
