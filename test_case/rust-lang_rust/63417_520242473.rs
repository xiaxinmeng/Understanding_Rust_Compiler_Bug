
    Finished release [optimized] target(s) in 14m 29s
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building LLVM for x86_64-unknown-linux-gnu
running: "cmake" "/home/luna/shared/projects/rust/src/llvm-project/llvm" "-DLLVM_ENABLE_ASSERTIONS=ON" "-DLLVM_TARGETS_TO_BUILD=AArch64;ARM;Hexagon;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DWITH_POLLY=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_Z3_SOLVER=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=4" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DLLVM_OCAML_INSTALL_PATH=usr/lib/ocaml" "-DLLVM_LINK_LLVM_DYLIB=ON" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_VERSION_SUFFIX=-rust-1.38.0-dev-f2b0c6766" "-DPYTHON_EXECUTABLE=/usr/bin/python2.7" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER=clang" "-DCMAKE_CXX_COMPILER=clang++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu -static-libstdc++" "-DCMAKE_INSTALL_PREFIX=/home/luna/shared/projects/rust/build/x86_64-unknown-linux-gnu/llvm" "-DCMAKE_BUILD_TYPE=Release"
-- Could NOT find Z3: Found unsuitable version "0.0.0", but required is at least "4.7.1" (found Z3_LIBRARIES-NOTFOUND)
CMake Error at cmake/modules/CheckCompilerVersion.cmake:83 (message):
  libstdc++ version must be at least 4.8.
Call Stack (most recent call first):
  cmake/config-ix.cmake:13 (include)
  CMakeLists.txt:618 (include)


-- Configuring incomplete, errors occurred!
See also "/home/luna/shared/projects/rust/build/x86_64-unknown-linux-gnu/llvm/build/CMakeFiles/CMakeOutput.log".
See also "/home/luna/shared/projects/rust/build/x86_64-unknown-linux-gnu/llvm/build/CMakeFiles/CMakeError.log".
thread 'main' panicked at '
command did not execute successfully, got: exit code: 1

build script failed, must exit now', /home/luna/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.38/src/lib.rs:813:5
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:47
   3: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:36
   4: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:200
   5: std::panicking::default_hook
             at src/libstd/panicking.rs:214
   6: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:477
   7: std::panicking::continue_panic_fmt
             at src/libstd/panicking.rs:384
   8: std::panicking::begin_panic_fmt
             at src/libstd/panicking.rs:339
   9: cmake::fail
             at /home/luna/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.38/src/lib.rs:813
  10: cmake::run
             at /home/luna/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.38/src/lib.rs:791
  11: cmake::Config::build
             at /home/luna/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.38/src/lib.rs:629
  12: <bootstrap::native::Llvm as bootstrap::builder::Step>::run
             at src/bootstrap/native.rs:304
  13: bootstrap::builder::Builder::ensure
             at src/bootstrap/builder.rs:1285
  14: bootstrap::compile::build_codegen_backend
             at src/bootstrap/compile.rs:749
  15: <bootstrap::compile::CodegenBackend as bootstrap::builder::Step>::run
             at src/bootstrap/compile.rs:706
  16: bootstrap::builder::Builder::ensure
             at src/bootstrap/builder.rs:1285
  17: <bootstrap::compile::CodegenBackend as bootstrap::builder::Step>::make_run
             at src/bootstrap/compile.rs:667
  18: bootstrap::builder::StepDescription::maybe_run
             at src/bootstrap/builder.rs:183
  19: bootstrap::builder::StepDescription::run
             at src/bootstrap/builder.rs:207
  20: bootstrap::builder::Builder::run_step_descriptions
             at src/bootstrap/builder.rs:560
  21: bootstrap::builder::Builder::execute_cli
             at src/bootstrap/builder.rs:550
  22: bootstrap::Build::build
             at src/bootstrap/lib.rs:456
  23: bootstrap::main
             at src/bootstrap/bin/main.rs:18
  24: std::rt::lang_start::{{closure}}
             at /rustc/178aa66119c5bf7c6f2f0bf1551319e54b86ea93/src/libstd/rt.rs:64
  25: std::rt::lang_start_internal::{{closure}}
             at src/libstd/rt.rs:49
  26: std::panicking::try::do_call
             at src/libstd/panicking.rs:296
  27: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:82
  28: std::panicking::try
             at src/libstd/panicking.rs:275
  29: std::panic::catch_unwind
             at src/libstd/panic.rs:394
  30: std::rt::lang_start_internal
             at src/libstd/rt.rs:48
  31: std::rt::lang_start
             at /rustc/178aa66119c5bf7c6f2f0bf1551319e54b86ea93/src/libstd/rt.rs:64
  32: main
  33: __libc_start_main
  34: _start
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
	finished in 1.245
failed to run: /home/luna/shared/projects/rust/build/bootstrap/debug/bootstrap build --stage 0
Build completed unsuccessfully in 0:15:17
