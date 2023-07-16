plain
travis_time:end:05ecd7e0:start=1554745416288817703,finish=1554745499246443252,duration=82957625549
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:02:53]   Downloaded mime_guess v2.0.0-alpha.6
[00:02:53]   Downloaded num-traits v0.2.6
[00:02:53]   Downloaded syn v0.11.11
[00:02:53]   Downloaded winapi-build v0.1.1
[00:02:53]   Downloaded futures-cpupool v0.1.8
[00:02:53]   Downloaded error-chain v0.11.0
[00:02:53]   Downloaded handlebars v0.32.4
[00:02:53]   Downloaded failure v0.1.5
[00:02:53]   Downloaded rustc-ap-serialize v407.0.0
---
[00:51:59]    Compiling ucd-util v0.1.3
[00:52:00]    Compiling num-integer v0.1.39
[00:52:00]    Compiling regex v1.1.0
[00:52:00]    Compiling unicode-segmentation v1.2.1
[00:52:01]    Compiling try-lock v0.2.2
[00:52:01]    Compiling string v0.1.3
[00:52:01]    Compiling unicode-xid v0.0.4
[00:52:01]    Compiling native-tls v0.2.2
[00:52:01]    Compiling indexmap v1.0.2
---
[00:52:39]    Compiling tokio-executor v0.1.6
[00:52:39]    Compiling crossbeam-epoch v0.3.1
[00:52:39]    Compiling want v0.0.6
[00:52:41]    Compiling pest_meta v2.1.0
[00:52:42] error: failed to run custom build command for `openssl-sys v0.9.40`
[00:52:42] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/build/openssl-sys-aaef693af933fbb3/build-script-main` (exit code: 101)
[00:52:42] --- stdout
[00:52:42] cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_LIB_DIR
[00:52:42] cargo:rerun-if-env-changed=OPENSSL_LIB_DIR
[00:52:42] cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_INCLUDE_DIR
[00:52:42] cargo:rerun-if-env-changed=OPENSSL_INCLUDE_DIR
[00:52:42] cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_DIR
[00:52:42] cargo:rerun-if-env-changed=OPENSSL_DIR
[00:52:42] run pkg_config fail: "Failed to run `\"pkg-config\" \"--libs\" \"--cflags\" \"openssl\"`: No such file or directory (os error 2)"
[00:52:42] --- stderr
[00:52:42] thread 'main' panicked at '
[00:52:42] 
[00:52:42] 
[00:52:42] Could not find directory of OpenSSL installation, and this `-sys` crate cannot
[00:52:42] proceed without this knowledge. If OpenSSL is installed and this crate had
[00:52:42] trouble finding it,  you can set the `OPENSSL_DIR` environment variable for the
[00:52:42] 
[00:52:42] 
[00:52:42] Make sure you also have the development packages of openssl installed.
[00:52:42] For example, `libssl-dev` on Ubuntu or `openssl-devel` on Fedora.
[00:52:42] 
[00:52:42] If you're in a situation where you think the directory *should* be found
[00:52:42] automatically, please open a bug at https://github.com/sfackler/rust-openssl
[00:52:42] and include information about your system as well as this message.
[00:52:42]     $HOST = x86_64-unknown-linux-gnu
[00:52:42]     $TARGET = x86_64-unknown-linux-gnu
[00:52:42]     openssl-sys = 0.9.40
[00:52:42] 
[00:52:42] 
[00:52:42] 
[00:52:42] It looks like you're compiling on Linux and also targeting Linux. Currently this
[00:52:42] requires the `pkg-config` utility to find OpenSSL but unfortunately `pkg-config`
[00:52:42] could not be found. If you have OpenSSL installed you can likely fix this by
[00:52:42] installing `pkg-config`.
[00:52:42] ', /cargo/registry/src/github.com-1ecc6299db9ec823/openssl-sys-0.9.40/build/main.rs:269:9
[00:52:42] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:52:42] 
[00:52:42] warning: build failed, waiting for other jobs to finish...
---
travis_time:end:0bd4dff4:start=1554748679790492458,finish=1554748679797973938,duration=7481480
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0226f86c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0f033d00
$ dmesg | grep -i kill
