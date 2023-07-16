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
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking hashbrown v0.11.0
    Checking object v0.22.0
    Checking addr2line v0.14.0
error[E0433]: failed to resolve: use of undeclared type `BufferMode`
    |
298 |     BufferMode::Line
298 |     BufferMode::Line
    |     ^^^^^^^^^^ use of undeclared type `BufferMode`

error[E0412]: cannot find type `BufferMode` in this scope
    |
    |
297 | pub fn default_stdout_buffer_mode() -> BufferMode {
    |
help: consider importing this enum
    |
3   | use crate::io::BufferMode;
