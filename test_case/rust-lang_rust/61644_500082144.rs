plain
[00:05:34]    Vendoring fortanix-sgx-abi v0.3.2 (/cargo/registry/src/github.com-1ecc6299db9ec823/fortanix-sgx-abi-0.3.2) to /checkout/obj/build/tmp/dist/rustc-1.37.0-dev-src/vendor/fortanix-sgx-abi
[00:05:34]    Vendoring fs2 v0.4.3 (/cargo/registry/src/github.com-1ecc6299db9ec823/fs2-0.4.3) to /checkout/obj/build/tmp/dist/rustc-1.37.0-dev-src/vendor/fs2
[00:05:34]    Vendoring fs_extra v1.1.0 (/cargo/registry/src/github.com-1ecc6299db9ec823/fs_extra-1.1.0) to /checkout/obj/build/tmp/dist/rustc-1.37.0-dev-src/vendor/fs_extra
[00:05:34]    Vendoring fst v0.3.0 (/cargo/registry/src/github.com-1ecc6299db9ec823/fst-0.3.0) to /checkout/obj/build/tmp/dist/rustc-1.37.0-dev-src/vendor/fst
[00:05:34]    Vendoring fuchsia-cprng v0.1.1 (/cargo/registry/src/github.com-1ecc6299db9ec823/fuchsia-cprng-0.1.1) to /checkout/obj/build/tmp/dist/rustc-1.37.0-dev-src/vendor/fuchsia-cprng
[00:05:34]    Vendoring fuchsia-zircon-sys v0.3.3 (/cargo/registry/src/github.com-1ecc6299db9ec823/fuchsia-zircon-sys-0.3.3) to /checkout/obj/build/tmp/dist/rustc-1.37.0-dev-src/vendor/fuchsia-zircon-sys
[00:05:34]    Vendoring futf v0.1.4 (/cargo/registry/src/github.com-1ecc6299db9ec823/futf-0.1.4) to /checkout/obj/build/tmp/dist/rustc-1.37.0-dev-src/vendor/futf
[00:05:34]    Vendoring futures v0.1.21 (/cargo/registry/src/github.com-1ecc6299db9ec823/futures-0.1.21) to /checkout/obj/build/tmp/dist/rustc-1.37.0-dev-src/vendor/futures
[00:05:34]    Vendoring fwdansi v1.0.1 (/cargo/registry/src/github.com-1ecc6299db9ec823/fwdansi-1.0.1) to /checkout/obj/build/tmp/dist/rustc-1.37.0-dev-src/vendor/fwdansi
---
[00:05:35]    Vendoring rand_pcg v0.1.1 (/cargo/registry/src/github.com-1ecc6299db9ec823/rand_pcg-0.1.1) to /checkout/obj/build/tmp/dist/rustc-1.37.0-dev-src/vendor/rand_pcg
[00:05:35]    Vendoring rand_xorshift v0.1.0 (/cargo/registry/src/github.com-1ecc6299db9ec823/rand_xorshift-0.1.0) to /checkout/obj/build/tmp/dist/rustc-1.37.0-dev-src/vendor/rand_xorshift
[00:05:35]    Vendoring rayon v1.0.1 (/cargo/registry/src/github.com-1ecc6299db9ec823/rayon-1.0.1) to /checkout/obj/build/tmp/dist/rustc-1.37.0-dev-src/vendor/rayon
[00:05:35]    Vendoring rayon-core v1.4.0 (/cargo/registry/src/github.com-1ecc6299db9ec823/rayon-core-1.4.0) to /checkout/obj/build/tmp/dist/rustc-1.37.0-dev-src/vendor/rayon-core
[00:05:35]    Vendoring rdrand v0.4.0 (/cargo/registry/src/github.com-1ecc6299db9ec823/rdrand-0.4.0) to /checkout/obj/build/tmp/dist/rustc-1.37.0-dev-src/vendor/rdrand
[00:05:35]    Vendoring redox_termios v0.1.1 (/cargo/registry/src/github.com-1ecc6299db9ec823/redox_termios-0.1.1) to /checkout/obj/build/tmp/dist/rustc-1.37.0-dev-src/vendor/redox_termios
[00:05:35]    Vendoring redox_users v0.3.0 (/cargo/registry/src/github.com-1ecc6299db9ec823/redox_users-0.3.0) to /checkout/obj/build/tmp/dist/rustc-1.37.0-dev-src/vendor/redox_users
[00:05:35]    Vendoring regex v0.2.11 (/cargo/registry/src/github.com-1ecc6299db9ec823/regex-0.2.11) to /checkout/obj/build/tmp/dist/rustc-1.37.0-dev-src/vendor/regex-0.2.11
[00:05:35]    Vendoring regex v1.1.6 (/cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.1.6) to /checkout/obj/build/tmp/dist/rustc-1.37.0-dev-src/vendor/regex
---
[00:15:29] * wrapping_iter_arith              lib      stable       1.14.0  
[00:15:29] * wrapping_neg                     lib      stable       1.10.0  
[00:15:29] * wrapping_next_power_of_two       lib      unstable     None    
[00:15:29] * wrapping_ref                     lib      stable       1.14.0  
[00:15:32] invalid license ISC in /checkout/obj/build/tmp/distcheck/src/../vendor/rdrand/Cargo.toml
[00:15:32] invalid license LICENSE in /checkout/obj/build/tmp/distcheck/src/../vendor/fuchsia-cprng/Cargo.toml
[00:15:32] some tidy checks failed
[00:15:32] 
[00:15:32] 
[00:15:32] command did not execute successfully: "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/tmp/distcheck/src" "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
[00:15:32] 
[00:15:32] 
[00:15:32] failed to run: /checkout/obj/build/tmp/distcheck/build/bootstrap/debug/bootstrap test
[00:15:32] Build completed unsuccessfully in 0:02:35
[00:15:32] Build completed unsuccessfully in 0:02:35
[00:15:32] make: *** [check] Error 1
[00:15:32] Makefile:48: recipe for target 'check' failed
[00:15:32] 
[00:15:32] command did not execute successfully: "make" "check"
[00:15:32] expected success, got: exit code: 2
[00:15:32] 
---
travis_time:end:10c83fba:start=1559957330877999397,finish=1559957330894977406,duration=16978009
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0d28299f
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:057e2999
travis_time:start:057e2999
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00bee6c7
$ dmesg | grep -i kill
