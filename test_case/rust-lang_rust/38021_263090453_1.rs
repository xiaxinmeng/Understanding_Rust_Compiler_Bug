
$ rustup run nightly-2016-11-06 cargo run --bin json-benchmark --release --no-default-features --features="lib-json-rust performance file-canada" -- -n 256
                                DOM                STRUCT
======= json-rust ======== parse|stringify === parse|stringify ===
data/canada.json           9.1ms     3.9ms
