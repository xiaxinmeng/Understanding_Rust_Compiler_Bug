
./configure \
    --build=aarch64-pc-windows-msvc               \
    --host=aarch64-pc-windows-msvc                \
    --target=aarch64-pc-windows-msvc              \
    --enable-full-tools --enable-profiler         \
    --enable-missing-tools --enable-ninja         \
    --set build.rustc=C:/some/path/to/native/rustc.exe      \
    --set build.cargo=C:/some/path/to/native/cargo.exe      \
    --set build.rustfmt=C:/some/path/to/native/rustfmt.exe  \
    --set llvm.clang-cl=C:/some/path/clang-cl.exe (native or x86_32, probably both work?)\
    --set rust.incremental                        \
    --set rust.backtrace                            \
    --enable-verbose-tests                        \
    --enable-codegen-tests                        \
    --enable-lld
