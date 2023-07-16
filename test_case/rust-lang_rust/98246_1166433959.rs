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
    Checking hashbrown v0.12.0
    Checking object v0.26.2
    Checking addr2line v0.16.0
error[E0425]: cannot find value `is_zero` in this scope
   --> library/std/src/sys/windows/fs.rs:562:9
    |
562 |         is_zero = |t: c::FILETIME| t.dwLowDateTime == 0 && t.dwHighDateTime == 0;


error[E0425]: cannot find value `is_zero` in this scope
   --> library/std/src/sys/windows/fs.rs:563:41
    |
563 |         if times.accessed.map_or(false, is_zero) || times.modified.map_or(false, is_zero) {


error[E0425]: cannot find value `is_zero` in this scope
   --> library/std/src/sys/windows/fs.rs:563:82
    |
563 |         if times.accessed.map_or(false, is_zero) || times.modified.map_or(false, is_zero) {

For more information about this error, try `rustc --explain E0425`.
error: could not compile `std` due to 3 previous errors
Build completed unsuccessfully in 0:00:17
