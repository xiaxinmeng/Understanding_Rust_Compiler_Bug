plain
   Compiling object v0.30.1
   Compiling miniz_oxide v0.6.2
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
   Compiling addr2line v0.19.0
error: specialization impl does not specialize any associated items
   |
   |
97 | impl crate::sealed::Sealed for OsString {}
   |
note: impl is a specialization of this impl
  --> library/std/src/sys/unix/path/posix_path.rs:49:1
   |
   |
49 | impl<P: AsRef<Path>> crate::sealed::Sealed for P {}

error: could not compile `std` (lib) due to previous error
Build completed unsuccessfully in 0:03:23
