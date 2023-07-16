bash
$ clang-11 -target riscv64-unknown-linux-gnu -B$RISCV --sysroot=$RISCV/sysroot hello.c -static -lpthread -o hello -v
Ubuntu clang version 11.0.0-2~ubuntu20.04.1
Target: riscv64-unknown-linux-gnu
Thread model: posix
InstalledDir: /usr/bin
Found candidate GCC installation: /home/addisoncrump/.local/riscv/lib/gcc/riscv64-unknown-elf/9.2.0
Found candidate GCC installation: /home/addisoncrump/.local/riscv/lib/gcc/riscv64-unknown-linux-gnu/9.2.0
Selected GCC installation: /home/addisoncrump/.local/riscv/lib/gcc/riscv64-unknown-linux-gnu/9.2.0
 "/usr/lib/llvm-11/bin/clang" -cc1 -triple riscv64-unknown-linux-gnu -emit-obj -mrelax-all -disable-free -disable-llvm-verifier -discard-value-names -main-file-name hello.c -static-define -mrelocation-model static -mframe-pointer=all -fmath-errno -fno-rounding-math -mconstructor-aliases -target-feature +m -target-feature +a -target-feature +f -target-feature +d -target-feature +c -target-feature +relax -target-feature -save-restore -target-abi lp64d -msmall-data-limit 8 -fno-split-dwarf-inlining -debugger-tuning=gdb -v -resource-dir /usr/lib/llvm-11/lib/clang/11.0.0 -isysroot /home/addisoncrump/.local/riscv/sysroot -internal-isystem /home/addisoncrump/.local/riscv/sysroot/usr/local/include -internal-isystem /usr/lib/llvm-11/lib/clang/11.0.0/include -internal-externc-isystem /home/addisoncrump/.local/riscv/sysroot/include -internal-externc-isystem /home/addisoncrump/.local/riscv/sysroot/usr/include -fdebug-compilation-dir /tmp/tmp.bgiqwBexGr -ferror-limit 19 -fno-signed-char -fgnuc-version=4.2.1 -fcolor-diagnostics -faddrsig -o /tmp/hello-c45a2f.o -x c hello.c
clang -cc1 version 11.0.0 based upon LLVM 11.0.0 default target x86_64-pc-linux-gnu
ignoring nonexistent directory "/home/addisoncrump/.local/riscv/sysroot/usr/local/include"
ignoring nonexistent directory "/home/addisoncrump/.local/riscv/sysroot/include"
#include "..." search starts here:
#include <...> search starts here:
 /usr/lib/llvm-11/lib/clang/11.0.0/include
 /home/addisoncrump/.local/riscv/sysroot/usr/include
End of search list.
 "/home/addisoncrump/.local/riscv/bin/riscv64-unknown-linux-gnu-ld" --sysroot=/home/addisoncrump/.local/riscv/sysroot -z relro --hash-style=gnu --build-id --eh-frame-hdr -m elf64lriscv -static -o hello /home/addisoncrump/.local/riscv/sysroot/usr/lib/crt1.o /home/addisoncrump/.local/riscv/lib/gcc/riscv64-unknown-linux-gnu/9.2.0/crti.o /home/addisoncrump/.local/riscv/lib/gcc/riscv64-unknown-linux-gnu/9.2.0/crtbeginT.o -L/home/addisoncrump/.local/riscv/lib/gcc/riscv64-unknown-linux-gnu/9.2.0 -L/home/addisoncrump/.local/riscv/lib/gcc/riscv64-unknown-linux-gnu/9.2.0/../../../../riscv64-unknown-linux-gnu/lib -L/home/addisoncrump/.local/riscv/sysroot/lib -L/home/addisoncrump/.local/riscv/sysroot/usr/lib /tmp/hello-c45a2f.o -lpthread --start-group -lgcc -lgcc_eh -lc --end-group /home/addisoncrump/.local/riscv/lib/gcc/riscv64-unknown-linux-gnu/9.2.0/crtend.o /home/addisoncrump/.local/riscv/lib/gcc/riscv64-unknown-linux-gnu/9.2.0/crtn.o
$ qemu-riscv64 hello
Hello, world!
