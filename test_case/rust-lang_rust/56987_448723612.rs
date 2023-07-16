
$ find master -type f | xargs du -ah | sort -rh | head
74M     master/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
68M     master/lib/rustlib/x86_64-unknown-linux-gnu/lib/libLLVM-8svn.so
36M     master/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-emscripten.so
36M     master/bin/rustdoc
26M     master/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc-8197a5f727130e18.so
26M     master/lib/librustc-8197a5f727130e18.so
25M     master/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-f929d44e8ee822a6.rlib
11M     master/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_mir-9e333f7280aadc85.so
11M     master/lib/librustc_mir-9e333f7280aadc85.so
9.9M    master/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-317bdea014a4d323.rlib

$ find try -type f | xargs du -ah | sort -rh | head
116M    try/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
87M     try/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc-749452a0b8d67c66.rlib
78M     try/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-emscripten.so
68M     try/lib/rustlib/x86_64-unknown-linux-gnu/lib/libLLVM-8svn.so
57M     try/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_driver-7422448280a926b8.so
57M     try/lib/librustc_driver-7422448280a926b8.so
37M     try/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_mir-5bcf33ecc7db40f1.rlib
36M     try/bin/rustdoc
25M     try/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-8a01f40ec9cf63ef.rlib
23M     try/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax-c233c5cd5247336c.rlib
