plain
travis_time:end:0863f68e:start=1554755864533140631,finish=1554755951639292647,duration=87106152016
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:03:27]   Downloaded error-chain v0.11.0
[00:03:27]   Downloaded adler32 v1.0.3
[00:03:27]   Downloaded reqwest v0.9.11
[00:03:27]   Downloaded precomputed-hash v0.1.1
[00:03:27]   Downloaded futures-cpupool v0.1.8
[00:03:27]   Downloaded toml v0.5.0
[00:03:27]   Downloaded textwrap v0.10.0
[00:03:28]   Downloaded aho-corasick v0.6.9
[00:03:28]   Downloaded rustc_version v0.2.3
---
[00:56:57]    Compiling precomputed-hash v0.1.1
[00:56:57]    Compiling matches v0.1.8
[00:56:57]    Compiling maplit v1.0.1
[00:56:57]    Compiling unicode-xid v0.0.4
[00:56:57]    Compiling try-lock v0.2.2
[00:56:57]    Compiling string v0.1.3
[00:56:57]    Compiling ucd-util v0.1.3
[00:56:57]    Compiling unicode-segmentation v1.2.1
[00:56:58]    Compiling indexmap v1.0.2
---
[00:57:42]    Compiling crossbeam-epoch v0.3.1
[00:57:44]    Compiling syn v0.11.11
[00:57:44]    Compiling idna v0.1.5
[00:57:45]    Compiling pest_meta v2.1.0
[00:58:01] error: failed to run custom build command for `openssl-sys v0.9.40`
[00:58:01] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/build/openssl-sys-aaef693af933fbb3/build-script-main` (exit code: 101)
[00:58:01] --- stdout
[00:58:01] cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_LIB_DIR
[00:58:01] cargo:rerun-if-env-changed=OPENSSL_LIB_DIR
[00:58:01] cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_INCLUDE_DIR
[00:58:01] cargo:rerun-if-env-changed=OPENSSL_INCLUDE_DIR
[00:58:01] cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_DIR
[00:58:01] cargo:rerun-if-env-changed=OPENSSL_DIR
[00:58:01] run pkg_config fail: "Failed to run `\"pkg-config\" \"--libs\" \"--cflags\" \"openssl\"`: No such file or directory (os error 2)"
[00:58:01] --- stderr
[00:58:01] thread 'main' panicked at '
[00:58:01] 
[00:58:01] 
[00:58:01] Could not find directory of OpenSSL installation, and this `-sys` crate cannot
[00:58:01] proceed without this knowledge. If OpenSSL is installed and this crate had
[00:58:01] trouble finding it,  you can set the `OPENSSL_DIR` environment variable for the
[00:58:01] 
[00:58:01] 
[00:58:01] Make sure you also have the development packages of openssl installed.
[00:58:01] For example, `libssl-dev` on Ubuntu or `openssl-devel` on Fedora.
[00:58:01] 
[00:58:01] If you're in a situation where you think the directory *should* be found
[00:58:01] automatically, please open a bug at https://github.com/sfackler/rust-openssl
[00:58:01] and include information about your system as well as this message.
[00:58:01]     $HOST = x86_64-unknown-linux-gnu
[00:58:01]     $TARGET = x86_64-unknown-linux-gnu
[00:58:01]     openssl-sys = 0.9.40
[00:58:01] 
[00:58:01] 
[00:58:01] 
[00:58:01] It looks like you're compiling on Linux and also targeting Linux. Currently this
[00:58:01] requires the `pkg-config` utility to find OpenSSL but unfortunately `pkg-config`
[00:58:01] could not be found. If you have OpenSSL installed you can likely fix this by
[00:58:01] installing `pkg-config`.
[00:58:01] ', /cargo/registry/src/github.com-1ecc6299db9ec823/openssl-sys-0.9.40/build/main.rs:269:9
[00:58:01] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:58:01] 
[00:58:01] warning: build failed, waiting for other jobs to finish...
---
travis_time:end:158e8312:start=1554759450336666824,finish=1554759450341691837,duration=5025013
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00f338c4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "
