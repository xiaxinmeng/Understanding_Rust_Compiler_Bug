
cargo +1.45 run # uses 1.45.2
rustup toolchain uninstall 1.45 # removes 1.45.2, does not touch stable or 1.45.1 installation
cargo +1.45 run # errors with 'toolchain 1.45 is not installed', even though 1.45.1 is available and stable is 1.45.2
