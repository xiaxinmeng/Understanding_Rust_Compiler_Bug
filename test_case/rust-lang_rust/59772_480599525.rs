plain
travis_time:end:042ef528:start=1554647268199977840,finish=1554647269067454301,duration=867476461
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:03:18]   Downloaded shell-escape v0.1.4
[00:03:18]   Downloaded native-tls v0.2.2
[00:03:18]   Downloaded openssl-sys v0.9.40
[00:03:18]   Downloaded macro-utils v0.1.2
[00:03:18]   Downloaded futures-cpupool v0.1.8
[00:03:18]   Downloaded either v1.5.0
[00:03:18]   Downloaded serde_ignored v0.0.4
[00:03:18]   Downloaded bufstream v0.1.4
[00:03:18]   Downloaded tokio-io v0.1.11
---
[00:53:51]    Compiling crc32fast v1.1.2
[00:53:52]    Compiling unicode-segmentation v1.2.1
[00:53:52]    Compiling ucd-util v0.1.3
[00:53:53]    Compiling unicode-xid v0.0.4
[00:53:53]    Compiling try-lock v0.2.2
[00:53:53]    Compiling indexmap v1.0.2
[00:53:54]    Compiling rayon-core v1.4.0
[00:53:54]    Compiling quick-error v1.2.2
[00:53:54]    Compiling openssl-probe v0.1.2
[00:53:55]    Compiling unicode-width v0.1.5
---
[00:54:31]    Compiling crossbeam-epoch v0.7.0
[00:54:31]    Compiling crossbeam-epoch v0.3.1
[00:54:33]    Compiling idna v0.1.5
[00:54:33]    Compiling pest_meta v2.1.0
[00:54:48] error: failed to run custom build command for `openssl-sys v0.9.40`
[00:54:48] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/build/openssl-sys-aaef693af933fbb3/build-script-main` (exit code: 101)
[00:54:48] --- stdout
[00:54:48] cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_LIB_DIR
[00:54:48] cargo:rerun-if-env-changed=OPENSSL_LIB_DIR
[00:54:48] cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_INCLUDE_DIR
[00:54:48] cargo:rerun-if-env-changed=OPENSSL_INCLUDE_DIR
[00:54:48] cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_DIR
[00:54:48] cargo:rerun-if-env-changed=OPENSSL_DIR
[00:54:48] run pkg_config fail: "Failed to run `\"pkg-config\" \"--libs\" \"--cflags\" \"openssl\"`: No such file or directory (os error 2)"
[00:54:48] --- stderr
[00:54:48] thread 'main' panicked at '
[00:54:48] 
[00:54:48] 
[00:54:48] Could not find directory of OpenSSL installation, and this `-sys` crate cannot
[00:54:48] proceed without this knowledge. If OpenSSL is installed and this crate had
[00:54:48] trouble finding it,  you can set the `OPENSSL_DIR` environment variable for the
[00:54:48] 
[00:54:48] 
[00:54:48] Make sure you also have the development packages of openssl installed.
[00:54:48] For example, `libssl-dev` on Ubuntu or `openssl-devel` on Fedora.
[00:54:48] 
[00:54:48] If you're in a situation where you think the directory *should* be found
[00:54:48] automatically, please open a bug at https://github.com/sfackler/rust-openssl
[00:54:48] and include information about your system as well as this message.
[00:54:48]     $HOST = x86_64-unknown-linux-gnu
[00:54:48]     $TARGET = x86_64-unknown-linux-gnu
[00:54:48]     openssl-sys = 0.9.40
[00:54:48] 
[00:54:48] 
[00:54:48] 
[00:54:48] It looks like you're compiling on Linux and also targeting Linux. Currently this
[00:54:48] requires the `pkg-config` utility to find OpenSSL but unfortunately `pkg-config`
[00:54:48] could not be found. If you have OpenSSL installed you can likely fix this by
[00:54:48] installing `pkg-config`.
[00:54:48] ', /cargo/registry/src/github.com-1ecc6299db9ec823/openssl-sys-0.9.40/build/main.rs:269:9
[00:54:48] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:54:48] 
[00:54:48] warning: build failed, waiting for other jobs to finish...
