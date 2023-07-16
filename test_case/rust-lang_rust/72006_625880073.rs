
$ touch foo.rs 
$ rustc +stable foo.rs -C incremental=inc -O --crate-type rlib -C linker-plugin-lto
$ rustc +stable foo.rs -C incremental=inc -O --crate-type rlib -C linker-plugin-lto
thread 'rustc' panicked at 'failed to open bitcode file `/home/alex/code/rust3/lol/inc/foo-2274d9nt8ooh9/s-fn8iukgg9j-10g2qx1-working/2yck3ogdpf9mjtsp.pre-lto.bc`: No such file or directory (os error 2)', <::std::macros::panic macros>:5:6
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.43.1 (8d69840ab 2020-05-04) running on x86_64-unknown-linux-gnu

note: compiler flags: -C incremental -C linker-plugin-lto -C save-temps --crate-type rlib
