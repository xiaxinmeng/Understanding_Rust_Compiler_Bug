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
    Checking object v0.26.2
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking miniz_oxide v0.4.0
    Checking addr2line v0.16.0
error[E0433]: failed to resolve: use of undeclared type `Handle`
   --> library/std/src/os/windows/io/handle.rs:169:32
    |
169 |             return unsafe { Ok(Handle::from_raw_handle(handle)) };
...
311 | pub trait AsHandle {
    | ------------------ similarly named trait `AsHandle` defined here
    |
    |
help: a trait with a similar name exists
    |
169 |             return unsafe { Ok(AsHandle::from_raw_handle(handle)) };
help: consider importing this struct
    |
5   | use crate::sys::handle::Handle;
    |
