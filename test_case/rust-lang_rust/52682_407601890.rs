 console
$ cargo rustc --release -- --emit=obj

$ nm -C target/release/deps/used-81376e9de841c190.o
0000000000000000 T main
                 U std::io::stdio::_print
0000000000000000 T std::rt::lang_start
0000000000000000 t std::rt::lang_start::{{closure}}
                 U std::rt::lang_start_internal
0000000000000000 t core::ops::function::FnOnce::call_once
0000000000000000 t core::ptr::drop_in_place
0000000000000000 t used::bar
0000000000000000 t used::main
0000000000000000 d used::DUMMY
