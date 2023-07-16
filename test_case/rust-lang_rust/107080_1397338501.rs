plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
    |
985 |         if cfg!(release) {
    |                 ^^^^^^^
    |
    = note: `-D unexpected-cfgs` implied by `-D warnings`
error: could not compile `rustfmt-nightly` due to previous error
Build completed unsuccessfully in 0:03:56
