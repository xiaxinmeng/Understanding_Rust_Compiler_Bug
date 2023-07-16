bash
git clone --branch llvmorg-12.0.1 --single-branch https://github.com/llvm/llvm-project.git

cd llvm-project/

cmake -S llvm -B build \
    -DCMAKE_BUILD_TYPE=Release \
    -DLLVM_BUILD_LLVM_DYLIB=ON \
    -DLLVM_LINK_LLVM_DYLIB=ON \
    -DLLVM_TARGETS_TO_BUILD="host;AMDGPU" \
    -DLLVM_ENABLE_RTTI=ON \
    -DLLVM_HOST_TRIPLE="$(cc -dumpmachine)" \
    -DLLVM_INCLUDE_BENCHMARKS=OFF \
    -DLLVM_INCLUDE_EXAMPLES=OFF \
    -DLLVM_INCLUDE_DOCS=OFF \
    -DLLVM_INCLUDE_TESTS=OFF \
    -DLLVM_ENABLE_TERMINFO=OFF \
    -DLLVM_ENABLE_LIBXML2=OFF \
    -Wno-dev

cmake --build build
