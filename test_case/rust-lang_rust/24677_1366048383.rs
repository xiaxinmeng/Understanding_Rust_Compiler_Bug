
BPF SDK: /root/.local/share/solana/install/releases/stable-ab8180121d35f47895def5cb6a68ad3109d91b6c/solana-release/bin/sdk/bpf
cargo-build-bpf child: rustup toolchain list -v
cargo-build-bpf child: cargo +bpf build --target bpfel-unknown-unknown --release
error: process didn't exit successfully: `rustc -vV` (exit status: 127)
--- stderr
/root/.rustup/toolchains/bpf/bin/rustc: error while loading shared libraries: librustc_driver-0a24654c7f10b1f7.so: cannot open shared object file: No such file or directory
