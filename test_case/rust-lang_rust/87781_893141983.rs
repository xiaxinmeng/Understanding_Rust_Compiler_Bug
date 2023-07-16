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
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
error[E0658]: box expression syntax is experimental; you can call `Box::new` instead
   |
   |
47 |           static DUMMY_TYS: &'static TyS<'static> = Box::leak(box TyS::make_for_test(
48 | |             ty::Bool,
49 | |             TypeFlags::empty(),
49 | |             TypeFlags::empty(),
50 | |             DebruijnIndex::from_usize(0),
   | |_________^
   |
   = note: see issue #49733 <https://github.com/rust-lang/rust/issues/49733> for more information
   = help: add `#![feature(box_syntax)]` to the crate attributes to enable
