
% cargo new repro
% cd repro
% cat <<EOF >> Cargo.toml
log = "0.4.17"

[profile.release]
lto = true # or "thin"
debug = true
split-debuginfo = "unpacked" # or "packed"
EOF
% cargo build --release --verbose
   Compiling log v0.4.17
   Compiling cfg-if v1.0.0
     Running `rustc --crate-name build_script_build /home/tc/.cargo/registry/src/github.com-1ecc6299db9ec823/log-0.4.17/build.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C split-debuginfo=packed -C debuginfo=2 -C debug-assertions=off -C metadata=3d94976ee9c5caca -C extra-filename=-3d94976ee9c5caca --out-dir /home/tc/dev/repro/target/release/build/log-3d94976ee9c5caca -L dependency=/home/tc/dev/repro/target/release/deps --cap-lints allow`
     Running `rustc --crate-name cfg_if --edition=2018 /home/tc/.cargo/registry/src/github.com-1ecc6299db9ec823/cfg-if-1.0.0/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C linker-plugin-lto -C split-debuginfo=packed -C debuginfo=2 -C metadata=9e1e3997dce77d3a -C extra-filename=-9e1e3997dce77d3a --out-dir /home/tc/dev/repro/target/release/deps -L dependency=/home/tc/dev/repro/target/release/deps --cap-lints allow`
error: failed to build archive: No such file or directory

error: could not compile `cfg-if` due to previous error

Caused by:
  process didn't exit successfully: `rustc --crate-name cfg_if --edition=2018 /home/tc/.cargo/registry/src/github.com-1ecc6299db9ec823/cfg-if-1.0.0/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C linker-plugin-lto -C split-debuginfo=packed -C debuginfo=2 -C metadata=9e1e3997dce77d3a -C extra-filename=-9e1e3997dce77d3a --out-dir /home/tc/dev/repro/target/release/deps -L dependency=/home/tc/dev/repro/target/release/deps --cap-lints allow` (exit status: 1)
warning: build failed, waiting for other jobs to finish...
% rustc --version --verbose
rustc 1.65.0 (897e37553 2022-11-02)
binary: rustc
commit-hash: 897e37553bba8b42751c67658967889d11ecd120
commit-date: 2022-11-02
host: x86_64-unknown-linux-gnu
release: 1.65.0
LLVM version: 15.0.0
% rustup show
Default host: x86_64-unknown-linux-gnu
rustup home:  /home/tc/.rustup

stable-x86_64-unknown-linux-gnu (default)
rustc 1.65.0 (897e37553 2022-11-02)
