
/tmp # echo "int main() {}" > main.cpp

/tmp # clang++ -v main.cpp
Vanilla Linux clang version 8.0.1 (tags/RELEASE_801/final) (based on LLVM 8.0.1)
Target: x86_64-unknown-linux-musl
Thread model: posix
InstalledDir: /bin
 "/bin/clang-8" -cc1 -triple x86_64-unknown-linux-musl -emit-obj -mrelax-all -disable-free -disable-llvm-verifier -discard-value-names -main-file-name main.cpp -mrelocation-model pic -pic-level 2 -pic-is-pie -mthread-model posix -mdisable-fp-elim -fmath-errno -masm-verbose -mconstructor-aliases -munwind-tables -fuse-init-array -target-cpu x86-64 -dwarf-column-info -debugger-tuning=gdb -v -resource-dir /lib/clang/8.0.1 -internal-isystem /bin/../include/c++/v1 -internal-isystem /usr/local/include -internal-externc-isystem /include -internal-externc-isystem /usr/include -internal-isystem /lib/clang/8.0.1/include -fdeprecated-macro -fdebug-compilation-dir /tmp -ferror-limit 19 -fmessage-length 151 -fobjc-runtime=gcc -fcxx-exceptions -fexceptions -fdiagnostics-show-option -fcolor-diagnostics -o /tmp/main-39d04f.o -x c++ main.cpp -faddrsig
clang -cc1 version 8.0.1 based upon LLVM 8.0.1 default target x86_64-linux-musl
ignoring nonexistent directory "/usr/local/include"
ignoring duplicate directory "/include"
#include "..." search starts here:
#include <...> search starts here:
 /bin/../include/c++/v1
 /include
 /lib/clang/8.0.1/include
End of search list.
 "/bin/ld.lld" -pie --eh-frame-hdr -m elf_x86_64 -dynamic-linker /lib/ld-musl-x86_64.so.1 -o a.out /bin/../lib/Scrt1.o /bin/../lib/crti.o /lib/clang/8.0.1/lib/linux/crtbeginS.o -L/bin/../lib -L/lib -L/usr/lib /tmp/main-39d04f.o -lc++ -lm /lib/clang/8.0.1/lib/linux/libclang_rt.builtins-x86_64.a -lc /lib/clang/8.0.1/lib/linux/libclang_rt.builtins-x86_64.a /lib/clang/8.0.1/lib/linux/crtendS.o /bin/../lib/crtn.o

/tmp # ldd a.out
        /lib/ld-musl-x86_64.so.1 (0x7f5979fe7000)
        libc++.so.1 => /lib/libc++.so.1 (0x7f5979f24000)
        libc++abi.so.1 => /lib/libc++abi.so.1 (0x7f5979ee7000)
        libunwind.so.1 => /lib/libunwind.so.1 (0x7f5979ed7000)
        libc.so => /lib/ld-musl-x86_64.so.1 (0x7f5979fe7000)
