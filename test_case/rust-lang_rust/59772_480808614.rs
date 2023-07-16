plain
travis_time:end:1a16008e:start=1554722762123891347,finish=1554722849761212536,duration=87637321189
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:02:27]   Downloaded bit-vec v0.5.0
[00:02:27]   Downloaded rustc-ap-rustc_data_structures v407.0.0
[00:02:27]   Downloaded rusty-fork v0.2.1
[00:02:27]   Downloaded walkdir v2.2.7
[00:02:28]   Downloaded futures-cpupool v0.1.8
[00:02:28]   Downloaded serde_urlencoded v0.5.4
[00:02:28]   Downloaded crc32fast v1.1.2
[00:02:28]   Downloaded foreign-types-shared v0.1.1
[00:02:28]   Downloaded synstructure v0.10.1
---
[00:52:31]    Compiling string v0.1.3
[00:52:31]    Compiling ucd-util v0.1.3
[00:52:32]    Compiling unicode-segmentation v1.2.1
[00:52:32]    Compiling unicode-xid v0.0.4
[00:52:32]    Compiling try-lock v0.2.2
[00:52:33]    Compiling crc32fast v1.1.2
[00:52:33]    Compiling regex v1.1.0
[00:52:34]    Compiling indexmap v1.0.2
[00:52:34]    Compiling native-tls v0.2.2
---
[00:53:12]    Compiling crossbeam-epoch v0.3.1
[00:53:13]    Compiling rustc_version v0.2.3
[00:53:16]    Compiling idna v0.1.5
[00:53:17]    Compiling pest_meta v2.1.0
[00:53:18] error: failed to run custom build command for `openssl-sys v0.9.40`
[00:53:18] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/build/openssl-sys-aaef693af933fbb3/build-script-main` (exit code: 101)
[00:53:18] --- stdout
[00:53:18] cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_LIB_DIR
[00:53:18] cargo:rerun-if-env-changed=OPENSSL_LIB_DIR
[00:53:18] cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_INCLUDE_DIR
[00:53:18] cargo:rerun-if-env-changed=OPENSSL_INCLUDE_DIR
[00:53:18] cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_DIR
[00:53:18] cargo:rerun-if-env-changed=OPENSSL_DIR
[00:53:18] run pkg_config fail: "Failed to run `\"pkg-config\" \"--libs\" \"--cflags\" \"openssl\"`: No such file or directory (os error 2)"
[00:53:18] --- stderr
[00:53:18] thread 'main' panicked at '
[00:53:18] 
[00:53:18] 
[00:53:18] Could not find directory of OpenSSL installation, and this `-sys` crate cannot
[00:53:18] proceed without this knowledge. If OpenSSL is installed and this crate had
[00:53:18] trouble finding it,  you can set the `OPENSSL_DIR` environment variable for the
[00:53:18] 
[00:53:18] 
[00:53:18] Make sure you also have the development packages of openssl installed.
[00:53:18] For example, `libssl-dev` on Ubuntu or `openssl-devel` on Fedora.
[00:53:18] 
[00:53:18] If you're in a situation where you think the directory *should* be found
[00:53:18] automatically, please open a bug at https://github.com/sfackler/rust-openssl
[00:53:18] and include information about your system as well as this message.
[00:53:18]     $HOST = x86_64-unknown-linux-gnu
[00:53:18]     $TARGET = x86_64-unknown-linux-gnu
[00:53:18]     openssl-sys = 0.9.40
[00:53:18] 
[00:53:18] 
[00:53:18] 
[00:53:18] It looks like you're compiling on Linux and also targeting Linux. Currently this
[00:53:18] requires the `pkg-config` utility to find OpenSSL but unfortunately `pkg-config`
[00:53:18] could not be found. If you have OpenSSL installed you can likely fix this by
[00:53:18] installing `pkg-config`.
[00:53:18] ', /cargo/registry/src/github.com-1ecc6299db9ec823/openssl-sys-0.9.40/build/main.rs:269:9
[00:53:18] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:53:18] 
[00:53:18] warning: build failed, waiting for other jobs to finish...
