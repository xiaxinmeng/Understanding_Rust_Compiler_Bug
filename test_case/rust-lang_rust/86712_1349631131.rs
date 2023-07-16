shell
% ND=nightly-2021-08-21 ; rustup update $ND ; rustup target add --toolchain $ND x86_64-unknown-linux-musl ; RUSTFLAGS="-Ctarget-feature=+crt-static -Clinker=rust-lld" cargo +$ND build --target x86_64-unknown-linux-musl && ./target/x86_64-unknown-linux-musl/debug/test_crate
info: syncing channel updates for 'nightly-2021-08-21-x86_64-unknown-linux-gnu'

  nightly-2021-08-21-x86_64-unknown-linux-gnu unchanged - rustc 1.56.0-nightly (a0035916e 2021-08-20)

info: checking for self-updates
info: component 'rust-std' for target 'x86_64-unknown-linux-musl' is up to date
   Compiling bitflags v1.2.1
   Compiling libc v0.2.97
   Compiling nix v0.17.0
   Compiling cfg-if v0.1.10
   Compiling void v1.0.2
   Compiling test_crate v0.1.0 (/media/pnkfelix/Rust/issue_86712/rust-musl-segfault)
warning: unused import: `nix::sys::stat::Mode`
 --> src/main.rs:1:5
  |
1 | use nix::sys::stat::Mode;
  |     ^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `test_crate` (bin "test_crate") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 3.64s
Segmentation fault (core dumped)
15-15-24 issue_86712/rust-musl-segfault (git:main) [ERROR#139] % ND=nightly-2021-08-22 ; rustup update $ND ; rustup target add --toolchain $ND x86_64-unknown-linux-musl ; RUSTFLAGS="-Ctarget-feature=+crt-static -Clinker=rust-lld" cargo +$ND build --target x86_64-unknown-linux-musl && ./target/x86_64-unknown-linux-musl/debug/test_crate
info: syncing channel updates for 'nightly-2021-08-22-x86_64-unknown-linux-gnu'

  nightly-2021-08-22-x86_64-unknown-linux-gnu unchanged - rustc 1.56.0-nightly (d3e2578c3 2021-08-21)

info: checking for self-updates
info: component 'rust-std' for target 'x86_64-unknown-linux-musl' is up to date
   Compiling libc v0.2.97
   Compiling bitflags v1.2.1
   Compiling nix v0.17.0
   Compiling cfg-if v0.1.10
   Compiling void v1.0.2
   Compiling test_crate v0.1.0 (/media/pnkfelix/Rust/issue_86712/rust-musl-segfault)
warning: unused import: `nix::sys::stat::Mode`
 --> src/main.rs:1:5
  |
1 | use nix::sys::stat::Mode;
  |     ^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `test_crate` (bin "test_crate") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 3.60s
%
