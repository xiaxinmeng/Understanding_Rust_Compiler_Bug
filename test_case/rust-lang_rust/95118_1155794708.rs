plain

error: associated function has missing stability attribute
   --> library/std/src/os/fd/owned.rs:103:5
    |
103 | /     pub fn try_clone(&self) -> crate::io::Result<Self> {
104 | |         Err(crate::io::const_io_error!(
105 | |             crate::io::ErrorKind::Unsupported,
106 | |             "operation not supported on WASI yet",
108 | |     }
    | |_____^

[RUSTC-TIMING] std test:false 2.602
