
# with BAD rustc
$ nm -C ./target/debug/deps/app-*.0.o
0000000000000000 t rust_begin_unwind
0000000000000000 N __rustc_debug_gdb_scripts_section__
0000000000000000 T _start
0000000000000000 r str.0
0000000000000000 r str.1
0000000000000000 d app::_start::_MSG_FILE_LINE::hb927abaa970af5cf
                 U core::panicking::panic::h194ce5d68a8f28a1
0000000000000000 t drop::hc6005949ff734e37
