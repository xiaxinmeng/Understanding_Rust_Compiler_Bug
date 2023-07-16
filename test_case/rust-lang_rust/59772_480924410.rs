plain
travis_time:end:0c027386:start=1554740627339463798,finish=1554740850838269246,duration=223498805448
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:02:47]   Downloaded termcolor v1.0.4
[00:02:47]   Downloaded itertools v0.7.8
[00:02:47]   Downloaded lazycell v1.2.1
[00:02:47]   Downloaded bit-set v0.5.0
[00:02:47]   Downloaded futures-cpupool v0.1.8
[00:02:47]   Downloaded slab v0.4.2
[00:02:47]   Downloaded signal-hook v0.1.7
[00:02:47]   Downloaded block-buffer v0.3.3
[00:02:47]   Downloaded proptest v0.9.2
---
[00:53:12]    Compiling bitflags v1.0.4
[00:53:12]    Compiling unicode-normalization v0.1.7
[00:53:13]    Compiling unicode-xid v0.0.4
[00:53:13]    Compiling rustc-demangle v0.1.10
[00:53:13]    Compiling try-lock v0.2.2
[00:53:15]    Compiling regex v1.1.0
[00:53:15]    Compiling unicode-segmentation v1.2.1
[00:53:15]    Compiling ucd-util v0.1.3
[00:53:16]    Compiling indexmap v1.0.2
---
[00:53:54]    Compiling rustc_version v0.2.3
[00:53:55]    Compiling pest_meta v2.1.0
[00:53:56]    Compiling idna v0.1.5
[00:54:10]    Compiling syn v0.11.11
[00:54:10] error: failed to run custom build command for `openssl-sys v0.9.40`
[00:54:10] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/build/openssl-sys-aaef693af933fbb3/build-script-main` (exit code: 101)
[00:54:10] --- stdout
[00:54:10] cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_LIB_DIR
[00:54:10] cargo:rerun-if-env-changed=OPENSSL_LIB_DIR
[00:54:10] cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_INCLUDE_DIR
[00:54:10] cargo:rerun-if-env-changed=OPENSSL_INCLUDE_DIR
[00:54:10] cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_DIR
[00:54:10] cargo:rerun-if-env-changed=OPENSSL_DIR
[00:54:10] run pkg_config fail: "Failed to run `\"pkg-config\" \"--libs\" \"--cflags\" \"openssl\"`: No such file or directory (os error 2)"
[00:54:10] --- stderr
[00:54:10] thread 'main' panicked at '
[00:54:10] 
[00:54:10] 
[00:54:10] Could not find directory of OpenSSL installation, and this `-sys` crate cannot
[00:54:10] proceed without this knowledge. If OpenSSL is installed and this crate had
[00:54:10] trouble finding it,  you can set the `OPENSSL_DIR` environment variable for the
[00:54:10] 
[00:54:10] 
[00:54:10] Make sure you also have the development packages of openssl installed.
[00:54:10] For example, `libssl-dev` on Ubuntu or `openssl-devel` on Fedora.
[00:54:10] 
[00:54:10] If you're in a situation where you think the directory *should* be found
[00:54:10] automatically, please open a bug at https://github.com/sfackler/rust-openssl
[00:54:10] and include information about your system as well as this message.
[00:54:10]     $HOST = x86_64-unknown-linux-gnu
[00:54:10]     $TARGET = x86_64-unknown-linux-gnu
[00:54:10]     openssl-sys = 0.9.40
[00:54:10] 
[00:54:10] 
[00:54:10] 
[00:54:10] It looks like you're compiling on Linux and also targeting Linux. Currently this
[00:54:10] requires the `pkg-config` utility to find OpenSSL but unfortunately `pkg-config`
[00:54:10] could not be found. If you have OpenSSL installed you can likely fix this by
[00:54:10] installing `pkg-config`.
[00:54:10] ', /cargo/registry/src/github.com-1ecc6299db9ec823/openssl-sys-0.9.40/build/main.rs:269:9
[00:54:10] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:54:10] 
[00:54:10] warning: build failed, waiting for other jobs to finish...
---
travis_time:end:03565b78:start=1554744118056836570,finish=1554744118061914953,duration=5078383
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0280421f
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b4728b2
travis_time:start:0b4728b2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b71e90b
$ dmesg | grep -i kill
