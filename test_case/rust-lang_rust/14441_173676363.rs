
$ objdump -S i686-unknown-linux-gnu/llvm/Release/bin/llc | grep 'xmm' | cut -d: -f2- | sort -u | head -n 3
$ objdump -S i686-unknown-linux-gnu/llvm/Release/bin/llc | grep 'st(' | cut -d: -f2- | sort -u | head -n 3
    d8 c1                   fadd   %st(1),%st
    d8 c9                   fmul   %st(1),%st
    d8 e4                   fsub   %st(4),%st

$ objdump -S i686-unknown-linux-gnu/stage2/lib/libstd-db5a760f.so | grep 'xmm' | cut -d: -f2- | sort -u | head -n 3
    0f 10 0a                movups (%edx),%xmm1
    0f 10 0e                movups (%esi),%xmm1
    0f 10 0f                movups (%edi),%xmm1
$ objdump -S i686-unknown-linux-gnu/stage2/lib/libstd-db5a760f.so | grep 'st(' | cut -d: -f2- | sort -u | head -n 3
$
