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
    Checking rustc_span v0.0.0 (/checkout/compiler/rustc_span)
error: variable does not need to be mutable
 --> compiler/rustc_span/src/symbol/tests.rs:7:9
  |
7 |     let mut i: Interner = Interner::default();
  |         |
  |         help: remove this `mut`
  |
  |
  = note: `-D unused-mut` implied by `-D warnings`
    Checking rustc_ast v0.0.0 (/checkout/compiler/rustc_ast)
    Checking rustc_target v0.0.0 (/checkout/compiler/rustc_target)
    Checking rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
    Checking rustc_parse_format v0.0.0 (/checkout/compiler/rustc_parse_format)
