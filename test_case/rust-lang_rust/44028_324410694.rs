
$ rg 'gcc( =|\])' --type toml
src/bootstrap/Cargo.toml
37:gcc = "0.3.50"

src/liballoc_jemalloc/Cargo.toml
22:gcc = "0.3.50"

src/libcompiler_builtins/Cargo.toml
11:[build-dependencies.gcc]

src/libprofiler_builtins/Cargo.toml
18:gcc = "0.3.50"

src/librustc_llvm/Cargo.toml
20:gcc = "0.3.50"

src/librustdoc/Cargo.toml
18:gcc = "0.3.50"

src/librustc_trans/Cargo.toml
35:gcc = "0.3.50"

src/libstd/Cargo.toml
39:gcc = "0.3.50"

src/liblibc/libc-test/run-generated-Cargo.toml
19:gcc = "0.3"

src/rustc/compiler_builtins_shim/Cargo.toml
18:gcc = "0.3"
