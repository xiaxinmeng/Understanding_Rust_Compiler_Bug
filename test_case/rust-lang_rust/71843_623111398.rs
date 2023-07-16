plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0012ab3e-1442-4f26-9a7c-d303a67707fa.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71843/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71843/merge:refs/remotes/pull/71843/merge
---
 ---> f7353ccad5b1
Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
 ---> Using cache
 ---> ed38efbaa060
Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
 ---> c5008ef7ae8e
Successfully built c5008ef7ae8e
Successfully tagged rust-ci:latest
Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
---
Checking clippy artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Checking std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling cc v1.0.50
    Checking core v0.0.0 (/checkout/src/libcore)
error: expected one of `!`, `(`, `)`, `+`, `,`, `::`, or `<`, found `fetch_order`
    --> src/libcore/sync/atomic.rs:1846:40
1199 | / macro_rules! atomic_int {
1199 | / macro_rules! atomic_int {
1200 | |     ($cfg_cas:meta,
1201 | |      $stable:meta,
1202 | |      $stable_cxchg:meta,
1845 | |                                        set_order: Ordering
     | |                                                           -
     | |                                                           |
     | |                                                           expected one of 7 possible tokens
---
2005 | |     }
2006 | | }
     | |_- in this expansion of `atomic_int!`
...
2009 | / atomic_int! {
2010 | |     cfg(target_has_atomic = "8"),
2011 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
2012 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2024 | |     i8 AtomicI8 ATOMIC_I8_INIT
     | |_- in this macro invocation


error: expected one of `!`, `(`, `)`, `+`, `,`, `::`, or `<`, found `fetch_order`
    --> src/libcore/sync/atomic.rs:1846:40
1199 | / macro_rules! atomic_int {
1199 | / macro_rules! atomic_int {
1200 | |     ($cfg_cas:meta,
1201 | |      $stable:meta,
1202 | |      $stable_cxchg:meta,
1845 | |                                        set_order: Ordering
     | |                                                           -
     | |                                                           |
     | |                                                           expected one of 7 possible tokens
---
2005 | |     }
2006 | | }
     | |_- in this expansion of `atomic_int!`
...
2027 | / atomic_int! {
2028 | |     cfg(target_has_atomic = "8"),
2029 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
2030 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2042 | |     u8 AtomicU8 ATOMIC_U8_INIT
     | |_- in this macro invocation


error: expected one of `!`, `(`, `)`, `+`, `,`, `::`, or `<`, found `fetch_order`
    --> src/libcore/sync/atomic.rs:1846:40
1199 | / macro_rules! atomic_int {
1199 | / macro_rules! atomic_int {
1200 | |     ($cfg_cas:meta,
1201 | |      $stable:meta,
1202 | |      $stable_cxchg:meta,
1845 | |                                        set_order: Ordering
     | |                                                           -
     | |                                                           |
     | |                                                           expected one of 7 possible tokens
---
2005 | |     }
2006 | | }
     | |_- in this expansion of `atomic_int!`
...
2045 | / atomic_int! {
2046 | |     cfg(target_has_atomic = "16"),
2047 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
2048 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2060 | |     i16 AtomicI16 ATOMIC_I16_INIT
     | |_- in this macro invocation


error: expected one of `!`, `(`, `)`, `+`, `,`, `::`, or `<`, found `fetch_order`
    --> src/libcore/sync/atomic.rs:1846:40
1199 | / macro_rules! atomic_int {
1199 | / macro_rules! atomic_int {
1200 | |     ($cfg_cas:meta,
1201 | |      $stable:meta,
1202 | |      $stable_cxchg:meta,
1845 | |                                        set_order: Ordering
     | |                                                           -
     | |                                                           |
     | |                                                           expected one of 7 possible tokens
---
2005 | |     }
2006 | | }
     | |_- in this expansion of `atomic_int!`
...
2063 | / atomic_int! {
2064 | |     cfg(target_has_atomic = "16"),
2065 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
2066 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2078 | |     u16 AtomicU16 ATOMIC_U16_INIT
     | |_- in this macro invocation


error: expected one of `!`, `(`, `)`, `+`, `,`, `::`, or `<`, found `fetch_order`
    --> src/libcore/sync/atomic.rs:1846:40
1199 | / macro_rules! atomic_int {
1199 | / macro_rules! atomic_int {
1200 | |     ($cfg_cas:meta,
1201 | |      $stable:meta,
1202 | |      $stable_cxchg:meta,
1845 | |                                        set_order: Ordering
     | |                                                           -
     | |                                                           |
     | |                                                           expected one of 7 possible tokens
---
2005 | |     }
2006 | | }
     | |_- in this expansion of `atomic_int!`
...
2081 | / atomic_int! {
2082 | |     cfg(target_has_atomic = "32"),
2083 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
2084 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2096 | |     i32 AtomicI32 ATOMIC_I32_INIT
     | |_- in this macro invocation


error: expected one of `!`, `(`, `)`, `+`, `,`, `::`, or `<`, found `fetch_order`
    --> src/libcore/sync/atomic.rs:1846:40
1199 | / macro_rules! atomic_int {
1199 | / macro_rules! atomic_int {
1200 | |     ($cfg_cas:meta,
1201 | |      $stable:meta,
1202 | |      $stable_cxchg:meta,
1845 | |                                        set_order: Ordering
     | |                                                           -
     | |                                                           |
     | |                                                           expected one of 7 possible tokens
---
2005 | |     }
2006 | | }
     | |_- in this expansion of `atomic_int!`
...
2099 | / atomic_int! {
2100 | |     cfg(target_has_atomic = "32"),
2101 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
2102 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2114 | |     u32 AtomicU32 ATOMIC_U32_INIT
     | |_- in this macro invocation


error: expected one of `!`, `(`, `)`, `+`, `,`, `::`, or `<`, found `fetch_order`
    --> src/libcore/sync/atomic.rs:1846:40
1199 | / macro_rules! atomic_int {
1199 | / macro_rules! atomic_int {
1200 | |     ($cfg_cas:meta,
1201 | |      $stable:meta,
1202 | |      $stable_cxchg:meta,
1845 | |                                        set_order: Ordering
     | |                                                           -
     | |                                                           |
     | |                                                           expected one of 7 possible tokens
---
2005 | |     }
2006 | | }
     | |_- in this expansion of `atomic_int!`
...
2117 | / atomic_int! {
2118 | |     cfg(target_has_atomic = "64"),
2119 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
2120 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2132 | |     i64 AtomicI64 ATOMIC_I64_INIT
     | |_- in this macro invocation


error: expected one of `!`, `(`, `)`, `+`, `,`, `::`, or `<`, found `fetch_order`
    --> src/libcore/sync/atomic.rs:1846:40
1199 | / macro_rules! atomic_int {
1199 | / macro_rules! atomic_int {
1200 | |     ($cfg_cas:meta,
1201 | |      $stable:meta,
1202 | |      $stable_cxchg:meta,
1845 | |                                        set_order: Ordering
     | |                                                           -
     | |                                                           |
     | |                                                           expected one of 7 possible tokens
---
2005 | |     }
2006 | | }
     | |_- in this expansion of `atomic_int!`
...
2135 | / atomic_int! {
2136 | |     cfg(target_has_atomic = "64"),
2137 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
2138 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
...    |
2150 | |     u64 AtomicU64 ATOMIC_U64_INIT
     | |_- in this macro invocation


error: expected one of `!`, `(`, `)`, `+`, `,`, `::`, or `<`, found `fetch_order`
    --> src/libcore/sync/atomic.rs:1846:40
1199 | / macro_rules! atomic_int {
1199 | / macro_rules! atomic_int {
1200 | |     ($cfg_cas:meta,
1201 | |      $stable:meta,
1202 | |      $stable_cxchg:meta,
1845 | |                                        set_order: Ordering
     | |                                                           -
     | |                                                           |
     | |                                                           expected one of 7 possible tokens
---
2005 | |     }
2006 | | }
     | |_- in this expansion of `atomic_int!`
...
2210 | / atomic_int! {
2211 | |     cfg(target_has_atomic = "ptr"),
2212 | |     stable(feature = "rust1", since = "1.0.0"),
2213 | |     stable(feature = "extended_compare_and_swap", since = "1.10.0"),
2225 | |     isize AtomicIsize ATOMIC_ISIZE_INIT
2226 | | }
     | |_- in this macro invocation


error: expected one of `!`, `(`, `)`, `+`, `,`, `::`, or `<`, found `fetch_order`
    --> src/libcore/sync/atomic.rs:1846:40
1199 | / macro_rules! atomic_int {
1199 | / macro_rules! atomic_int {
1200 | |     ($cfg_cas:meta,
1201 | |      $stable:meta,
1202 | |      $stable_cxchg:meta,
1845 | |                                        set_order: Ordering
     | |                                                           -
     | |                                                           |
     | |                                                           expected one of 7 possible tokens
---
2005 | |     }
2006 | | }
     | |_- in this expansion of `atomic_int!`
...
2228 | / atomic_int! {
2229 | |     cfg(target_has_atomic = "ptr"),
2230 | |     stable(feature = "rust1", since = "1.0.0"),
2231 | |     stable(feature = "extended_compare_and_swap", since = "1.10.0"),
2243 | |     usize AtomicUsize ATOMIC_USIZE_INIT
2244 | | }
     | |_- in this macro invocation

---
  local time: Sun May  3 13:35:35 UTC 2020
  network time: Sun, 03 May 2020 13:35:36 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71843/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71843/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (6119) (python)
##[section]Finishing: Finalize Job
