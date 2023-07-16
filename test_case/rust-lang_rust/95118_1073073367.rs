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
    Checking miniz_oxide v0.4.0
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking hashbrown v0.12.0
    Checking addr2line v0.16.0
error: associated function has missing stability attribute
   --> library/std/src/os/windows/io/handle.rs:334:5
    |
334 |     fn as_handle(&self) -> BorrowedHandle<'_>;

error: associated function has missing stability attribute
error: associated function has missing stability attribute
   --> library/std/src/os/windows/io/socket.rs:215:5
    |
215 |     fn as_socket(&self) -> BorrowedSocket<'_>;

error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:00:21
