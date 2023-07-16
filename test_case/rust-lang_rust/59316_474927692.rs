plain
travis_time:end:11b1b5dc:start=1553095864840844072,finish=1553095868258495909,duration=3417651837
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:31] 
[01:20:31] running 120 tests
[01:20:57] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:21:02] .i......iii.i.....ii
[01:21:02] 
[01:21:02]  finished in 30.218
[01:21:02] travis_fold:end:test_debuginfo

---
travis_time:start:test_ui-fulldeps
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:02] 
[01:21:02] running 22 tests
[01:21:09] ...FF.................
[01:21:09] 
[01:21:09] ---- [ui] ui-fulldeps/internal-lints/default_hash_types.rs stdout ----
[01:21:09] diff of stderr:
[01:21:09] 
[01:21:09] 
[01:21:09] - warning: Prefer FxHashMap over HashMap, it has better performance
[01:21:09] -    |
[01:21:09] -    |
[01:21:09] - LL | use std::collections::{HashMap, HashSet};
[01:21:09] -    |                        ^^^^^^^ help: use: `FxHashMap`
[01:21:09] -    = note: #[warn(default_hash_types)] on by default
[01:21:09] -    = note: #[warn(default_hash_types)] on by default
[01:21:09] -    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary
[01:21:09] - 
[01:21:09] - warning: Prefer FxHashSet over HashSet, it has better performance
[01:21:09] -    |
[01:21:09] -    |
[01:21:09] - LL | use std::collections::{HashMap, HashSet};
[01:21:09] -    |                                 ^^^^^^^ help: use: `FxHashSet`
[01:21:09] -    |
[01:21:09] -    = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary
[01:21:09] - 
[01:21:09] - error: Prefer FxHashMap over HashMap, it has better performance
[01:21:09] -    |
[01:21:09] -    |
[01:21:09] - LL |     let _map: HashMap<String, String> = HashMap::default();
[01:21:09] -    |               ^^^^^^^ help: use: `FxHashMap`
[01:21:09] - note: lint level defined here
[01:21:09] -   --> $DIR/default_hash_types.rs:12:8
[01:21:09] -    |
[01:21:09] -    |
[01:21:09] - LL | #[deny(default_hash_types)]
[01:21:09] -    |        ^^^^^^^^^^^^^^^^^^
[01:21:09] -    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary
[01:21:09] - 
[01:21:09] - error: Prefer FxHashMap over HashMap, it has better performance
[01:21:09] -    |
[01:21:09] -    |
[01:21:09] - LL |     let _map: HashMap<String, String> = HashMap::default();
[01:21:09] -    |                                         ^^^^^^^ help: use: `FxHashMap`
[01:21:09] -    |
[01:21:09] -    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary
[01:21:09] - 
[01:21:09] - error: Prefer FxHashSet over HashSet, it has better performance
[01:21:09] -    |
[01:21:09] -    |
[01:21:09] - LL |     let _set: HashSet<String> = HashSet::default();
[01:21:09] -    |               ^^^^^^^ help: use: `FxHashSet`
[01:21:09] -    |
[01:21:09] -    = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary
[01:21:09] - 
[01:21:09] - error: Prefer FxHashSet over HashSet, it has better performance
[01:21:09] -    |
[01:21:09] -    |
[01:21:09] - LL |     let _set: HashSet<String> = HashSet::default();
[01:21:09] -    |                                 ^^^^^^^ help: use: `FxHashSet`
[01:21:09] -    |
[01:21:09] -    = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary
[01:21:09] - error: aborting due to 4 previous errors
[01:21:09] + error: unknown debugging option: `internal-lints`
[01:21:09] 56 
[01:21:09] 57 
[01:21:09] 57 
[01:21:09] 
[01:21:09] 
[01:21:09] The actual stderr differed from the expected stderr.
[01:21:09] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/default_hash_types/default_hash_types.stderr
[01:21:09] To update references, rerun the tests and pass the `--bless` flag
[01:21:09] To only update this specific test, also pass `--test-args internal-lints/default_hash_types.rs`
[01:21:09] error: 1 errors occurred comparing output.
[01:21:09] status: exit code: 1
[01:21:09] status: exit code: 1
[01:21:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/default_hash_types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/default_hash_types/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "internal-lints" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/default_hash_types/auxiliary" "-A" "unused"
[01:21:09] ------------------------------------------
[01:21:09] 
[01:21:09] ------------------------------------------
[01:21:09] stderr:
[01:21:09] stderr:
[01:21:09] ------------------------------------------
[01:21:09] {"message":"unknown debugging option: `internal-lints`","code":null,"level":"error","spans":[],"children":[],"rendered":"error: unknown debugging option: `internal-lints`\n\n"}
[01:21:09] ------------------------------------------
[01:21:09] 
[01:21:09] thread '[ui] ui-fulldeps/internal-lints/default_hash_types.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:21:09] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:21:09] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:21:09] 
[01:21:09] ---- [ui] ui-fulldeps/internal-lints/ty_tykind_usage.rs stdout ----
[01:21:09] 
[01:21:09] 
[01:21:09] - error: usage of `ty::TyKind::<kind>`
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:11:15
[01:21:09] -    |
[01:21:09] - LL |     let sty = TyKind::Bool; //~ ERROR usage of `ty::TyKind::<kind>`
[01:21:09] -    |               ^^^^^^ help: try using ty::<kind> directly: `ty`
[01:21:09] - note: lint level defined here
[01:21:09] - note: lint level defined here
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:9:8
[01:21:09] -    |
[01:21:09] - LL | #[deny(usage_of_ty_tykind)]
[01:21:09] - 
[01:21:09] - 
[01:21:09] - error: usage of `ty::TyKind::<kind>`
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:14:9
[01:21:09] -    |
[01:21:09] - LL |         TyKind::Bool => (), //~ ERROR usage of `ty::TyKind::<kind>`
[01:21:09] -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
[01:21:09] - 
[01:21:09] - error: usage of `ty::TyKind::<kind>`
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:15:9
[01:21:09] -    |
[01:21:09] - LL |         TyKind::Char => (), //~ ERROR usage of `ty::TyKind::<kind>`
[01:21:09] -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
[01:21:09] - 
[01:21:09] - error: usage of `ty::TyKind::<kind>`
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:16:9
[01:21:09] -    |
[01:21:09] - LL |         TyKind::Int(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
[01:21:09] -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
[01:21:09] - 
[01:21:09] - error: usage of `ty::TyKind::<kind>`
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:17:9
[01:21:09] -    |
[01:21:09] - LL |         TyKind::Uint(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
[01:21:09] -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
[01:21:09] - 
[01:21:09] - error: usage of `ty::TyKind::<kind>`
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:18:9
[01:21:09] -    |
[01:21:09] - LL |         TyKind::Float(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
[01:21:09] -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
[01:21:09] - 
[01:21:09] - error: usage of `ty::TyKind::<kind>`
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:19:9
[01:21:09] -    |
[01:21:09] - LL |         TyKind::Adt(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
[01:21:09] -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
[01:21:09] - 
[01:21:09] - error: usage of `ty::TyKind::<kind>`
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:20:9
[01:21:09] -    |
[01:21:09] - LL |         TyKind::Foreign(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
[01:21:09] -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
[01:21:09] - 
[01:21:09] - error: usage of `ty::TyKind::<kind>`
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:21:9
[01:21:09] -    |
[01:21:09] - LL |         TyKind::Str => (), //~ ERROR usage of `ty::TyKind::<kind>`
[01:21:09] -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
[01:21:09] - 
[01:21:09] - error: usage of `ty::TyKind::<kind>`
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:22:9
[01:21:09] -    |
[01:21:09] - LL |         TyKind::Array(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
[01:21:09] -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
[01:21:09] - 
[01:21:09] - error: usage of `ty::TyKind::<kind>`
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:23:9
[01:21:09] -    |
[01:21:09] - LL |         TyKind::Slice(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
[01:21:09] -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
[01:21:09] - 
[01:21:09] - error: usage of `ty::TyKind::<kind>`
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:24:9
[01:21:09] -    |
[01:21:09] - LL |         TyKind::RawPtr(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
[01:21:09] -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
[01:21:09] - 
[01:21:09] - error: usage of `ty::TyKind::<kind>`
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:25:9
[01:21:09] -    |
[01:21:09] - LL |         TyKind::Ref(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
[01:21:09] -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
[01:21:09] - 
[01:21:09] - error: usage of `ty::TyKind::<kind>`
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:26:9
[01:21:09] -    |
[01:21:09] - LL |         TyKind::FnDef(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
[01:21:09] -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
[01:21:09] - 
[01:21:09] - error: usage of `ty::TyKind::<kind>`
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:27:9
[01:21:09] -    |
[01:21:09] - LL |         TyKind::FnPtr(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
[01:21:09] -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
[01:21:09] - 
[01:21:09] - error: usage of `ty::TyKind::<kind>`
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:28:9
[01:21:09] -    |
[01:21:09] - LL |         TyKind::Dynamic(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
[01:21:09] -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
[01:21:09] - 
[01:21:09] - error: usage of `ty::TyKind::<kind>`
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:29:9
[01:21:09] -    |
[01:21:09] - LL |         TyKind::Closure(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
[01:21:09] -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
[01:21:09] - 
[01:21:09] - error: usage of `ty::TyKind::<kind>`
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:30:9
[01:21:09] -    |
[01:21:09] - LL |         TyKind::Generator(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
[01:21:09] -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
[01:21:09] - 
[01:21:09] - error: usage of `ty::TyKind::<kind>`
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:31:9
[01:21:09] -    |
[01:21:09] - LL |         TyKind::GeneratorWitness(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
[01:21:09] -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
[01:21:09] - 
[01:21:09] - error: usage of `ty::TyKind::<kind>`
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:32:9
[01:21:09] -    |
[01:21:09] - LL |         TyKind::Never => (), //~ ERROR usage of `ty::TyKind::<kind>`
[01:21:09] -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
[01:21:09] - 
[01:21:09] - error: usage of `ty::TyKind::<kind>`
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:33:9
[01:21:09] -    |
[01:21:09] - LL |         TyKind::Tuple(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
[01:21:09] -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
[01:21:09] - 
[01:21:09] - error: usage of `ty::TyKind::<kind>`
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:34:9
[01:21:09] -    |
[01:21:09] - LL |         TyKind::Projection(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
[01:21:09] -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
[01:21:09] - 
[01:21:09] - error: usage of `ty::TyKind::<kind>`
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:35:9
[01:21:09] -    |
[01:21:09] - LL |         TyKind::UnnormalizedProjection(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
[01:21:09] -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
[01:21:09] - 
[01:21:09] - error: usage of `ty::TyKind::<kind>`
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:36:9
[01:21:09] -    |
[01:21:09] - LL |         TyKind::Opaque(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
[01:21:09] -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
[01:21:09] - 
[01:21:09] - error: usage of `ty::TyKind::<kind>`
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:37:9
[01:21:09] -    |
[01:21:09] - LL |         TyKind::Param(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
[01:21:09] -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
[01:21:09] - 
[01:21:09] - error: usage of `ty::TyKind::<kind>`
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:38:9
[01:21:09] -    |
[01:21:09] - LL |         TyKind::Bound(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
[01:21:09] -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
[01:21:09] - 
[01:21:09] - error: usage of `ty::TyKind::<kind>`
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:39:9
[01:21:09] -    |
[01:21:09] - LL |         TyKind::Placeholder(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
[01:21:09] -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
[01:21:09] - 
[01:21:09] - error: usage of `ty::TyKind::<kind>`
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:40:9
[01:21:09] -    |
[01:21:09] - LL |         TyKind::Infer(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
[01:21:09] -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
[01:21:09] - 
[01:21:09] - error: usage of `ty::TyKind::<kind>`
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:41:9
[01:21:09] -    |
[01:21:09] - LL |         TyKind::Error => (), //~ ERROR usage of `ty::TyKind::<kind>`
[01:21:09] -    |         ^^^^^^ help: try using ty::<kind> directly: `ty`
[01:21:09] - 
[01:21:09] - error: usage of `ty::TyKind::<kind>`
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:46:12
[01:21:09] -    |
[01:21:09] - LL |     if let TyKind::Int(int_ty) = sty {} //~ ERROR usage of `ty::TyKind::<kind>`
[01:21:09] -    |            ^^^^^^ help: try using ty::<kind> directly: `ty`
[01:21:09] - error: usage of `ty::TyKind`
[01:21:09] - error: usage of `ty::TyKind`
[01:21:09] -   --> $DIR/ty_tykind_usage.rs:48:24
[01:21:09] -    |
[01:21:09] - LL |     fn ty_kind(ty_bad: TyKind<'_>, ty_good: Ty<'_>) {} //~ ERROR usage of `ty::TyKind`
[01:21:09] -    |
[01:21:09] -    = help: try using `ty::Ty` instead
[01:21:09] - 
[01:21:09] - error: aborting due to 31 previous errors
[01:21:09] - error: aborting due to 31 previous errors
[01:21:09] + error: unknown debugging option: `internal-lints`
[01:21:09] 196 
[01:21:09] 197 
[01:21:09] 
[01:21:09] 
[01:21:09] The actual stderr differed from the expected stderr.
[01:21:09] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/ty_tykind_usage/ty_tykind_usage.stderr
[01:21:09] To update references, rerun the tests and pass the `--bless` flag
[01:21:09] To only update this specific test, also pass `--test-args internal-lints/ty_tykind_usage.rs`
[01:21:09] error: 1 errors occurred comparing output.
[01:21:09] status: exit code: 1
[01:21:09] status: exit code: 1
[01:21:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/ty_tykind_usage/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "internal-lints" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/ty_tykind_usage/auxiliary" "-A" "unused"
[01:21:09] ------------------------------------------
[01:21:09] 
[01:21:09] ------------------------------------------
[01:21:09] stderr:
[01:21:09] stderr:
[01:21:09] ------------------------------------------
[01:21:09] {"message":"unknown debugging option: `internal-lints`","code":null,"level":"error","spans":[],"children":[],"rendered":"error: unknown debugging option: `internal-lints`\n\n"}
[01:21:09] ------------------------------------------
[01:21:09] 
[01:21:09] 
[01:21:09] thread '[ui] ui-fulldeps/internal-lints/ty_tykind_usage.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:21:09] 
[01:21:09] failures:
[01:21:09]     [ui] ui-fulldeps/internal-lints/default_hash_types.rs
[01:21:09]     [ui] ui-fulldeps/internal-lints/default_hash_types.rs
[01:21:09]     [ui] ui-fulldeps/internal-lints/ty_tykind_usage.rs
[01:21:09] 
[01:21:09] test result: FAILED. 20 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:21:09] 
[01:21:09] 
[01:21:09] 
[01:21:09] 
[01:21:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:21:09] 
[01:21:09] 
[01:21:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:21:09] Build completed unsuccessfully in 0:12:28
[01:21:09] Build completed unsuccessfully in 0:12:28
[01:21:09] make: *** [check] Error 1
[01:21:09] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0898e024
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Mar 20 16:52:29 UTC 2019
---
travis_time:end:0900769b:start=1553100751096494654,finish=1553100751101272437,duration=4777783
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:014d1d9a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:10f16584
travis_time:start:10f16584
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02cdc0b4
$ dmesg | grep -i kill
