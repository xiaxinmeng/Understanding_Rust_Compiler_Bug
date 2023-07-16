
$ diff -u \
    ./build/arm-unknown-linux-gnueabi/stage1/bin/rustc \
    ./build/x86_64-unknown-linux-gnu/stage0-rustc/arm-unknown-linux-gnueabi/release/rustc && \
    echo SAME
SAME

$ diff -u \
    ./build/arm-unknown-linux-gnueabi/stage1/bin/rustc \
    ./build/x86_64-unknown-linux-gnu/stage1-rustc/arm-unknown-linux-gnueabi/release/rustc && \
    echo SAME
Binary files ./build/arm-unknown-linux-gnueabi/stage1/bin/rustc and ./build/x86_64-unknown-linux-gnu/stage1-rustc/arm-unknown-linux-gnueabi/release/rustc differ
