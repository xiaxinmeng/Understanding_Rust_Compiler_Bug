
objdump -r target/debug/deps/fringetest.0.o | grep _ZN6fringe4arch3imp4swap10trampoline17he5525edeeec5683eE
0000000000000240 R_X86_64_PC32     _ZN6fringe4arch3imp4swap10trampoline17he5525edeeec5683eE-0x0000000000000004
00000000000005b7 R_X86_64_PC32     _ZN6fringe4arch3imp4swap10trampoline17he5525edeeec5683eE-0x0000000000000004
00000000000001ed R_X86_64_PC32     _ZN6fringe4arch3imp4swap10trampoline17he5525edeeec5683eE-0x0000000000000004
00000000000001ff R_X86_64_PC32     _ZN6fringe4arch3imp4swap10trampoline17he5525edeeec5683eE-0x0000000000000004
00000000000002d6 R_X86_64_PC32     _ZN6fringe4arch3imp4swap10trampoline17he5525edeeec5683eE-0x0000000000000004

nm -s target/debug/deps/fringetest.0.o | grep _ZN6fringe4arch3imp4swap10trampoline17he5525edeeec5683eE
                 U _ZN6fringe4arch3imp4swap10trampoline17he5525edeeec5683eE
