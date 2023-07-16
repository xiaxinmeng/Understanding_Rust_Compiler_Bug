
-- Build files have been written to: /mnt/scratch/git/rust-wasm/src/rustc-1.49.0-src/build/x86_64-unknown-linux-gnu/lld/build
running: "cmake" "--build" "." "--target" "install" "--config" "Release" "--" "-j" "8"
[61/141] Building CXX object COFF/CMakeFiles/lldCOFF.dir/Writer.cpp.o
FAILED: COFF/CMakeFiles/lldCOFF.dir/Writer.cpp.o 
/usr/lib/ccache/bin/c++ -DGTEST_HAS_RTTI=0 -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -ICOFF -I/mnt/scratch/git/rust-wasm/src/rustc-1.49.0-src/src/llvm-project/lld/COFF -I/mnt/scratch/git/rust-wasm/src/rustc-1.49.0-src/src/llvm-project/lld/include -Iinclude -ffunction-sections -fdata-sections -fPIC -m64 -march=x86-64 -mtune=generic -pipe -fno-plt -fPIC -fvisibility-inlines-hidden -Werror=date-time -w -fdiagnostics-color -ffunction-sections -fdata-sections -O3 -DNDEBUG  -fno-exceptions -fno-rtti -std=gnu++14 -MD -MT COFF/CMakeFiles/lldCOFF.dir/Writer.cpp.o -MF COFF/CMakeFiles/lldCOFF.dir/Writer.cpp.o.d -o COFF/CMakeFiles/lldCOFF.dir/Writer.cpp.o -c /mnt/scratch/git/rust-wasm/src/rustc-1.49.0-src/src/llvm-project/lld/COFF/Writer.cpp
/mnt/scratch/git/rust-wasm/src/rustc-1.49.0-src/src/llvm-project/lld/COFF/Writer.cpp: In member function ‘void {anonymous}::Writer::fixTlsAlignment()’:
/mnt/scratch/git/rust-wasm/src/rustc-1.49.0-src/src/llvm-project/lld/COFF/Writer.cpp:2030:13: error: ‘using coff_tls_directory64 = struct llvm::object::coff_tls_directory<llvm::support::detail::packed_endian_specific_integral<long int, llvm::support::little, 1> >’ {aka ‘struct llvm::object::coff_tls_directory<llvm::support::detail::packed_endian_specific_integral<long int, llvm::support::little, 1> >’} has no member named ‘setAlignment’; did you mean ‘getAlignment’?
 2030 |     tlsDir->setAlignment(tlsAlignment);
      |             ^~~~~~~~~~~~
      |             getAlignment
/mnt/scratch/git/rust-wasm/src/rustc-1.49.0-src/src/llvm-project/lld/COFF/Writer.cpp:2034:13: error: ‘using coff_tls_directory32 = struct llvm::object::coff_tls_directory<llvm::support::detail::packed_endian_specific_integral<int, llvm::support::little, 1> >’ {aka ‘struct llvm::object::coff_tls_directory<llvm::support::detail::packed_endian_specific_integral<int, llvm::support::little, 1> >’} has no member named ‘setAlignment’; did you mean ‘getAlignment’?
 2034 |     tlsDir->setAlignment(tlsAlignment);
      |             ^~~~~~~~~~~~
      |             getAlignment
[68/141] Building CXX object COFF/CMakeFiles/lldCOFF.dir/SymbolTable.cpp.o
ninja: build stopped: subcommand failed.
thread 'main' panicked at '
command did not execute successfully, got: exit code: 1

build script failed, must exit now', /mnt/scratch/git/rust-wasm/src/rustc-1.49.0-src/vendor/cmake/src/lib.rs:885:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
	finished in 24.215 seconds
failed to run: /mnt/scratch/git/rust-wasm/src/rustc-1.49.0-src/build/bootstrap/debug/bootstrap dist -j 8
Build completed unsuccessfully in 0:07:46
==> ERROR: A failure occurred in build().
