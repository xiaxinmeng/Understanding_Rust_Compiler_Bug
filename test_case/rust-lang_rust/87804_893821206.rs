plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking rustc_index v0.0.0 (/checkout/compiler/rustc_index)
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
    Checking rustc_arena v0.0.0 (/checkout/compiler/rustc_arena)
    Checking rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
error[E0658]: box expression syntax is experimental; you can call `Box::new` instead
  --> compiler/rustc_arena/src/tests.rs:96:25
   |
96 |         let _: Box<_> = box (Point { x: 1, y: 2, z: 3 });
   |
   = note: see issue #49733 <https://github.com/rust-lang/rust/issues/49733> for more information
   = help: add `#![feature(box_syntax)]` to the crate attributes to enable


error[E0658]: box expression syntax is experimental; you can call `Box::new` instead
   --> compiler/rustc_arena/src/tests.rs:230:13
    |
230 |             box (Noncopy { string: "hello world".to_string(), array: vec![1, 2, 3, 4, 5] });
    |
    = note: see issue #49733 <https://github.com/rust-lang/rust/issues/49733> for more information
    = help: add `#![feature(box_syntax)]` to the crate attributes to enable

