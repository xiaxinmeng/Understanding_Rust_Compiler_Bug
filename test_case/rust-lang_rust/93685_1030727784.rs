plain
   Compiling toml v0.5.7
error: unused import: `OpenOptions`
   --> src/bootstrap/lib.rs:109:27
    |
109 | use std::fs::{self, File, OpenOptions};
    |
    |
    = note: `-D unused-imports` implied by `-D warnings`

error: unused imports: `SeekFrom`, `Seek`, `Write`
   --> src/bootstrap/lib.rs:110:21
    |
110 | use std::io::{Read, Seek, SeekFrom, Write};

error: unused import: `Read`
   --> src/bootstrap/lib.rs:110:15
    |
    |
110 | use std::io::{Read, Seek, SeekFrom, Write};

error: could not compile `bootstrap` due to 3 previous errors
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
Build completed unsuccessfully in 0:01:09
