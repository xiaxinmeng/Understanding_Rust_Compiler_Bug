
objdump -R target/debug/libfringetest.so | grep fringe.*stack
00000000003171f0 R_X86_64_JUMP_SLOT  _ZN62_$LT$fringe..stack..os..OsStack$u20$as$u20$core..ops..Drop$GT$4drop17h9b06805ccb79bdb0E@@Base
0000000000317aa0 R_X86_64_JUMP_SLOT  _ZN6fringe5stack2os3sys9page_size17h61688e1bae0d3c5aE@@Base
