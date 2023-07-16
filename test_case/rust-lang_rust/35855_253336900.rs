
$ make clean
rm -rf incr/*
cargo clean

$ make
RUSTFLAGS="-Z incremental=incr" \
    cargo rustc -p syntex_syntax --  -Z incremental-info
   Compiling log v0.3.6
   Compiling winapi-build v0.1.1
   Compiling libc v0.2.15
   Compiling bitflags v0.5.0
   Compiling rustc-serialize v0.3.19
   Compiling winapi v0.2.8
   Compiling unicode-xid v0.0.3
   Compiling kernel32-sys v0.2.2
   Compiling term v0.4.4
   Compiling syntex_pos v0.42.0
   Compiling syntex_errors v0.42.0
   Compiling syntex_syntax v0.42.0
incremental: re-using 0 out of 50 modules

$ make touch
cargo clean -p syntex_syntax

$ make
RUSTFLAGS="-Z incremental=incr" \
    cargo rustc -p syntex_syntax --  -Z incremental-info
   Compiling syntex_syntax v0.42.0
incr. comp. session directory: 103 files hard-linked
incr. comp. session directory: 0 files copied
incremental: re-using 50 out of 50 modules
