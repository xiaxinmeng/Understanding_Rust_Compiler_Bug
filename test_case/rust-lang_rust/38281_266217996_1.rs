
# with GOOD rustc
$ cargo rustc -- --emit=obj

$ nm -C ./target/debug/deps/app-*.o
0000000000000000 T rust_begin_unwind
0000000000000000 N __rustc_debug_gdb_scripts_section__
0000000000000000 T _start
0000000000000000 r str.0
0000000000000000 r str.1
0000000000000000 d app::_start::_MSG_FILE_LINE::h0939c9c7dafe8539
                 U core::panicking::panic::h194ce5d68a8f28a1
0000000000000000 t drop::he2bf721309cf573a
