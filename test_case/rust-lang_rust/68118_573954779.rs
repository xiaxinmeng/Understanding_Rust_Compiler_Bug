plain
2020-01-14T01:25:04.3342710Z 
2020-01-14T01:25:04.3343346Z 46 LL |         #[rustc_def_path]
2020-01-14T01:25:04.3343833Z 47    |         ^^^^^^^^^^^^^^^^^
2020-01-14T01:25:04.3344080Z 48 
2020-01-14T01:25:04.3344869Z - error: symbol-name(_ZN209_$LT$$u5b$$RF$dyn$u20$impl1..Foo$u2b$Assoc$u20$$u3d$$u20$extern$u20$$u22$C$u22$$u20$fn$LP$$RF$u8$C$$u20$...$RP$$u2b$impl1..AutoTrait$u3b$$u20$3$u5d$$u20$as$u20$impl1..main..$u7b$$u7b$closure$u7d$$u7d$..Bar$GT$6method17h62e540f14f879d56E)
2020-01-14T01:25:04.3345810Z + error: symbol-name(_ZN209_$LT$$u5b$$RF$dyn$u20$impl1..Foo$u2b$Assoc$u20$$u3d$$u20$extern$u20$$u22$C$u22$$u20$fn$LP$$RF$u8$C$$u20$...$RP$$u2b$impl1..AutoTrait$u3b$$u20$3$u5d$$u20$as$u20$impl1..main..$u7b$$u7b$closure$u7d$$u7d$..Bar$GT$6method17hdb62078998ce7ea8E)
2020-01-14T01:25:04.3347236Z 51    |
2020-01-14T01:25:04.3347431Z 52 LL |             #[rustc_symbol_name]
2020-01-14T01:25:04.3347501Z 
2020-01-14T01:25:04.3347727Z 53    |             ^^^^^^^^^^^^^^^^^^^^
2020-01-14T01:25:04.3347727Z 53    |             ^^^^^^^^^^^^^^^^^^^^
2020-01-14T01:25:04.3347785Z 54 
2020-01-14T01:25:04.3349014Z - error: demangling(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; 3] as impl1::main::{{closure}}::Bar>::method::h62e540f14f879d56)
2020-01-14T01:25:04.3349149Z + error: demangling(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; 3] as impl1::main::{{closure}}::Bar>::method::hdb62078998ce7ea8)
2020-01-14T01:25:04.3350201Z 57    |
2020-01-14T01:25:04.3350253Z 58 LL |             #[rustc_symbol_name]
2020-01-14T01:25:04.3350292Z 
2020-01-14T01:25:04.3350341Z 
2020-01-14T01:25:04.3350341Z 
2020-01-14T01:25:04.3350398Z The actual stderr differed from the expected stderr.
2020-01-14T01:25:04.3351095Z Actual stderr saved to /checkout/obj/build/i686-unknown-linux-gnu/test/ui/symbol-names/impl1.legacy/impl1.legacy.stderr
2020-01-14T01:25:04.3351355Z To update references, rerun the tests and pass the `--bless` flag
2020-01-14T01:25:04.3351794Z To only update this specific test, also pass `--test-args symbol-names/impl1.rs`
2020-01-14T01:25:04.3351847Z 
2020-01-14T01:25:04.3351936Z error in revision `legacy`: 1 errors occurred comparing output.
2020-01-14T01:25:04.3352004Z status: exit code: 1
2020-01-14T01:25:04.3352873Z command: "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/impl1.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--cfg" "legacy" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui/symbol-names/impl1.legacy" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=legacy" "-L" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui/symbol-names/impl1.legacy/auxiliary" "-A" "unused"
2020-01-14T01:25:04.3353539Z ------------------------------------------
2020-01-14T01:25:04.3353581Z 
2020-01-14T01:25:04.3353775Z ------------------------------------------
2020-01-14T01:25:04.3353855Z stderr:
2020-01-14T01:25:04.3353855Z stderr:
2020-01-14T01:25:04.3354197Z ------------------------------------------
2020-01-14T01:25:04.3354503Z error: symbol-name(_ZN5impl13foo3Foo3bar17h92cf46db76791039E)
2020-01-14T01:25:04.3354959Z    |
2020-01-14T01:25:04.3355027Z LL |         #[rustc_symbol_name]
2020-01-14T01:25:04.3355082Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-01-14T01:25:04.3355137Z 
2020-01-14T01:25:04.3355137Z 
2020-01-14T01:25:04.3355192Z error: demangling(impl1::foo::Foo::bar::h92cf46db76791039)
2020-01-14T01:25:04.3355653Z    |
2020-01-14T01:25:04.3355720Z LL |         #[rustc_symbol_name]
2020-01-14T01:25:04.3355858Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-01-14T01:25:04.3355912Z 
2020-01-14T01:25:04.3355912Z 
2020-01-14T01:25:04.3356124Z error: demangling-alt(impl1::foo::Foo::bar)
2020-01-14T01:25:04.3356398Z    |
2020-01-14T01:25:04.3356473Z LL |         #[rustc_symbol_name]
2020-01-14T01:25:04.3356526Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-01-14T01:25:04.3356578Z 
2020-01-14T01:25:04.3356578Z 
2020-01-14T01:25:04.3356747Z error: def-path(foo::Foo::bar)
2020-01-14T01:25:04.3357017Z    |
2020-01-14T01:25:04.3357083Z LL |         #[rustc_def_path]
2020-01-14T01:25:04.3357135Z    |         ^^^^^^^^^^^^^^^^^
2020-01-14T01:25:04.3357168Z 
2020-01-14T01:25:04.3357168Z 
2020-01-14T01:25:04.3357599Z error: symbol-name(_ZN5impl13bar33_$LT$impl$u20$impl1..foo..Foo$GT$3baz17h90c4a800b1aa0df0E)
2020-01-14T01:25:04.3357901Z    |
2020-01-14T01:25:04.3357976Z LL |         #[rustc_symbol_name]
2020-01-14T01:25:04.3358032Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-01-14T01:25:04.3358087Z 
2020-01-14T01:25:04.3358087Z 
2020-01-14T01:25:04.3358146Z error: demangling(impl1::bar::<impl impl1::foo::Foo>::baz::h90c4a800b1aa0df0)
2020-01-14T01:25:04.3358448Z    |
2020-01-14T01:25:04.3358516Z LL |         #[rustc_symbol_name]
2020-01-14T01:25:04.3358571Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-01-14T01:25:04.3358623Z 
2020-01-14T01:25:04.3358623Z 
2020-01-14T01:25:04.3359006Z error: demangling-alt(impl1::bar::<impl impl1::foo::Foo>::baz)
2020-01-14T01:25:04.3359299Z    |
2020-01-14T01:25:04.3359366Z LL |         #[rustc_symbol_name]
2020-01-14T01:25:04.3359423Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-01-14T01:25:04.3359476Z 
2020-01-14T01:25:04.3359476Z 
2020-01-14T01:25:04.3359664Z error: def-path(bar::<impl foo::Foo>::baz)
2020-01-14T01:25:04.3359960Z    |
2020-01-14T01:25:04.3360030Z LL |         #[rustc_def_path]
2020-01-14T01:25:04.3360085Z    |         ^^^^^^^^^^^^^^^^^
2020-01-14T01:25:04.3360138Z 
2020-01-14T01:25:04.3360138Z 
2020-01-14T01:25:04.3360640Z error: symbol-name(_ZN209_$LT$$u5b$$RF$dyn$u20$impl1..Foo$u2b$Assoc$u20$$u3d$$u20$extern$u20$$u22$C$u22$$u20$fn$LP$$RF$u8$C$$u20$...$RP$$u2b$impl1..AutoTrait$u3b$$u20$3$u5d$$u20$as$u20$impl1..main..$u7b$$u7b$closure$u7d$$u7d$..Bar$GT$6method17hdb62078998ce7ea8E)
2020-01-14T01:25:04.3361175Z    |
2020-01-14T01:25:04.3361224Z LL |             #[rustc_symbol_name]
2020-01-14T01:25:04.3361298Z    |             ^^^^^^^^^^^^^^^^^^^^
2020-01-14T01:25:04.3361335Z 
2020-01-14T01:25:04.3361335Z 
2020-01-14T01:25:04.3361432Z error: demangling(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; 3] as impl1::main::{{closure}}::Bar>::method::hdb62078998ce7ea8)
2020-01-14T01:25:04.3361753Z    |
2020-01-14T01:25:04.3361802Z LL |             #[rustc_symbol_name]
2020-01-14T01:25:04.3361874Z    |             ^^^^^^^^^^^^^^^^^^^^
2020-01-14T01:25:04.3361909Z 
2020-01-14T01:25:04.3361909Z 
2020-01-14T01:25:04.3362449Z error: demangling-alt(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; 3] as impl1::main::{{closure}}::Bar>::method)
2020-01-14T01:25:04.3362817Z    |
2020-01-14T01:25:04.3362867Z LL |             #[rustc_symbol_name]
2020-01-14T01:25:04.3362942Z    |             ^^^^^^^^^^^^^^^^^^^^
2020-01-14T01:25:04.3362980Z 
2020-01-14T01:25:04.3362980Z 
2020-01-14T01:25:04.3363273Z error: def-path(<[&dyn Foo<Assoc = for<'r> extern "C" fn(&'r u8, ...)> + AutoTrait; 3] as main::{{closure}}#1::Bar>::method)
2020-01-14T01:25:04.3363593Z    |
2020-01-14T01:25:04.3363644Z LL |             #[rustc_def_path]
2020-01-14T01:25:04.3363810Z    |             ^^^^^^^^^^^^^^^^^
2020-01-14T01:25:04.3363847Z 
---
2020-01-14T01:25:04.3365205Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:387:22
2020-01-14T01:25:04.3365309Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-14T01:25:04.3411288Z 
2020-01-14T01:25:04.3413064Z 
2020-01-14T01:25:04.3415497Z command did not execute successfully: "/checkout/obj/build/i686-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/i686-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i686-unknown-linux-gnu" "--mode" "ui" "--target" "i686-unknown-linux-gnu" "--host" "i686-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/i686-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-14T01:25:04.3416033Z 
2020-01-14T01:25:04.3416083Z 
2020-01-14T01:25:04.3416138Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-14T01:25:04.3416222Z Build completed unsuccessfully in 1:14:36
2020-01-14T01:25:04.3416222Z Build completed unsuccessfully in 1:14:36
2020-01-14T01:25:04.3433248Z == clock drift check ==
2020-01-14T01:25:04.3449707Z   local time: Tue Jan 14 01:25:04 UTC 2020
2020-01-14T01:25:04.4163775Z   network time: Tue, 14 Jan 2020 01:25:04 GMT
2020-01-14T01:25:04.4170386Z == end clock drift check ==
2020-01-14T01:25:04.8911600Z 
2020-01-14T01:25:04.9009382Z ##[error]Bash exited with code '1'.
2020-01-14T01:25:04.9053142Z ##[section]Starting: Checkout
2020-01-14T01:25:04.9056294Z ==============================================================================
2020-01-14T01:25:04.9056414Z Task         : Get sources
2020-01-14T01:25:04.9056503Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
