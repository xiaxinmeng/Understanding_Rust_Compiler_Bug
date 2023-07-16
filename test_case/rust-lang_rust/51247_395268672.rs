plain
[00:03:34] fatal: clone of 'https://github.com/rust-lang/cargo.git' into submodule path '/home/travis/build/rust-lang/rust/src/tools/cargo' failed
[00:03:34] Failed to clone 'src/tools/cargo'. Retry scheduled
[00:03:34] Cloning into '/home/travis/build/rust-lang/rust/src/tools/cargo'...
[00:03:36] Cloning into '/home/travis/build/rust-lang/rust/src/libbacktrace'...
[00:07:07] fatal: unable to access 'https://github.com/rust-lang-nursery/libbacktrace/': Failed to connect to github.com port 443: Connection timed out
[00:07:07] fatal: clone of 'https://github.com/rust-lang-nursery/libbacktrace' into submodule path '/home/travis/build/rust-lang/rust/src/libbacktrace' failed
[00:07:07] Failed to clone 'src/libbacktrace' a second time, aborting
[00:07:08] Cleared directory 'src/dlmalloc'
[00:07:08] Submodule 'src/dlmalloc' (https://github.com/alexcrichton/dlmalloc-rs.git) unregistered for path 'src/dlmalloc'
[00:07:08] Cleared directory 'src/doc/nomicon'
[00:07:08] Submodule 'src/doc/nomicon' (https://github.com/rust-lang-nursery/nomicon.git) unregistered for path 'src/doc/nomicon'
---
###############################################################           88.3%
######################################################################## 100.0%
[00:10:18] extracting /checkout/obj/build/cache/2018-05-10/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:10:18]     Updating registry `https://github.com/rust-lang/crates.io-index`
No output has been received in the last 30m0s, this potentially indicates a stalled build or something wrong with the build itself.
Check the details on how to adjust your build configuration on: https://docs.travis-ci.com/user/common-build-problems/#Build-times-out-because-no-output-was-received
The build has been terminated
