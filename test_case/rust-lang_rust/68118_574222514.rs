plain
2020-01-14T15:09:12.7171589Z 
2020-01-14T15:09:12.7171668Z 46 LL |         #[rustc_def_path]
2020-01-14T15:09:12.7171739Z 47    |         ^^^^^^^^^^^^^^^^^
2020-01-14T15:09:12.7171820Z 48 
2020-01-14T15:09:12.7172573Z - error: symbol-name(_ZN209_$LT$$u5b$$RF$dyn$u20$impl1..Foo$u2b$Assoc$u20$$u3d$$u20$extern$u20$$u22$C$u22$$u20$fn$LP$$RF$u8$C$$u20$...$RP$$u2b$impl1..AutoTrait$u3b$$u20$3$u5d$$u20$as$u20$impl1..main..$u7b$$u7b$closure$u7d$$u7d$..Bar$GT$6method17SYMBOL_HASHE)
2020-01-14T15:09:12.7173321Z + error: symbol-name(_ZN209_$LT$$u5b$$RF$dyn$u20$impl1..Foo$u2b$Assoc$u20$$u3d$$u20$extern$u20$$u22$C$u22$$u20$fn$LP$$RF$u8$C$$u20$...$RP$$u2b$impl1..AutoTrait$u3b$$u20$3$u5d$$u20$as$u20$impl1..main..$u7b$$u7b$closure$u7d$$u7d$..Bar$GT$6method17hdb62078998ce7ea8E)
2020-01-14T15:09:12.7173997Z 51    |
2020-01-14T15:09:12.7174059Z 52 LL |             #[rustc_symbol_name]
2020-01-14T15:09:12.7174115Z 
2020-01-14T15:09:12.7174173Z 53    |             ^^^^^^^^^^^^^^^^^^^^
2020-01-14T15:09:12.7174173Z 53    |             ^^^^^^^^^^^^^^^^^^^^
2020-01-14T15:09:12.7174251Z 54 
2020-01-14T15:09:12.7174599Z - error: demangling(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; 3] as impl1::main::{{closure}}::Bar>::method::SYMBOL_HASH)
2020-01-14T15:09:12.7174748Z + error: demangling(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; 3] as impl1::main::{{closure}}::Bar>::method::hdb62078998ce7ea8)
2020-01-14T15:09:12.7175106Z 57    |
2020-01-14T15:09:12.7175178Z 58 LL |             #[rustc_symbol_name]
2020-01-14T15:09:12.7175220Z 
2020-01-14T15:09:12.7175252Z 
2020-01-14T15:09:12.7175252Z 
2020-01-14T15:09:12.7175331Z The actual stderr differed from the expected stderr.
2020-01-14T15:09:12.7175658Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.legacy/impl1.legacy.stderr
2020-01-14T15:09:12.7175954Z To update references, rerun the tests and pass the `--bless` flag
2020-01-14T15:09:12.7176235Z To only update this specific test, also pass `--test-args symbol-names/impl1.rs`
2020-01-14T15:09:12.7176307Z 
2020-01-14T15:09:12.7176477Z error in revision `legacy`: 1 errors occurred comparing output.
2020-01-14T15:09:12.7176560Z status: exit code: 1
2020-01-14T15:09:12.7177479Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/impl1.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--cfg" "legacy" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.legacy" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-Z" "symbol-mangling-version=legacy" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.legacy/auxiliary" "-A" "unused"
2020-01-14T15:09:12.7177995Z ------------------------------------------
2020-01-14T15:09:12.7178041Z 
2020-01-14T15:09:12.7178288Z ------------------------------------------
2020-01-14T15:09:12.7178354Z stderr:
2020-01-14T15:09:12.7178354Z stderr:
2020-01-14T15:09:12.7178592Z ------------------------------------------
2020-01-14T15:09:12.7178869Z error: symbol-name(_ZN5impl13foo3Foo3bar17h92cf46db76791039E)
2020-01-14T15:09:12.7179225Z    |
2020-01-14T15:09:12.7179287Z LL |         #[rustc_symbol_name]
2020-01-14T15:09:12.7179480Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-01-14T15:09:12.7179522Z 
2020-01-14T15:09:12.7179522Z 
2020-01-14T15:09:12.7179599Z error: demangling(impl1::foo::Foo::bar::h92cf46db76791039)
2020-01-14T15:09:12.7180505Z    |
2020-01-14T15:09:12.7180576Z LL |         #[rustc_symbol_name]
2020-01-14T15:09:12.7180667Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-01-14T15:09:12.7180711Z 
2020-01-14T15:09:12.7180711Z 
2020-01-14T15:09:12.7181012Z error: demangling-alt(impl1::foo::Foo::bar)
2020-01-14T15:09:12.7181385Z    |
2020-01-14T15:09:12.7181445Z LL |         #[rustc_symbol_name]
2020-01-14T15:09:12.7181527Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-01-14T15:09:12.7181571Z 
2020-01-14T15:09:12.7181571Z 
2020-01-14T15:09:12.7181793Z error: def-path(foo::Foo::bar)
2020-01-14T15:09:12.7182320Z    |
2020-01-14T15:09:12.7182452Z LL |         #[rustc_def_path]
2020-01-14T15:09:12.7182540Z    |         ^^^^^^^^^^^^^^^^^
2020-01-14T15:09:12.7182583Z 
2020-01-14T15:09:12.7182583Z 
2020-01-14T15:09:12.7182920Z error: symbol-name(_ZN5impl13bar33_$LT$impl$u20$impl1..foo..Foo$GT$3baz17h90c4a800b1aa0df0E)
2020-01-14T15:09:12.7183318Z    |
2020-01-14T15:09:12.7183378Z LL |         #[rustc_symbol_name]
2020-01-14T15:09:12.7183458Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-01-14T15:09:12.7183502Z 
2020-01-14T15:09:12.7183502Z 
2020-01-14T15:09:12.7183587Z error: demangling(impl1::bar::<impl impl1::foo::Foo>::baz::h90c4a800b1aa0df0)
2020-01-14T15:09:12.7184181Z    |
2020-01-14T15:09:12.7184237Z LL |         #[rustc_symbol_name]
2020-01-14T15:09:12.7184312Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-01-14T15:09:12.7184362Z 
2020-01-14T15:09:12.7184362Z 
2020-01-14T15:09:12.7184628Z error: demangling-alt(impl1::bar::<impl impl1::foo::Foo>::baz)
2020-01-14T15:09:12.7185190Z    |
2020-01-14T15:09:12.7185245Z LL |         #[rustc_symbol_name]
2020-01-14T15:09:12.7185321Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-01-14T15:09:12.7185362Z 
2020-01-14T15:09:12.7185362Z 
2020-01-14T15:09:12.7185598Z error: def-path(bar::<impl foo::Foo>::baz)
2020-01-14T15:09:12.7185929Z    |
2020-01-14T15:09:12.7185985Z LL |         #[rustc_def_path]
2020-01-14T15:09:12.7186060Z    |         ^^^^^^^^^^^^^^^^^
2020-01-14T15:09:12.7186100Z 
2020-01-14T15:09:12.7186100Z 
2020-01-14T15:09:12.7186605Z error: symbol-name(_ZN209_$LT$$u5b$$RF$dyn$u20$impl1..Foo$u2b$Assoc$u20$$u3d$$u20$extern$u20$$u22$C$u22$$u20$fn$LP$$RF$u8$C$$u20$...$RP$$u2b$impl1..AutoTrait$u3b$$u20$3$u5d$$u20$as$u20$impl1..main..$u7b$$u7b$closure$u7d$$u7d$..Bar$GT$6method17hdb62078998ce7ea8E)
2020-01-14T15:09:12.7187014Z    |
2020-01-14T15:09:12.7187077Z LL |             #[rustc_symbol_name]
2020-01-14T15:09:12.7187157Z    |             ^^^^^^^^^^^^^^^^^^^^
2020-01-14T15:09:12.7187199Z 
2020-01-14T15:09:12.7187199Z 
2020-01-14T15:09:12.7187304Z error: demangling(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; 3] as impl1::main::{{closure}}::Bar>::method::hdb62078998ce7ea8)
2020-01-14T15:09:12.7187671Z    |
2020-01-14T15:09:12.7187728Z LL |             #[rustc_symbol_name]
2020-01-14T15:09:12.7187805Z    |             ^^^^^^^^^^^^^^^^^^^^
2020-01-14T15:09:12.7187846Z 
2020-01-14T15:09:12.7187846Z 
2020-01-14T15:09:12.7188189Z error: demangling-alt(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; 3] as impl1::main::{{closure}}::Bar>::method)
2020-01-14T15:09:12.7188644Z    |
2020-01-14T15:09:12.7188717Z LL |             #[rustc_symbol_name]
2020-01-14T15:09:12.7188782Z    |             ^^^^^^^^^^^^^^^^^^^^
2020-01-14T15:09:12.7188820Z 
2020-01-14T15:09:12.7188820Z 
2020-01-14T15:09:12.7189131Z error: def-path(<[&dyn Foo<Assoc = for<'r> extern "C" fn(&'r u8, ...)> + AutoTrait; 3] as main::{{closure}}#1::Bar>::method)
2020-01-14T15:09:12.7189462Z    |
2020-01-14T15:09:12.7189527Z LL |             #[rustc_def_path]
2020-01-14T15:09:12.7189587Z    |             ^^^^^^^^^^^^^^^^^
2020-01-14T15:09:12.7189625Z 
---
2020-01-14T15:09:12.7197776Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:387:22
2020-01-14T15:09:12.7197883Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-14T15:09:12.7204862Z 
2020-01-14T15:09:12.7205131Z 
2020-01-14T15:09:12.7207610Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-14T15:09:12.7208740Z 
2020-01-14T15:09:12.7217131Z 
2020-01-14T15:09:12.7217637Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
2020-01-14T15:09:12.7217775Z Build completed unsuccessfully in 1:15:27
2020-01-14T15:09:12.7217775Z Build completed unsuccessfully in 1:15:27
2020-01-14T15:09:12.7267634Z == clock drift check ==
2020-01-14T15:09:12.7288621Z   local time: Tue Jan 14 15:09:12 UTC 2020
2020-01-14T15:09:12.8344466Z   network time: Tue, 14 Jan 2020 15:09:12 GMT
2020-01-14T15:09:12.8344602Z == end clock drift check ==
2020-01-14T15:09:13.2522084Z 
2020-01-14T15:09:13.2641973Z ##[error]Bash exited with code '1'.
2020-01-14T15:09:13.2686105Z ##[section]Starting: Checkout
2020-01-14T15:09:13.2687869Z ==============================================================================
2020-01-14T15:09:13.2687948Z Task         : Get sources
2020-01-14T15:09:13.2688035Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
