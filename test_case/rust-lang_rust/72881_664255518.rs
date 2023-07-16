
./configure --build=i686-pc-windows-msvc                 \
   --host=i686-pc-windows-msvc,aarch64-pc-windows-msvc   \
   --target=i686-pc-windows-msvc,aarch64-pc-windows-msvc \
   --enable-full-tools --enable-profiler                 \
   --enable-missing-tools                                \
   --set llvm.clang-cl=C:/LLVM/bin/clang-cl.exe
