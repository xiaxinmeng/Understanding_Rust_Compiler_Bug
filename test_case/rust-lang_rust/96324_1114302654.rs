plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking hashbrown v0.12.0
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking addr2line v0.16.0
 Documenting std v0.0.0 (/checkout/library/std)
error: unresolved link to `TcpStream::set_quickack`
  --> library/std/src/os/linux/net.rs:39:55
   |
39 |     /// For more information about this option, see [`TcpStream::set_quickack`].
   |                                                       ^^^^^^^^^^^^^^^^^^^^^^^ no item named `TcpStream` in scope
   |
   = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
error: could not parse code block as Rust code
  --> library/std/src/os/linux/net.rs:26:9
   |
26 |       /// 