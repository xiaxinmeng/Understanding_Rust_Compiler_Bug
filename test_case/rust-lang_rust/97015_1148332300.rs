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
    Checking object v0.26.2
    Checking addr2line v0.16.0
error[E0596]: cannot borrow `cursor` as mutable, as it is not declared as mutable
   --> library/std/src/sys/windows/handle.rs:114:44
    |
112 |     pub fn read_buf(&self, cursor: BorrowCursor<'_, '_>) -> io::Result<()> {
    |                            ------ help: consider changing this to be mutable: `mut cursor`
113 |         let res =
114 |             unsafe { self.synchronous_read(cursor.as_mut().as_mut_ptr(), cursor.capacity(), None) };
    |                                            ^^^^^^^^^^^^^^^ cannot borrow as mutable

error[E0596]: cannot borrow `cursor` as mutable, as it is not declared as mutable
   --> library/std/src/sys/windows/handle.rs:120:21
    |
112 |     pub fn read_buf(&self, cursor: BorrowCursor<'_, '_>) -> io::Result<()> {
    |                            ------ help: consider changing this to be mutable: `mut cursor`
...
120 |                     cursor.advance(read as usize);
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot borrow as mutable
For more information about this error, try `rustc --explain E0596`.
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:00:18
