plain
travis_time:end:05de7330:start=1554654956582235202,finish=1554654958983432994,duration=2401197792
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:03:05]   Downloaded racer v2.1.21
[00:03:05]   Downloaded derive_more v0.13.0
[00:03:05]   Downloaded tokio-current-thread v0.1.4
[00:03:05]   Downloaded quote v0.3.15
[00:03:05]   Downloaded futures-cpupool v0.1.8
[00:03:05]   Downloaded maplit v1.0.1
[00:03:05]   Downloaded rusty-fork v0.2.1
[00:03:05]   Downloaded hyper v0.12.25
[00:03:05]   Downloaded memoffset v0.2.1
---
[00:53:23]    Compiling openssl v0.10.16
[00:53:24]    Compiling maplit v1.0.1
[00:53:24]    Compiling fnv v1.0.6
[00:53:24]    Compiling num-traits v0.2.6
[00:53:24]    Compiling native-tls v0.2.2
[00:53:24]    Compiling try-lock v0.2.2
[00:53:24]    Compiling unicode-normalization v0.1.7
[00:53:24]    Compiling unicode-xid v0.0.4
[00:53:25]    Compiling bitflags v1.0.4
[00:53:25]    Compiling ucd-util v0.1.3
---
[00:54:06]    Compiling tokio-executor v0.1.6
[00:54:06]    Compiling want v0.0.6
[00:54:06]    Compiling rustc_version v0.2.3
[00:54:07]    Compiling pest_meta v2.1.0
[00:54:08] error: failed to run custom build command for `openssl-sys v0.9.40`
[00:54:08] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/build/openssl-sys-aaef693af933fbb3/build-script-main` (exit code: 101)
[00:54:08] --- stdout
[00:54:08] cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_LIB_DIR
[00:54:08] cargo:rerun-if-env-changed=OPENSSL_LIB_DIR
[00:54:08] cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_INCLUDE_DIR
[00:54:08] cargo:rerun-if-env-changed=OPENSSL_INCLUDE_DIR
[00:54:08] cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_DIR
[00:54:08] cargo:rerun-if-env-changed=OPENSSL_DIR
[00:54:08] run pkg_config fail: "Failed to run `\"pkg-config\" \"--libs\" \"--cflags\" \"openssl\"`: No such file or directory (os error 2)"
[00:54:08] --- stderr
[00:54:08] thread 'main' panicked at '
[00:54:08] 
[00:54:08] 
[00:54:08] Could not find directory of OpenSSL installation, and this `-sys` crate cannot
[00:54:08] proceed without this knowledge. If OpenSSL is installed and this crate had
[00:54:08] trouble finding it,  you can set the `OPENSSL_DIR` environment variable for the
[00:54:08] 
[00:54:08] 
[00:54:08] Make sure you also have the development packages of openssl installed.
[00:54:08] For example, `libssl-dev` on Ubuntu or `openssl-devel` on Fedora.
[00:54:08] 
[00:54:08] If you're in a situation where you think the directory *should* be found
[00:54:08] automatically, please open a bug at https://github.com/sfackler/rust-openssl
[00:54:08] and include information about your system as well as this message.
[00:54:08]     $HOST = x86_64-unknown-linux-gnu
[00:54:08]     $TARGET = x86_64-unknown-linux-gnu
[00:54:08]     openssl-sys = 0.9.40
[00:54:08] 
[00:54:08] 
[00:54:08] 
[00:54:08] It looks like you're compiling on Linux and also targeting Linux. Currently this
[00:54:08] requires the `pkg-config` utility to find OpenSSL but unfortunately `pkg-config`
[00:54:08] could not be found. If you have OpenSSL installed you can likely fix this by
[00:54:08] installing `pkg-config`.
[00:54:08] ', /cargo/registry/src/github.com-1ecc6299db9ec823/openssl-sys-0.9.40/build/main.rs:269:9
[00:54:08] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:54:08] 
[00:54:08] warning: build failed, waiting for other jobs to finish...
