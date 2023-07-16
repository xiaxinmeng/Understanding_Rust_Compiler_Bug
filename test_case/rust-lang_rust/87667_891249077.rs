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
 Documenting alloc v0.0.0 (/checkout/library/alloc)
error: could not parse code block as Rust code
   --> library/alloc/src/vec/source_iter_marker.rs:85:5
    |
85  |   //! 