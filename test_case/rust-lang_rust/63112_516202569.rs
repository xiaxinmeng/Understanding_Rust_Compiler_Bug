plain
2019-07-29T21:38:52.1867088Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-29T21:38:52.1867139Z 
2019-07-29T21:38:52.1867356Z   git checkout -b <new-branch-name>
2019-07-29T21:38:52.1867393Z 
2019-07-29T21:38:52.1867637Z HEAD is now at db5aca697 Auto merge of #63112 - Centril:rollup-kjgv7ak, r=Centril
2019-07-29T21:38:52.2105356Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-29T21:38:52.2108149Z ==============================================================================
2019-07-29T21:38:52.2108226Z Task         : Bash
2019-07-29T21:38:52.2108944Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-29T23:39:05.2675366Z test workspaces::ws_warn_unused ... ok
2019-07-29T23:39:05.2682222Z 
2019-07-29T23:39:05.2682373Z failures:
2019-07-29T23:39:05.2682413Z 
2019-07-29T23:39:05.2683160Z ---- bench::bench_bench_implicit stdout ----
2019-07-29T23:39:05.2683544Z running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo bench --benches`
2019-07-29T23:39:05.2683861Z thread 'bench::bench_bench_implicit' panicked at '
2019-07-29T23:39:05.2683942Z Expected: execs
2019-07-29T23:39:05.2684025Z     but: exited with exit code: 101
2019-07-29T23:39:05.2684248Z --- stdout
2019-07-29T23:39:05.2684513Z --- stderr
2019-07-29T23:39:05.2684513Z --- stderr
2019-07-29T23:39:05.2684885Z    Compiling foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t126/foo)
2019-07-29T23:39:05.2685421Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-07-29T23:39:05.2685712Z   |
2019-07-29T23:39:05.2685712Z   |
2019-07-29T23:39:05.2685789Z 5 |             #[bench] fn run1(_ben: &mut test::Bencher) { }
2019-07-29T23:39:05.2686092Z   |
2019-07-29T23:39:05.2686340Z   = note: for more information, see https://github.com/rust-lang/rust/issues/50297
2019-07-29T23:39:05.2686340Z   = note: for more information, see https://github.com/rust-lang/rust/issues/50297
2019-07-29T23:39:05.2686608Z   = help: add `#![feature(test)]` to the crate attributes to enable
2019-07-29T23:39:05.2686715Z error: aborting due to previous error
2019-07-29T23:39:05.2686751Z 
2019-07-29T23:39:05.2687141Z For more information about this error, try `rustc --explain E0658`.
2019-07-29T23:39:05.2687220Z error: Could not compile `foo`.
2019-07-29T23:39:05.2687220Z error: Could not compile `foo`.
2019-07-29T23:39:05.2687277Z warning: build failed, waiting for other jobs to finish...
2019-07-29T23:39:05.2687356Z error: build failed
2019-07-29T23:39:05.2687571Z ', src/tools/cargo/tests/testsuite/support/mod.rs:837:13
2019-07-29T23:39:05.2687656Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-29T23:39:05.2687697Z 
2019-07-29T23:39:05.2687896Z ---- bench::bench_dylib stdout ----
2019-07-29T23:39:05.2688157Z running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo bench -v`
2019-07-29T23:39:05.2688601Z thread 'bench::bench_dylib' panicked at '
2019-07-29T23:39:05.2688657Z Expected: execs
2019-07-29T23:39:05.2688724Z     but: exited with exit code: 101
2019-07-29T23:39:05.2688896Z --- stdout
2019-07-29T23:39:05.2689105Z --- stderr
2019-07-29T23:39:05.2689105Z --- stderr
2019-07-29T23:39:05.2689390Z    Compiling bar v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t128/foo/bar)
2019-07-29T23:39:05.2690063Z      Running `rustc --crate-name bar bar/src/lib.rs --color never --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 -C metadata=4f8d0978b3215d2f --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t128/foo/target/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t128/foo/target/release/deps`
2019-07-29T23:39:05.2690483Z    Compiling foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t128/foo)
2019-07-29T23:39:05.2691190Z      Running `rustc --crate-name foo src/lib.rs --color never --crate-type dylib --emit=dep-info,link -C opt-level=3 -C metadata=4ee8a8298a9c578d --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t128/foo/target/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t128/foo/target/release/deps --extern bar=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t128/foo/target/release/deps/libbar.so`
2019-07-29T23:39:05.2692794Z      Running `rustc --crate-name foo src/lib.rs --color never --emit=dep-info,link -C opt-level=3 --test -C metadata=76646e16f6a2657e -C extra-filename=-76646e16f6a2657e --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t128/foo/target/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t128/foo/target/release/deps --extern bar=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t128/foo/target/release/deps/libbar.so`
2019-07-29T23:39:05.2693356Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-07-29T23:39:05.2693700Z   |
2019-07-29T23:39:05.2693700Z   |
2019-07-29T23:39:05.2693760Z 9 |             #[bench]
2019-07-29T23:39:05.2693904Z   |
2019-07-29T23:39:05.2694222Z   = note: for more information, see https://github.com/rust-lang/rust/issues/50297
2019-07-29T23:39:05.2694222Z   = note: for more information, see https://github.com/rust-lang/rust/issues/50297
2019-07-29T23:39:05.2694318Z   = help: add `#![feature(test)]` to the crate attributes to enable
2019-07-29T23:39:05.2694448Z error: aborting due to previous error
2019-07-29T23:39:05.2694508Z 
2019-07-29T23:39:05.2694785Z For more information about this error, try `rustc --explain E0658`.
2019-07-29T23:39:05.2694892Z error: Could not compile `foo`.
2019-07-29T23:39:05.2694892Z error: Could not compile `foo`.
2019-07-29T23:39:05.2694936Z 
2019-07-29T23:39:05.2694994Z Caused by:
2019-07-29T23:39:05.2696176Z   process didn't exit successfully: `rustc --crate-name foo src/lib.rs --color never --crate-type dylib --emit=dep-info,link -C opt-level=3 -C metadata=4ee8a8298a9c578d --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t128/foo/target/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t128/foo/target/release/deps --extern bar=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t128/foo/target/release/deps/libbar.so` (exit code: 1)
2019-07-29T23:39:05.2696437Z error: build failed
2019-07-29T23:39:05.2696670Z ', src/tools/cargo/tests/testsuite/support/mod.rs:837:13
2019-07-29T23:39:05.2696808Z 
2019-07-29T23:39:05.2696808Z 
2019-07-29T23:39:05.2697048Z ---- bench::bench_with_deep_lib_dep stdout ----
2019-07-29T23:39:05.2697313Z running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo bench`
2019-07-29T23:39:05.2697553Z thread 'bench::bench_with_deep_lib_dep' panicked at '
2019-07-29T23:39:05.2697614Z Expected: execs
2019-07-29T23:39:05.2697680Z     but: exited with exit code: 101
2019-07-29T23:39:05.2697849Z --- stdout
2019-07-29T23:39:05.2698060Z --- stderr
2019-07-29T23:39:05.2698060Z --- stderr
2019-07-29T23:39:05.2698405Z    Compiling foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t133/foo)
2019-07-29T23:39:05.2698721Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-07-29T23:39:05.2699020Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t133/foo/src/lib.rs:8:15
2019-07-29T23:39:05.2699092Z   |
2019-07-29T23:39:05.2699166Z 8 |             #[bench]
2019-07-29T23:39:05.2699285Z   |
2019-07-29T23:39:05.2699547Z   = note: for more information, see https://github.com/rust-lang/rust/issues/50297
2019-07-29T23:39:05.2699547Z   = note: for more information, see https://github.com/rust-lang/rust/issues/50297
2019-07-29T23:39:05.2699641Z   = help: add `#![feature(test)]` to the crate attributes to enable
2019-07-29T23:39:05.2699747Z error: aborting due to previous error
2019-07-29T23:39:05.2699784Z 
2019-07-29T23:39:05.2700017Z For more information about this error, try `rustc --explain E0658`.
2019-07-29T23:39:05.2700100Z error: Could not compile `foo`.
2019-07-29T23:39:05.2700100Z error: Could not compile `foo`.
2019-07-29T23:39:05.2700135Z 
2019-07-29T23:39:05.2700372Z To learn more, run the command again with --verbose.
2019-07-29T23:39:05.2700609Z ', src/tools/cargo/tests/testsuite/support/mod.rs:837:13
2019-07-29T23:39:05.2700668Z 
2019-07-29T23:39:05.2700869Z ---- bench::bench_with_examples stdout ----
2019-07-29T23:39:05.2701162Z running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo bench -v`
2019-07-29T23:39:05.2701409Z thread 'bench::bench_with_examples' panicked at '
2019-07-29T23:39:05.2701485Z Expected: execs
2019-07-29T23:39:05.2701719Z     but: exited with exit code: 101
2019-07-29T23:39:05.2702330Z --- stdout
2019-07-29T23:39:05.2702585Z --- stderr
2019-07-29T23:39:05.2702585Z --- stderr
2019-07-29T23:39:05.2702948Z    Compiling foo v6.6.6 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t134/foo)
2019-07-29T23:39:05.2703740Z      Running `rustc --crate-name foo src/lib.rs --color never --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C metadata=b952d6eb35de49af -C extra-filename=-b952d6eb35de49af --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t134/foo/target/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t134/foo/target/release/deps`
2019-07-29T23:39:05.2704602Z      Running `rustc --crate-name foo src/lib.rs --color never --emit=dep-info,link -C opt-level=3 --test -C metadata=d9b3f2cf2ad0014b -C extra-filename=-d9b3f2cf2ad0014b --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t134/foo/target/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t134/foo/target/release/deps`
2019-07-29T23:39:05.2705071Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-07-29T23:39:05.2705435Z    |
2019-07-29T23:39:05.2705435Z    |
2019-07-29T23:39:05.2705662Z 14 |             #[bench]
2019-07-29T23:39:05.2705780Z    |
2019-07-29T23:39:05.2706210Z    = note: for more information, see https://github.com/rust-lang/rust/issues/50297
2019-07-29T23:39:05.2706210Z    = note: for more information, see https://github.com/rust-lang/rust/issues/50297
2019-07-29T23:39:05.2706285Z    = help: add `#![feature(test)]` to the crate attributes to enable
2019-07-29T23:39:05.2706487Z error: aborting due to previous error
2019-07-29T23:39:05.2706522Z 
2019-07-29T23:39:05.2706784Z For more information about this error, try `rustc --explain E0658`.
2019-07-29T23:39:05.2706847Z error: Could not compile `foo`.
2019-07-29T23:39:05.2706847Z error: Could not compile `foo`.
2019-07-29T23:39:05.2706879Z 
2019-07-29T23:39:05.2706940Z Caused by:
2019-07-29T23:39:05.2707620Z   process didn't exit successfully: `rustc --crate-name foo src/lib.rs --color never --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C metadata=b952d6eb35de49af -C extra-filename=-b952d6eb35de49af --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t134/foo/target/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t134/foo/target/release/deps` (exit code: 1)
2019-07-29T23:39:05.2707870Z error: build failed
2019-07-29T23:39:05.2708110Z ', src/tools/cargo/tests/testsuite/support/mod.rs:837:13
2019-07-29T23:39:05.2708177Z 
2019-07-29T23:39:05.2708177Z 
2019-07-29T23:39:05.2708369Z ---- bench::bench_with_lib_dep stdout ----
2019-07-29T23:39:05.2708644Z running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo bench`
2019-07-29T23:39:05.2708863Z thread 'bench::bench_with_lib_dep' panicked at '
2019-07-29T23:39:05.2708940Z Expected: execs
2019-07-29T23:39:05.2708988Z     but: exited with exit code: 101
2019-07-29T23:39:05.2709174Z --- stdout
2019-07-29T23:39:05.2709383Z --- stderr
2019-07-29T23:39:05.2709383Z --- stderr
2019-07-29T23:39:05.2709654Z    Compiling foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t135/foo)
2019-07-29T23:39:05.2709955Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-07-29T23:39:05.2710218Z    |
2019-07-29T23:39:05.2710218Z    |
2019-07-29T23:39:05.2710272Z 14 |             #[bench] fn lib_bench(_b: &mut test::Bencher) {}
2019-07-29T23:39:05.2711447Z    |
2019-07-29T23:39:05.2712361Z    = note: for more information, see https://github.com/rust-lang/rust/issues/50297
2019-07-29T23:39:05.2712361Z    = note: for more information, see https://github.com/rust-lang/rust/issues/50297
2019-07-29T23:39:05.2712463Z    = help: add `#![feature(test)]` to the crate attributes to enable
2019-07-29T23:39:05.2712598Z error: aborting due to previous error
2019-07-29T23:39:05.2712640Z 
2019-07-29T23:39:05.2713056Z For more information about this error, try `rustc --explain E0658`.
2019-07-29T23:39:05.2713154Z error: Could not compile `foo`.
2019-07-29T23:39:05.2713154Z error: Could not compile `foo`.
2019-07-29T23:39:05.2713241Z warning: build failed, waiting for other jobs to finish...
2019-07-29T23:39:05.2713331Z error: build failed
2019-07-29T23:39:05.2713604Z ', src/tools/cargo/tests/testsuite/support/mod.rs:837:13
2019-07-29T23:39:05.2713655Z 
2019-07-29T23:39:05.2714645Z ---- bench::external_bench_explicit stdout ----
2019-07-29T23:39:05.2715063Z running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo bench`
2019-07-29T23:39:05.2715394Z thread 'bench::external_bench_explicit' panicked at '
2019-07-29T23:39:05.2715470Z Expected: execs
2019-07-29T23:39:05.2715558Z     but: exited with exit code: 101
2019-07-29T23:39:05.2715918Z --- stdout
2019-07-29T23:39:05.2716135Z --- stderr
2019-07-29T23:39:05.2716135Z --- stderr
2019-07-29T23:39:05.2716426Z    Compiling foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t141/foo)
2019-07-29T23:39:05.2716713Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-07-29T23:39:05.2716995Z   |
2019-07-29T23:39:05.2716995Z   |
2019-07-29T23:39:05.2717061Z 7 |             #[bench]
2019-07-29T23:39:05.2717177Z   |
2019-07-29T23:39:05.2717413Z   = note: for more information, see https://github.com/rust-lang/rust/issues/50297
2019-07-29T23:39:05.2717413Z   = note: for more information, see https://github.com/rust-lang/rust/issues/50297
2019-07-29T23:39:05.2717507Z   = help: add `#![feature(test)]` to the crate attributes to enable
2019-07-29T23:39:05.2717748Z error: aborting due to previous error
2019-07-29T23:39:05.2717782Z 
2019-07-29T23:39:05.2718037Z For more information about this error, try `rustc --explain E0658`.
2019-07-29T23:39:05.2718118Z error: Could not compile `foo`.
2019-07-29T23:39:05.2718118Z error: Could not compile `foo`.
2019-07-29T23:39:05.2718177Z warning: build failed, waiting for other jobs to finish...
2019-07-29T23:39:05.2718250Z error: build failed
2019-07-29T23:39:05.2718477Z ', src/tools/cargo/tests/testsuite/support/mod.rs:837:13
2019-07-29T23:39:05.2718518Z 
2019-07-29T23:39:05.2718788Z ---- bench::external_bench_implicit stdout ----
2019-07-29T23:39:05.2719110Z running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo bench`
2019-07-29T23:39:05.2719335Z thread 'bench::external_bench_implicit' panicked at '
2019-07-29T23:39:05.2719411Z Expected: execs
2019-07-29T23:39:05.2719460Z     but: exited with exit code: 101
2019-07-29T23:39:05.2719663Z --- stdout
2019-07-29T23:39:05.2719877Z --- stderr
2019-07-29T23:39:05.2719877Z --- stderr
2019-07-29T23:39:05.2720147Z    Compiling foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t142/foo)
2019-07-29T23:39:05.2720444Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-07-29T23:39:05.2720715Z   |
2019-07-29T23:39:05.2720715Z   |
2019-07-29T23:39:05.2720761Z 8 |             #[bench]
2019-07-29T23:39:05.2720875Z   |
2019-07-29T23:39:05.2721129Z   = note: for more information, see https://github.com/rust-lang/rust/issues/50297
2019-07-29T23:39:05.2721129Z   = note: for more information, see https://github.com/rust-lang/rust/issues/50297
2019-07-29T23:39:05.2721204Z   = help: add `#![feature(test)]` to the crate attributes to enable
2019-07-29T23:39:05.2721309Z error: aborting due to previous error
2019-07-29T23:39:05.2721343Z 
2019-07-29T23:39:05.2722250Z For more information about this error, try `rustc --explain E0658`.
2019-07-29T23:39:05.2722351Z error: Could not compile `foo`.
2019-07-29T23:39:05.2722351Z error: Could not compile `foo`.
2019-07-29T23:39:05.2722474Z warning: build failed, waiting for other jobs to finish...
2019-07-29T23:39:05.2722547Z error: build failed
2019-07-29T23:39:05.2722875Z ', src/tools/cargo/tests/testsuite/support/mod.rs:837:13
2019-07-29T23:39:05.2724467Z 
2019-07-29T23:39:05.2724835Z ---- bench::lib_bin_same_name stdout ----
2019-07-29T23:39:05.2725173Z running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo bench`
2019-07-29T23:39:05.2725615Z thread 'bench::lib_bin_same_name' panicked at '
2019-07-29T23:39:05.2725686Z Expected: execs
2019-07-29T23:39:05.2725915Z     but: exited with exit code: 101
2019-07-29T23:39:05.2727635Z --- stdout
2019-07-29T23:39:05.2727937Z --- stderr
2019-07-29T23:39:05.2727937Z --- stderr
2019-07-29T23:39:05.2728505Z    Compiling foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t145/foo)
2019-07-29T23:39:05.2735015Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-07-29T23:39:05.2735656Z   |
2019-07-29T23:39:05.2735656Z   |
2019-07-29T23:39:05.2735741Z 5 |             #[bench] fn lib_bench(_b: &mut test::Bencher) {}
2019-07-29T23:39:05.2735887Z   |
2019-07-29T23:39:05.2736166Z   = note: for more information, see https://github.com/rust-lang/rust/issues/50297
2019-07-29T23:39:05.2736166Z   = note: for more information, see https://github.com/rust-lang/rust/issues/50297
2019-07-29T23:39:05.2739136Z   = help: add `#![feature(test)]` to the crate attributes to enable
2019-07-29T23:39:05.2739295Z error: aborting due to previous error
2019-07-29T23:39:05.2739338Z 
2019-07-29T23:39:05.2739708Z For more information about this error, try `rustc --explain E0658`.
2019-07-29T23:39:05.2739978Z error: Could not compile `foo`.
2019-07-29T23:39:05.2739978Z error: Could not compile `foo`.
2019-07-29T23:39:05.2740053Z warning: build failed, waiting for other jobs to finish...
2019-07-29T23:39:05.2740145Z error: build failed
2019-07-29T23:39:05.2740637Z ', src/tools/cargo/tests/testsuite/support/mod.rs:837:13
2019-07-29T23:39:05.2740708Z 
2019-07-29T23:39:05.2740957Z ---- bench::lib_with_standard_name stdout ----
2019-07-29T23:39:05.2741309Z running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo bench`
2019-07-29T23:39:05.2742552Z thread 'bench::lib_with_standard_name' panicked at '
2019-07-29T23:39:05.2742656Z Expected: execs
2019-07-29T23:39:05.2742720Z     but: exited with exit code: 101
2019-07-29T23:39:05.2742959Z --- stdout
2019-07-29T23:39:05.2743222Z --- stderr
2019-07-29T23:39:05.2743704Z    Compiling syntax v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t146/foo)
2019-07-29T23:39:05.2743704Z    Compiling syntax v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t146/foo)
2019-07-29T23:39:05.2746190Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-07-29T23:39:05.2746868Z    |
2019-07-29T23:39:05.2746868Z    |
2019-07-29T23:39:05.2746941Z 11 |             #[bench]
2019-07-29T23:39:05.2747398Z    |
2019-07-29T23:39:05.2747866Z    = note: for more information, see https://github.com/rust-lang/rust/issues/50297
2019-07-29T23:39:05.2747866Z    = note: for more information, see https://github.com/rust-lang/rust/issues/50297
2019-07-29T23:39:05.2747955Z    = help: add `#![feature(test)]` to the crate attributes to enable
2019-07-29T23:39:05.2748341Z error: aborting due to previous error
2019-07-29T23:39:05.2748379Z 
2019-07-29T23:39:05.2748826Z For more information about this error, try `rustc --explain E0658`.
2019-07-29T23:39:05.2748902Z error: Could not compile `syntax`.
---
2019-07-29T23:39:05.2752998Z 
2019-07-29T23:39:05.2753314Z error: test failed, to rerun pass '--test testsuite'
2019-07-29T23:39:05.2757557Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/tools/cargo src/tools/cargotest
2019-07-29T23:39:05.2757708Z Build completed unsuccessfully in 1:55:19
2019-07-29T23:39:05.2816422Z Makefile:50: recipe for target 'check-aux' failed
2019-07-29T23:39:05.2819622Z make: *** [check-aux] Error 1
2019-07-29T23:39:10.2462391Z ##[error]Bash exited with code '2'.
2019-07-29T23:39:10.2508012Z ##[section]Starting: Upload CPU usage statistics
2019-07-29T23:39:10.2516064Z ==============================================================================
2019-07-29T23:39:10.2516166Z Task         : Bash
2019-07-29T23:39:10.2516397Z Description  : Run a Bash script on macOS, Linux, or Windows
