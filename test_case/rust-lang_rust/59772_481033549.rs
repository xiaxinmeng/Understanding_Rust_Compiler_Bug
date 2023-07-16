plain
travis_time:start:1167d10d
$ travis_apt_get_update
travis_time:end:1167d10d:start=1554759803898258306,finish=1554759814081118095,duration=10182859789
travis_time:start:329272bf
$ sudo -E apt-get -yq --no-install-suggests --no-install-recommends $(travis_apt_get_options) install gdb openssl libssl-dev libssh-dev
Building dependency tree...
Reading state information...
The following additional packages will be installed:
  libbabeltrace-ctf1 libbabeltrace1 libssl1.0.0
---
travis_time:end:1aee00e0:start=1554759829320789467,finish=1554759903348477524,duration=74027688057
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:03:12]   Downloaded rls-data v0.18.2
[00:03:12]   Downloaded ordslice v0.3.0
[00:03:12]   Downloaded crossbeam-utils v0.2.2
[00:03:12]   Downloaded num-integer v0.1.39
[00:03:12]   Downloaded futures-cpupool v0.1.8
[00:03:12]   Downloaded pest_derive v2.1.0
[00:03:13]   Downloaded httparse v1.3.3
[00:03:13]   Downloaded pretty_assertions v0.5.1
[00:03:13]   Downloaded memoffset v0.2.1
---
[00:50:50]    Compiling ucd-util v0.1.3
[00:50:50]    Compiling unicode-normalization v0.1.7
[00:50:51]    Compiling native-tls v0.2.2
[00:50:51]    Compiling unicode-segmentation v1.2.1
[00:50:51]    Compiling try-lock v0.2.2
[00:50:52]    Compiling failure_derive v0.1.5
[00:50:52]    Compiling openssl-probe v0.1.2
[00:50:53]    Compiling unicode-width v0.1.5
[00:50:53]    Compiling percent-encoding v1.0.1
---
[00:51:28]    Compiling want v0.0.6
[00:51:29]    Compiling idna v0.1.5
[00:51:30]    Compiling syn v0.11.11
[00:51:31]    Compiling pest_meta v2.1.0
[00:51:46] error: failed to run custom build command for `openssl-sys v0.9.40`
[00:51:46] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/build/openssl-sys-aaef693af933fbb3/build-script-main` (exit code: 101)
[00:51:46] --- stdout
[00:51:46] cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_LIB_DIR
[00:51:46] cargo:rerun-if-env-changed=OPENSSL_LIB_DIR
[00:51:46] cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_INCLUDE_DIR
[00:51:46] cargo:rerun-if-env-changed=OPENSSL_INCLUDE_DIR
[00:51:46] cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_DIR
[00:51:46] cargo:rerun-if-env-changed=OPENSSL_DIR
[00:51:46] run pkg_config fail: "Failed to run `\"pkg-config\" \"--libs\" \"--cflags\" \"openssl\"`: No such file or directory (os error 2)"
[00:51:46] --- stderr
[00:51:46] thread 'main' panicked at '
[00:51:46] 
[00:51:46] 
[00:51:46] Could not find directory of OpenSSL installation, and this `-sys` crate cannot
[00:51:46] proceed without this knowledge. If OpenSSL is installed and this crate had
[00:51:46] trouble finding it,  you can set the `OPENSSL_DIR` environment variable for the
[00:51:46] 
[00:51:46] 
[00:51:46] Make sure you also have the development packages of openssl installed.
[00:51:46] For example, `libssl-dev` on Ubuntu or `openssl-devel` on Fedora.
[00:51:46] 
[00:51:46] If you're in a situation where you think the directory *should* be found
[00:51:46] automatically, please open a bug at https://github.com/sfackler/rust-openssl
[00:51:46] and include information about your system as well as this message.
[00:51:46]     $HOST = x86_64-unknown-linux-gnu
[00:51:46]     $TARGET = x86_64-unknown-linux-gnu
[00:51:46]     openssl-sys = 0.9.40
[00:51:46] 
[00:51:46] 
[00:51:46] 
[00:51:46] It looks like you're compiling on Linux and also targeting Linux. Currently this
[00:51:46] requires the `pkg-config` utility to find OpenSSL but unfortunately `pkg-config`
[00:51:46] could not be found. If you have OpenSSL installed you can likely fix this by
[00:51:46] installing `pkg-config`.
[00:51:46] ', /cargo/registry/src/github.com-1ecc6299db9ec823/openssl-sys-0.9.40/build/main.rs:269:9
[00:51:46] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:51:46] 
[00:51:46] warning: build failed, waiting for other jobs to finish...
---
travis_time:end:09302ad3:start=1554763027012649814,finish=1554763027017176652,duration=4526838
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:001c71df
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f 
