
objdump -T ./target/debug/libtest.so
 00000000000039c0 g    DF .text 00000000000007c4  Base        init

objdump -t ./target/debug/libtest.so | grep init
00000000000039c0 g     F .text  00000000000007c4              init
