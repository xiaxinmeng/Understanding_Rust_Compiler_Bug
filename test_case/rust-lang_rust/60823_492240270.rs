plain
travis_time:end:03fb089e:start=1557836502830192823,finish=1557836503564876034,duration=734683211
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:11:22] ...........iiiii.................................................................................... 1100/5530
[01:11:25] .................................................................................................... 1200/5530
[01:11:27] .................................................................................................... 1300/5530
[01:11:30] .................................................................................................... 1400/5530
[01:11:33] ..................................................................F................................. 1500/5530
[01:11:39] .............................i...................................................................... 1700/5530
[01:11:43] .................................................................................................... 1800/5530
[01:11:47] .................................................................................................... 1900/5530
[01:11:50] .................................................................................................... 2000/5530
---
[01:14:10] 1154 
[01:14:10] 1155 warning: unused attribute
[01:14:10] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:55:1
[01:14:10] -    |
[01:14:10] - LL | #![no_mangle]
[01:14:10] - 
[01:14:10] - warning: unused attribute
[01:14:10] 1162   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:56:1
[01:14:10] 1163    |
[01:14:10] 1163    |
[01:14:10] 1164 LL | #![no_link]
[01:14:10] 
[01:14:10] The actual stderr differed from the expected stderr.
[01:14:10] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs/issue-43106-gating-of-builtin-attrs.stderr
[01:14:10] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs/issue-43106-gating-of-builtin-attrs.stderr
[01:14:10] To update references, rerun the tests and pass the `--bless` flag
[01:14:10] To only update this specific test, also pass `--test-args feature-gate/issue-43106-gating-of-builtin-attrs.rs`
[01:14:10] error: 1 errors occurred comparing output.
[01:14:10] status: exit code: 1
[01:14:10] status: exit code: 1
[01:14:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs/auxiliary" "-A" "unused"
[01:14:10] ------------------------------------------
[01:14:10] 
[01:14:10] ------------------------------------------
[01:14:10] stderr:
[01:14:10] stderr:
[01:14:10] ------------------------------------------
[01:14:10] warning: unknown lint: `x5400`
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:38:9
[01:14:10]    |
[01:14:10] LL | #![warn(x5400)] //~ WARN unknown lint: `x5400`
[01:14:10]    |
[01:14:10] note: lint level defined here
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:33:28
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL | #![warn(unused_attributes, unknown_lints)]
[01:14:10]    |                            ^^^^^^^^^^^^^
[01:14:10] 
[01:14:10] warning: unknown lint: `x5300`
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:39:10
[01:14:10]    |
[01:14:10] LL | #![allow(x5300)] //~ WARN unknown lint: `x5300`
[01:14:10] 
[01:14:10] warning: unknown lint: `x5200`
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:40:11
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL | #![forbid(x5200)] //~ WARN unknown lint: `x5200`
[01:14:10] 
[01:14:10] warning: unknown lint: `x5100`
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:41:9
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL | #![deny(x5100)] //~ WARN unknown lint: `x5100`
[01:14:10] 
[01:14:10] warning: unknown lint: `x5400`
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:99:8
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL | #[warn(x5400)]
[01:14:10]    |        ^^^^^
[01:14:10] 
[01:14:10] warning: unknown lint: `x5400`
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:102:25
[01:14:10]    |
[01:14:10] LL |     mod inner { #![warn(x5400)] }
[01:14:10] 
[01:14:10] warning: unknown lint: `x5400`
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:105:12
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[warn(x5400)] fn f() { }
[01:14:10] 
[01:14:10] warning: unknown lint: `x5400`
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:108:12
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[warn(x5400)] struct S;
[01:14:10] 
[01:14:10] warning: unknown lint: `x5400`
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:111:12
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[warn(x5400)] type T = S;
[01:14:10] 
[01:14:10] warning: unknown lint: `x5400`
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:114:12
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[warn(x5400)] impl S { }
[01:14:10] 
[01:14:10] warning: unknown lint: `x5300`
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:118:9
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL | #[allow(x5300)]
[01:14:10]    |         ^^^^^
[01:14:10] 
[01:14:10] warning: unknown lint: `x5300`
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:121:26
[01:14:10]    |
[01:14:10] LL |     mod inner { #![allow(x5300)] }
[01:14:10] 
[01:14:10] warning: unknown lint: `x5300`
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:124:13
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[allow(x5300)] fn f() { }
[01:14:10] 
[01:14:10] warning: unknown lint: `x5300`
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:127:13
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[allow(x5300)] struct S;
[01:14:10] 
[01:14:10] warning: unknown lint: `x5300`
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:130:13
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[allow(x5300)] type T = S;
[01:14:10] 
[01:14:10] warning: unknown lint: `x5300`
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:133:13
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[allow(x5300)] impl S { }
[01:14:10] 
[01:14:10] warning: unknown lint: `x5200`
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:137:10
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL | #[forbid(x5200)]
[01:14:10] 
[01:14:10] warning: unknown lint: `x5200`
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:140:27
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     mod inner { #![forbid(x5200)] }
[01:14:10] 
[01:14:10] warning: unknown lint: `x5200`
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:143:14
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[forbid(x5200)] fn f() { }
[01:14:10] 
[01:14:10] warning: unknown lint: `x5200`
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:146:14
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[forbid(x5200)] struct S;
[01:14:10] 
[01:14:10] warning: unknown lint: `x5200`
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:149:14
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[forbid(x5200)] type T = S;
[01:14:10] 
[01:14:10] warning: unknown lint: `x5200`
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:152:14
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[forbid(x5200)] impl S { }
[01:14:10] 
[01:14:10] warning: unknown lint: `x5100`
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:156:8
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL | #[deny(x5100)]
[01:14:10]    |        ^^^^^
[01:14:10] 
[01:14:10] warning: unknown lint: `x5100`
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:159:25
[01:14:10]    |
[01:14:10] LL |     mod inner { #![deny(x5100)] }
[01:14:10] 
[01:14:10] warning: unknown lint: `x5100`
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:162:12
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[deny(x5100)] fn f() { }
[01:14:10] 
[01:14:10] warning: unknown lint: `x5100`
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:165:12
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[deny(x5100)] struct S;
[01:14:10] 
[01:14:10] warning: unknown lint: `x5100`
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:168:12
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[deny(x5100)] type T = S;
[01:14:10] 
[01:14:10] warning: unknown lint: `x5100`
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:171:12
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[deny(x5100)] impl S { }
[01:14:10] 
[01:14:10] warning: macro_escape is a deprecated synonym for macro_use
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:455:1
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL | #[macro_escape]
[01:14:10]    | ^^^^^^^^^^^^^^^
[01:14:10] 
[01:14:10] warning: macro_escape is a deprecated synonym for macro_use
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:458:17
[01:14:10]    |
[01:14:10] LL |     mod inner { #![macro_escape] }
[01:14:10]    |
[01:14:10]    |
[01:14:10]    = help: consider an outer attribute, #[macro_use] mod ...
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:179:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[macro_use] fn f() { }
[01:14:10]    |
[01:14:10] note: lint level defined here
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:33:9
[01:14:10]    |
---
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:185:5
[01:14:10]    |
[01:14:10] LL |     #[macro_use] type T = S;
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:188:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[macro_use] impl S { }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:195:17
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     mod inner { #![macro_export] }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:198:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[macro_export] fn f() { }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:201:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[macro_export] struct S;
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:204:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[macro_export] type T = S;
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:207:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[macro_export] impl S { }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:192:1
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL | #[macro_export]
[01:14:10]    | ^^^^^^^^^^^^^^^
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:214:17
[01:14:10]    |
[01:14:10] LL |     mod inner { #![plugin_registrar] }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:219:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[plugin_registrar] struct S;
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:222:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[plugin_registrar] type T = S;
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:225:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[plugin_registrar] impl S { }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:211:1
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL | #[plugin_registrar]
[01:14:10]    | ^^^^^^^^^^^^^^^^^^^
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:232:17
[01:14:10]    |
[01:14:10] LL |     mod inner { #![main] }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:237:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[main] struct S;
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:240:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[main] type T = S;
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:243:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[main] impl S { }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:229:1
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL | #[main]
[01:14:10]    | ^^^^^^^
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:250:17
[01:14:10]    |
[01:14:10] LL |     mod inner { #![start] }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:255:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[start] struct S;
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:258:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[start] type T = S;
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:261:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[start] impl S { }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:247:1
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL | #[start]
[01:14:10]    | ^^^^^^^^
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:314:5
[01:14:10]    |
[01:14:10] LL |     #[path = "3800"] fn f() { }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:317:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[path = "3800"]  struct S;
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:320:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[path = "3800"] type T = S;
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:323:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[path = "3800"] impl S { }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:330:17
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     mod inner { #![automatically_derived] }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:333:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[automatically_derived] fn f() { }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:336:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[automatically_derived] struct S;
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:339:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[automatically_derived] type T = S;
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:342:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[automatically_derived] impl S { }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:327:1
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL | #[automatically_derived]
[01:14:10]    | ^^^^^^^^^^^^^^^^^^^^^^^^
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:362:17
[01:14:10]    |
[01:14:10] LL |     mod inner { #![no_link] }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:365:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[no_link] fn f() { }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:368:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[no_link] struct S;
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:371:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[no_link]type T = S;
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:374:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[no_link] impl S { }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:359:1
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL | #[no_link]
[01:14:10]    | ^^^^^^^^^^
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:381:17
[01:14:10]    |
[01:14:10] LL |     mod inner { #![should_panic] }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:384:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[should_panic] fn f() { }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:387:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[should_panic] struct S;
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:390:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[should_panic] type T = S;
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:393:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[should_panic] impl S { }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:378:1
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL | #[should_panic]
[01:14:10]    | ^^^^^^^^^^^^^^^
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:400:17
[01:14:10]    |
[01:14:10] LL |     mod inner { #![ignore] }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:403:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[ignore] fn f() { }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:406:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[ignore] struct S;
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:409:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[ignore] type T = S;
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:412:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[ignore] impl S { }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:397:1
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL | #[ignore]
[01:14:10]    | ^^^^^^^^^
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:419:17
[01:14:10]    |
[01:14:10] LL |     mod inner { #![no_implicit_prelude] }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:422:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[no_implicit_prelude] fn f() { }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:425:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[no_implicit_prelude] struct S;
[01:14:10]    |     ^^^^^^^^^^^^^^^^^^^^^^
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:428:5
[01:14:10]    |
[01:14:10] LL |     #[no_implicit_prelude] type T = S;
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:431:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[no_implicit_prelude] impl S { }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:416:1
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL | #[no_implicit_prelude]
[01:14:10]    | ^^^^^^^^^^^^^^^^^^^^^^
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:438:17
[01:14:10]    |
[01:14:10] LL |     mod inner { #![reexport_test_harness_main="2900"] }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:441:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[reexport_test_harness_main = "2900"] fn f() { }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:444:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[reexport_test_harness_main = "2900"] struct S;
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:447:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[reexport_test_harness_main = "2900"] type T = S;
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:450:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[reexport_test_harness_main = "2900"] impl S { }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:435:1
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL | #[reexport_test_harness_main = "2900"]
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:461:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[macro_escape] fn f() { }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:464:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[macro_escape] struct S;
[01:14:10]    |     ^^^^^^^^^^^^^^^
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:467:5
[01:14:10]    |
[01:14:10] LL |     #[macro_escape] type T = S;
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:470:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[macro_escape] impl S { }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:478:17
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     mod inner { #![no_std] }
[01:14:10] 
[01:14:10] warning: crate-level attribute should be in the root module
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:478:17
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     mod inner { #![no_std] }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:482:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[no_std] fn f() { }
[01:14:10] 
[01:14:10] 
[01:14:10] warning: crate-level attribute should be an inner attribute: add an exclamation mark: #![foo]
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[no_std] fn f() { }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:486:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[no_std] struct S;
[01:14:10] 
[01:14:10] 
[01:14:10] warning: crate-level attribute should be an inner attribute: add an exclamation mark: #![foo]
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[no_std] struct S;
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:490:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[no_std] type T = S;
[01:14:10] 
[01:14:10] 
[01:14:10] warning: crate-level attribute should be an inner attribute: add an exclamation mark: #![foo]
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[no_std] type T = S;
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:494:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[no_std] impl S { }
[01:14:10] 
[01:14:10] 
[01:14:10] warning: crate-level attribute should be an inner attribute: add an exclamation mark: #![foo]
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[no_std] impl S { }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:474:1
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL | #[no_std]
[01:14:10]    | ^^^^^^^^^
[01:14:10] 
[01:14:10] warning: crate-level attribute should be an inner attribute: add an exclamation mark: #![foo]
[01:14:10]    |
[01:14:10] LL | #[no_std]
[01:14:10]    | ^^^^^^^^^
[01:14:10] 
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:633:17
[01:14:10]    |
[01:14:10] LL |     mod inner { #![crate_name="0900"] }
[01:14:10] 
[01:14:10] warning: crate-level attribute should be in the root module
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:633:17
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     mod inner { #![crate_name="0900"] }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:637:5
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[crate_name = "0900"] fn f() { }
[01:14:10] 
[01:14:10] 
[01:14:10] warning: crate-level attribute should be an inner attribute: add an exclamation mark: #![foo]
[01:14:10]    |
[01:14:10]    |
[01:14:10] LL |     #[crate_name = "0900"] fn f() { }
[01:14:10] 
[01:14:10] warning: unused attribute
[01:14:10]   --> /checkout/src/test/ui/feature-gate/issue-43106-gating-of-builtin-attrs.rs:641:5
---
[01:14:10] test result: FAILED. 5508 passed; 1 failed; 21 ignored; 0 measured; 0 filtered out
[01:14:10] 
[01:14:10] 
[01:14:10] 
[01:14:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:14:10] 
[01:14:10] 
[01:14:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:10] Build completed unsuccessfully in 0:04:49
[01:14:10] Build completed unsuccessfully in 0:04:49
[01:14:10] make: *** [check] Error 1
[01:14:10] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f169a27
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue May 14 13:36:05 UTC 2019
