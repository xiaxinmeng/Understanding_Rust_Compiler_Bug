
$ nix-shell -p 

[nix-shell:~/git/rust]$ rm -R build

[nix-shell:~/git/rust]$ cat config.toml
# Includes one of the default files in src/bootstrap/defaults
profile = "library"
changelog-seen = 2
[llvm]
download-ci-llvm = true

[nix-shell:~/git/rust]$ git log --pretty=oneline master..@
116f574643dbc38a7fdb21dba2a8a98b6a31648a (HEAD -> ci-llvm) bootstrap.py: nixos check in /etc/os-release with quotes
faa7ed79625928e6b7b08827fe6712f4f85c37f4 Fix bootstrap tests
2abdca8665dcf0e6feffa349dface8ee26fd81b1 fix caching bug on Mac and Windows
608973f1f33414db1f66e6088c75f914948b63f4 Move download-ci-llvm to rustbuild

[nix-shell:~/git/rust]$ ./x.py check
Updating only changed submodules
  Submodules updated in 0.01 seconds
downloading https://static.rust-lang.org/dist/2022-02-22/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
############################################################################################################ 100.0%
extracting /home/vh/git/rust/build/cache/2022-02-22/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
downloading https://static.rust-lang.org/dist/2022-02-22/rustc-beta-x86_64-unknown-linux-gnu.tar.xz
############################################################################################################ 100.0%
extracting /home/vh/git/rust/build/cache/2022-02-22/rustc-beta-x86_64-unknown-linux-gnu.tar.xz
downloading https://static.rust-lang.org/dist/2022-02-22/cargo-beta-x86_64-unknown-linux-gnu.tar.xz
############################################################################################################ 100.0%
extracting /home/vh/git/rust/build/cache/2022-02-22/cargo-beta-x86_64-unknown-linux-gnu.tar.xz
info: you seem to be using Nix. Attempting to patch /home/vh/git/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo
info: you seem to be using Nix. Attempting to patch /home/vh/git/rust/build/x86_64-unknown-linux-gnu/stage0/bin/rustc
info: you seem to be using Nix. Attempting to patch /home/vh/git/rust/build/x86_64-unknown-linux-gnu/stage0/bin/rustdoc
info: you seem to be using Nix. Attempting to patch /home/vh/git/rust/build/x86_64-unknown-linux-gnu/stage0/lib/libtest-d9ecd5377b7dfa2f.so
info: you seem to be using Nix. Attempting to patch /home/vh/git/rust/build/x86_64-unknown-linux-gnu/stage0/lib/libLLVM-14-rust-1.60.0-beta.so
info: you seem to be using Nix. Attempting to patch /home/vh/git/rust/build/x86_64-unknown-linux-gnu/stage0/lib/librustc_driver-c6cd2e91ce71a7d4.so
info: you seem to be using Nix. Attempting to patch /home/vh/git/rust/build/x86_64-unknown-linux-gnu/stage0/lib/libstd-ff9290e971253a38.so
downloading https://static.rust-lang.org/dist/2022-02-23/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
############################################################################################################ 100.0%
extracting /home/vh/git/rust/build/cache/2022-02-23/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
info: you seem to be using Nix. Attempting to patch /home/vh/git/rust/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt
info: you seem to be using Nix. Attempting to patch /home/vh/git/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo-fmt
Building rustbuild
   Compiling libc v0.2.116
   Compiling proc-macro2 v1.0.30
   Compiling memchr v2.4.1
   Compiling unicode-xid v0.2.2
   Compiling cfg-if v1.0.0
   Compiling getrandom v0.2.0
   Compiling syn v1.0.80
   Compiling lazy_static v1.4.0
   Compiling serde_derive v1.0.125
   Compiling cfg-if v0.1.10
   Compiling cc v1.0.69
   Compiling serde v1.0.125
   Compiling regex-automata v0.1.10
   Compiling pkg-config v0.3.18
   Compiling log v0.4.14
   Compiling ppv-lite86 v0.2.8
   Compiling ryu v1.0.5
   Compiling regex-syntax v0.6.25
   Compiling crossbeam-utils v0.8.6
   Compiling serde_json v1.0.59
   Compiling once_cell v1.7.2
   Compiling fnv v1.0.7
   Compiling same-file v1.0.6
   Compiling unicode-width v0.1.8
   Compiling remove_dir_all v0.5.3
   Compiling itoa v0.4.6
   Compiling bootstrap v0.0.0 (/home/vh/git/rust/src/bootstrap)
   Compiling cmake v0.1.44
   Compiling thread_local v1.1.4
   Compiling getopts v0.2.21
   Compiling walkdir v2.3.1
   Compiling aho-corasick v0.7.18
   Compiling bstr v0.2.13
   Compiling quote v1.0.7
   Compiling lzma-sys v0.1.16
   Compiling filetime v0.2.14
   Compiling xattr v0.2.2
   Compiling rand_core v0.6.2
   Compiling tar v0.4.37
   Compiling rand_chacha v0.3.0
   Compiling opener v0.5.0
   Compiling rand v0.8.4
   Compiling regex v1.5.4
   Compiling tempfile v3.2.0
   Compiling globset v0.4.5
   Compiling ignore v0.4.17
   Compiling xz2 v0.1.6
   Compiling toml v0.5.7
    Finished dev [unoptimized] target(s) in 18.35s
downloading https://ci-artifacts.rust-lang.org/rustc-builds/282778aee26166754315815552bae454fc968960/rust-dev-nightly-x86_64-unknown-linux-gnu.tar.xz
############################################################################################################ 100.0%
thread 'main' panicked at 'fs::rename(temppath, dest_path) failed with Invalid cross-device link (os error 18)', src/bootstrap/native.rs:296:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Build completed unsuccessfully in 0:01:11
