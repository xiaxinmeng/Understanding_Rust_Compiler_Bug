toml
[target.'cfg(any(windows, unix))']                                              
rustflags = ["-C", "target-cpu=native"]
# src: https://users.rust-lang.org/t/auto-vectorization-in-rust/24379/4
