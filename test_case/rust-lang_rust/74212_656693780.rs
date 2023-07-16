
/home/lzutao/fork/rust/compiler/build/x86_64-unknown-linux-gnu/stage0/bin/cargo \
    metadata \
    --all-features \
    --manifest-path=/home/lzutao/fork/rust/compiler/Cargo.toml \
    | jq '.packages[] | select(.name=="rdrand")'
