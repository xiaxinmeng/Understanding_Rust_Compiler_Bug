
  Compiling std v0.0.0 (file:///C:/projects/rust/src/libstd)
error[E0433]: failed to resolve. Use of undeclared type or module `sys`
   --> src\libstd\sys\windows\os.rs:327:71
    |
327 |         assert!(!::io::Error::from_raw_os_error(STATUS_UNSUCCESSFUL | sys::c::FACILITY_NT_BIT as _)
    |                                                                       ^^^^^^^^^^^^^^^^^^^^^^^ Use of undeclared type or module `sys`
