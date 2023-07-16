
# Get our parent/master toolchains
rustup-toolchain-install-master 718d53b0cb7dde93499cb92950d60b412f5a3d05 da7d405357600a76f2b93b8aa41fe5ee5da7885d
# Benchmark our top regressed benchmark in cachegrind
cargo run --release --bin collector -- profile_local cachegrind +718d53b0cb7dde93499cb92950d60b412f5a3d05 before --include stm32f4 --builds Check --runs Full
cargo run --release --bin collector -- profile_local cachegrind +da7d405357600a76f2b93b8aa41fe5ee5da7885d after --include stm32f4 --builds Check --runs Full
