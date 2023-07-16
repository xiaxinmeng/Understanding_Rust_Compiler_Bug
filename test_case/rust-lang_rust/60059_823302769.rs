
$ cmake -DCMAKE_INSTALL_PREFIX=/opt/llvm-12.0.0 \
    -DLLVM_INSTALL_UTILS=On \
    -DCMAKE_BUILD_TYPE=release \
    -DLLVM_ENABLE_PROJECTS="lld;clang" \
    ../llvm
