plain
travis_time:start:085c1b2d
$ travis_apt_get_update
travis_time:end:085c1b2d:start=1554764240467227335,finish=1554764249126462810,duration=8659235475
travis_time:start:12a5e5bc
$ sudo -E apt-get -yq --no-install-suggests --no-install-recommends $(travis_apt_get_options) install gdb openssl libssl-dev libssh-dev
Building dependency tree...
Reading state information...
The following additional packages will be installed:
  libbabeltrace-ctf1 libbabeltrace1 libssl1.0.0
---
travis_time:end:1a824ca4:start=1554764265416124486,finish=1554764349222764763,duration=83806640277
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:end:06fd807b:start=1554764355710141742,finish=1554764355716479483,duration=6337741
travis_fold:end:before_install.2
travis_fold:start:before_install.3
travis_time:start:00664c1a
$ if [ "$TRAVIS_OS_NAME" = linux ]; then echo '{"ipv6":true,"fixed-cidr-v6":"fd9a:8454:6789:13f7::/64"}' | sudo tee /etc/docker/daemon.json; sudo service docker restart; sudo apt-get install pkg-config --reinstall; fi
Reading package lists... 0%
Reading package lists... 100%
Reading package lists... Done
Building dependency tree... 0%
---
Need to get 45.0 kB of archives.
After this operation, 0 B of additional disk space will be used.
0% [Working]
Get:1 http://us-east-1.ec2.archive.ubuntu.com/ubuntu xenial/main amd64 pkg-config amd64 0.29.1-0ubuntu1 [45.0 kB]
24% [1 pkg-config 13.7 kB/45.0 kB 30%]
100% [Working]
(Reading database ... 
(Reading database ... 5%
(Reading database ... 10%
(Reading database ... 15%
---
[00:02:51]   Downloaded remove_dir_all v0.5.1
[00:02:51]   Downloaded tokio-udp v0.1.3
[00:02:51]   Downloaded handlebars v1.1.0
[00:02:51]   Downloaded signal-hook v0.1.7
[00:02:51]   Downloaded futures-cpupool v0.1.8
[00:02:51]   Downloaded fake-simd v0.1.2
[00:02:51]   Downloaded fuchsia-zircon-sys v0.3.3
[00:02:51]   Downloaded rustc-ap-rustc_target v407.0.0
[00:02:52]   Downloaded core-foundation-sys v0.6.2
---
[00:54:21]    Compiling unicode-xid v0.0.4
[00:54:21]    Compiling unicode-normalization v0.1.7
[00:54:21]    Compiling crc32fast v1.1.2
[00:54:21]    Compiling unicode-segmentation v1.2.1
[00:54:22]    Compiling try-lock v0.2.2
[00:54:22]    Compiling indexmap v1.0.2
[00:54:22]    Compiling encoding_rs v0.8.17
[00:54:22]    Compiling regex v0.2.11
[00:54:23]    Compiling quick-error v1.2.2
[00:54:23]    Compiling same-file v1.0.4
---
[00:55:01]    Compiling crossbeam-epoch v0.3.1
[00:55:04]    Compiling idna v0.1.5
[00:55:04]    Compiling pest_meta v2.1.0
[00:55:05]    Compiling syn v0.11.11
[00:55:21] error: failed to run custom build command for `openssl-sys v0.9.40`
[00:55:21] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/build/openssl-sys-aaef693af933fbb3/build-script-main` (exit code: 101)
[00:55:21] --- stdout
[00:55:21] cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_LIB_DIR
[00:55:21] cargo:rerun-if-env-changed=OPENSSL_LIB_DIR
[00:55:21] cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_INCLUDE_DIR
[00:55:21] cargo:rerun-if-env-changed=OPENSSL_INCLUDE_DIR
[00:55:21] cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_DIR
[00:55:21] cargo:rerun-if-env-changed=OPENSSL_DIR
[00:55:21] run pkg_config fail: "Failed to run `\"pkg-config\" \"--libs\" \"--cflags\" \"openssl\"`: No such file or directory (os error 2)"
[00:55:21] --- stderr
[00:55:21] thread 'main' panicked at '
[00:55:21] 
[00:55:21] 
[00:55:21] Could not find directory of OpenSSL installation, and this `-sys` crate cannot
[00:55:21] proceed without this knowledge. If OpenSSL is installed and this crate had
[00:55:21] trouble finding it,  you can set the `OPENSSL_DIR` environment variable for the
[00:55:21] 
[00:55:21] 
[00:55:21] Make sure you also have the development packages of openssl installed.
[00:55:21] For example, `libssl-dev` on Ubuntu or `openssl-devel` on Fedora.
[00:55:21] 
[00:55:21] If you're in a situation where you think the directory *should* be found
[00:55:21] automatically, please open a bug at https://github.com/sfackler/rust-openssl
[00:55:21] and include information about your system as well as this message.
[00:55:21]     $HOST = x86_64-unknown-linux-gnu
[00:55:21]     $TARGET = x86_64-unknown-linux-gnu
[00:55:21]     openssl-sys = 0.9.40
[00:55:21] 
[00:55:21] 
[00:55:21] 
[00:55:21] It looks like you're compiling on Linux and also targeting Linux. Currently this
[00:55:21] requires the `pkg-config` utility to find OpenSSL but unfortunately `pkg-config`
[00:55:21] could not be found. If you have OpenSSL installed you can likely fix this by
[00:55:21] installing `pkg-config`.
[00:55:21] ', /cargo/registry/src/github.com-1ecc6299db9ec823/openssl-sys-0.9.40/build/main.rs:269:9
[00:55:21] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:55:21] 
[00:55:21] warning: build failed, waiting for other jobs to finish...
---
travis_time:end:1d0f51aa:start=1554767693210885160,finish=1554767693216636648,duration=5751488
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:032b2522
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04715bde
travis_time:start:04715bde
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:015fff86
$ dmesg | grep -i kill
