
gcc -E -MMD -MP -MT i686-pc-mingw32/rt/arch/i386/morestack.o -MF i686-pc-mingw32/rt/arch/i386/morestack.d /home/cmr/hacking/rust/src/rt/arch/i386/morestack.S | /home/cmr/hacking/rust/build/x86_64-unknown-linux-gnu/llvm/Release+Asserts/bin/llvm-mc -assemble -filetype=obj -triple=i686-pc-mingw32 -o=i686-pc-mingw32/rt/arch/i386/morestack.o
/home/cmr/hacking/rust/src/rt/arch/i386/morestack.S:3:19: error: unexpected token in directive
.section .note.GNU-stack, "", @progbits
                  ^
/home/cmr/hacking/rust/src/rt/arch/i386/morestack.S:90:2: error: unknown directive
 .hidden __morestack
 ^
/home/cmr/hacking/rust/src/rt/arch/i386/morestack.S:98:8: error: expected absolute expression
 .type __morestack,@function
       ^
/home/cmr/hacking/rust/mk/rt.mk:93: recipe for target 'i686-pc-mingw32/rt/arch/i386/morestack.o' failed
make: *** [i686-pc-mingw32/rt/arch/i386/morestack.o] Error 1
