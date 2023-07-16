
$ RUSTFLAGS="-C link-args=-zstack-size=1024" cargo run --release | wc -l
1025
