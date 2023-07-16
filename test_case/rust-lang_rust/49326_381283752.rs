plain
[00:00:46] configure: rust.quiet-tests     := True
---
[00:06:14] 598 |                 ident_lhs.node == ident_rhs.node && is_raw_lhs == is_raw_rhs,
[00:06:14]     |                           ^^^^
[00:06:14]
[00:06:14] error[E0609]: no field `node` on type `&syntax_pos::symbol::Ident`
[00:06:14]    --> libsyntax/parse/token.rs:598:45
[00:06:14]     |
[00:06:14] 598 |                 ident_lhs.node == ident_rhs.node && is_raw_lhs == is_raw_rhs,
[00:06:14]     |                                             ^^^^
[00:06:14]
[00:06:14] error[E0609]: no field `ident` on type `&syntax_pos::symbol::Ident`
[00:06:14]    --> libsyntax/parse/token.rs:599:64
[00:06:14]     |
[00:06:14] 599 |             (NtLifetime(lt_lhs), NtLifetime(lt_rhs)) => lt_lhs.ident == lt_rhs.ident,
[00:06:14]     |                                                                ^^^^^
[00:06:14]
[00:06:14] error[E0609]: no field `ident` on type `&syntax_pos::symbol::Ident`
[00:06:14]    --> libsyntax/parse/token.rs:599:80
[00:06:14]     |
[00:06:14] 599 |             (NtLifetime(lt_lhs), NtLifetime(lt_rhs)) => lt_lhs.ident == lt_rhs.ident,
---
[00:06:16]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name syntax libsyntax/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=d3b6fcf798f7d22a -C extra-filename=-d3b6fcf798f7d22a --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-609e2421d03f9c9a.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-3e35efb9e5bc7a9a.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-9f3518d56a01456f.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-93cb1ddd29ab61a4.so --extern rustc_cratesio_shim=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-g
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:0caa2c80:start=1523662195618621066,finish=1523662195626682150,duration=8061084
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:22222a40
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:22222a40:start=1523662195633213946,finish=1523662195641322438,duration=8108492
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1bff6996
$ dmesg | grep -i kill
[   12.076477] init: failsafe main process (1096) killed by TERM signal
