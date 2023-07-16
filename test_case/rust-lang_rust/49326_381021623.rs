plain
[00:06:06] 598 |                 ident_lhs.node == ident_rhs.node && is_raw_lhs == is_raw_rhs,
[00:06:06]     |                           ^^^^
[00:06:06]
[00:06:06] error[E0609]: no field `node` on type `&syntax_pos::symbol::Ident`
[00:06:06]    --> libsyntax/parse/token.rs:598:45
[00:06:06]     |
[00:06:06] 598 |                 ident_lhs.node == ident_rhs.node && is_raw_lhs == is_raw_rhs,
[00:06:06]     |                                             ^^^^
[00:06:06]
[00:06:06] error[E0609]: no field `ident` on type `&syntax_pos::symbol::Ident`
[00:06:06]    --> libsyntax/parse/token.rs:599:64
[00:06:06]     |
[00:06:06] 599 |             (NtLifetime(lt_lhs), NtLifetime(lt_rhs)) => lt_lhs.ident == lt_rhs.ident,
[00:06:06]     |                                                                ^^^^^
[00:06:06]
[00:06:06] error[E0609]: no field `ident` on type `&syntax_pos::symbol::Ident`
[00:06:06]    --> libsyntax/parse/token.rs:599:80
[00:06:06]     |
[00:06:06] 599 |             (NtLifetime(lt_lhs), NtLifetime(lt_rhs)) => lt_lhs.ident == lt_rhs.ident,
---
[00:06:07]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name syntax libsyntax/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=d3b6fcf798f7d22a -C extra-filename=-d3b6fcf798f7d22a --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-9866e194db82a141.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c04ded78717d5d67.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c04ded78717d5d67.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-3e35efb9e5bc7a9a.rlib --extern rustc_cratesio_shim=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_cratesio_shim-296054b03e3b45fe.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-9f3518d56a01456f.so --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-efdb8dd1548942a4.rlib --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-609e2421d03f9c9a.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-93cb1ddd29ab61a4.so` (exit code: 101)
[00:06:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:06:07] expected success, got: exit code: 101
[00:06:07] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1085:9
[00:06:07] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:06:07] travis_fold:end:stage0-rustc
[00:06:07] travis_time:end:stage0-rustc:start=1523594442600078449,finish=1523594498617288755,duration=56017210306
[00:06:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:06:07] Build completed unsuccessfully in 0:01:09
[00:06:07] Makefile:28: recipe for target 'all' failed
[00:06:07] make: *** [all] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:193e06d8:start=1523594499096262374,finish=1523594499116036039,duration=19773665
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:234d2762
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:234d2762:start=1523594499120806784,finish=1523594499126966360,duration=6159576
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05bb5c16
$ dmesg | grep -i kill
[   10.120576] init: failsafe main process (1093) killed by TERM signal
