console
$ rm -rf rust/build
$ RUST_BACKTRACE=1 cargo run --bin collector bench_local --id dc89a803d64fb6172c8406996831353bee18c3a7 /home/kobzol/.rustup/toolchains/nightly-2023-02-18-x86_64-unknown-linux-gnu/bin/rustc --bench-rustc --iterations 1 --cargo /home/kobzol/.rustup/toolchains/nightly-2023-02-18-x86_64-unknown-linux-gnu/bin/cargo --include await

(examined with strace)
[pid 40260] symlink("/projects/personal/rust/rustc-perf/rust/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt", "/projects/personal/rust/rustc-perf/target/debug/rustfmt") = -1 EEXIST (File exists)
