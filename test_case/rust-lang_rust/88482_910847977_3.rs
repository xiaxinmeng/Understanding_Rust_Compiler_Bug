
Updating only changed submodules
Submodules updated in 0.02 seconds
    Finished dev [unoptimized + debuginfo] target(s) in 0.39s
Building stage0 std artifacts (x86_64-apple-darwin -> x86_64-apple-darwin)
    Finished release [optimized] target(s) in 0.16s
Copying stage0 std from stage0 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
Building stage0 compiler artifacts (x86_64-apple-darwin -> x86_64-apple-darwin)
    Finished release [optimized] target(s) in 0.39s
Copying stage0 rustc from stage0 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
Building LLD for x86_64-apple-darwin
running: "cmake" "/Users/alexander/Developer/rust/src/llvm-project/lld" "-G" "Ninja" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER=cc" "-DCMAKE_CXX_COMPILER=c++" "-DCMAKE_ASM_COMPILER=cc" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64 -arch x86_64 -stdlib=libc++" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64 -arch x86_64 -stdlib=libc++" "-DLLVM_CONFIG_PATH=/Users/alexander/Developer/rust/build/bootstrap/debug/llvm-config-wrapper" "-DLLVM_INCLUDE_TESTS=OFF" "-DCMAKE_CXX_STANDARD=14" "-DCMAKE_INSTALL_PREFIX=/Users/alexander/Developer/rust/build/x86_64-apple-darwin/lld" "-DCMAKE_ASM_FLAGS= -ffunction-sections -fdata-sections -fPIC -m64 -arch x86_64" "-DCMAKE_BUILD_TYPE=Release"
CMake Error at CMakeLists.txt:41 (message):
  LLVMConfig.cmake not found


-- Configuring incomplete, errors occurred!
See also "/Users/alexander/Developer/rust/build/x86_64-apple-darwin/lld/build/CMakeFiles/CMakeOutput.log".
thread 'main' panicked at '
command did not execute successfully, got: exit status: 1

build script failed, must exit now', /Users/alexander/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.44/src/lib.rs:885:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
	finished in 0.100 seconds
Build completed unsuccessfully in 0:00:02
