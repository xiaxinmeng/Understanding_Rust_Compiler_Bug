
# This mingw-w64 toolchain uses compiler-rt by the default
$ /clang64/bin/clang -print-libgcc-file-name
D:/msys64/clang64/lib/clang/12.0.0/lib/windows/libclang_rt.builtins-x86_64.a


# This mingw-w64 toolchain uses libgcc by the default
$ /mingw64/bin/clang -print-libgcc-file-name
D:/msys64/mingw64/lib/gcc/x86_64-w64-mingw32/10.3.0/libgcc.a

$ /mingw64/bin/clang -print-libgcc-file-name -rtlib=compiler-rt
D:/msys64/mingw64/lib/clang/12.0.0/lib/windows/libclang_rt.builtins-x86_64.a

