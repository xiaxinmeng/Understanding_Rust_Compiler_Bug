
cargo install rustup-toolchain-install-master
rustup-toolchain-install-master bd0bacc694d7d8175804bb6f690cb846bfa4a9ee -n old
rustup-toolchain-install-master e992a97da01ade25fd0eab2f30d9c5fe0e3e1591 -n new
cargo +old bench
cargo +new bench
