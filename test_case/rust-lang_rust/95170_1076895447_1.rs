
$ ./x.py check -v
detected default triple x86_64-unknown-linux-gnu from pre-installed rustc
Updating only changed submodules
  Submodules updated in 0.01 seconds
Building rustbuild
running: /home/vh/git/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /home/vh/git/rust/src/bootstrap/Cargo.toml --verbose
[...]
    Finished dev [unoptimized] target(s) in 0.08s
running: /home/vh/git/rust/build/bootstrap/debug/bootstrap check -v
finding compilers
CC_x86_64-unknown-linux-gnu = "gcc"
CFLAGS_x86_64-unknown-linux-gnu = ["-ffunction-sections", "-fdata-sections", "-fPIC", "-m64"]
CXX_x86_64-unknown-linux-gnu = "g++"
CXXFLAGS_x86_64-unknown-linux-gnu = ["-ffunction-sections", "-fdata-sections", "-fPIC", "-m64"]
AR_x86_64-unknown-linux-gnu = "ar"
running sanity check
learning about cargo
downloading https://ci-artifacts.rust-lang.org/rustc-builds/282778aee26166754315815552bae454fc968960/rust-dev-nightly-x86_64-unknown-linux-gnu.tar.xz
running: "curl" "-#" "-y" "30" "-Y" "10" "--connect-timeout" "30" "--retry" "3" "-Sf" "-o" "/run/user/1000/.tmpuU8ncP" "https://ci-artifacts.rust-lang.org/rustc-builds/282778aee26166754315815552bae454fc968960/rust-dev-nightly-x86_64-unknown-linux-gnu.tar.xz"
############################################################################################################ 100.0%
moving /run/user/1000/.tmpuU8ncP to /home/vh/git/rust/build/cache/llvm-282778aee26166754315815552bae454fc968960-false/rust-dev-nightly-x86_64-unknown-linux-gnu.tar.xz
thread 'main' panicked at 'fs::rename(temppath, dest_path) failed with Invalid cross-device link (os error 18)', src/bootstrap/native.rs:296:5
[...]

$ mount | grep /run/user/1000
tmpfs on /run/user/1000 type tmpfs (rw,nosuid,nodev,relatime,size=6564484k,nr_inodes=1641121,mode=700,uid=1000,gid=100)

