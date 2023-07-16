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
    Checking miniz_oxide v0.4.0
    Checking hashbrown v0.11.0
    Checking object v0.22.0
    Checking addr2line v0.14.0
error[E0658]: box expression syntax is experimental; you can call `Box::new` instead
  --> library/std/src/sys/windows/thread.rs:23:31
   |
23 |         let p = Box::into_raw(box p);
   |
   = note: see issue #49733 <https://github.com/rust-lang/rust/issues/49733> for more information
   = help: add `#![feature(box_syntax)]` to the crate attributes to enable

