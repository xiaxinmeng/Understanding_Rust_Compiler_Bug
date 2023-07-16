sh
perf record -g -e instructions:u -F max \
    setarch -R ./build/x86_64-unknown-linux-gnu/stage1/bin/rustc deep-vector.rs -C debuginfo=2
