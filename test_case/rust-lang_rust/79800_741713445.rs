
patchelf --set-interpreter $BREW_PREFIX/lib/ld-linux-x86-64.so.2 \
    build/x86_64-unknown-linux-gnu/stage0/bin/rustc

patchelf --set-rpath $(pwd)/build/x86_64-unknown-linux-gnu/stage0/lib/:$BREW_PREFIX/lib \
    build/x86_64-unknown-linux-gnu/stage0/lib/*.so 

make
