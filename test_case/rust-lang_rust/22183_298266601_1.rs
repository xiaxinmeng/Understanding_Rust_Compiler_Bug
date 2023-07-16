
$ nm -C libtest.a | head -n 15

test.0.o:

std-35ad9950c7e5074b.0.o:
                 U abort
                 U accept
                 U backtrace_create_state
                 U backtrace_pcinfo
                 U backtrace_syminfo
                 U bind
0000000000000000 r byte_str.3f
0000000000000000 r byte_str.3r
0000000000000000 r byte_str.4B
0000000000000000 r byte_str.4D
0000000000000000 r byte_str.4f

$ nm -C libtest-bin.a | head -n 15

test.0.o:
0000000000000000 T main
                 U std::rt::lang_start::h8126425a59d3b4a0
0000000000000000 t test::main::hf4d700377696cbb0

std-35ad9950c7e5074b.0.o:
                 U abort
                 U accept
                 U backtrace_create_state
                 U backtrace_pcinfo
                 U backtrace_syminfo
                 U bind
0000000000000000 r byte_str.3f
0000000000000000 r byte_str.3r
