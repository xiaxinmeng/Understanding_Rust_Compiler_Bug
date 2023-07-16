plain
Receiving objects: 100% (102/102), 11.39 KiB | 11.39 MiB/s, done.
---
Resolving deltas: 100% (97/97), completed with 89 local objects.
---
[00:00:45] configure: rust.quiet-tests     := True
---
[00:13:25] 115 |                     ty::BrNamed(id, name)))
[00:13:25]     |                                     ^^^^ expected struct `syntax::symbol::InternedString`, found struct `syntax::symbol::LocalInternedString`
---
[00:13:25] 127 |                     name,
[00:13:25]     |                     ^^^^ expected struct `syntax::symbol::InternedString`, found struct `syntax::symbol::LocalInternedString`
---
[00:13:25] 135 |                     bound_region: ty::BrNamed(id, name)
[00:13:25]     |                                                   ^^^^ expected struct `syntax::symbol::InternedString`, found struct `syntax::symbol::LocalInternedString`
---
[00:13:31] 889 |             name: l.lifetime.name.name().as_str(),
[00:13:31]     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `syntax::symbol::InternedString`, found struct `syntax::symbol::LocalInternedString`
---
[00:13:31] 1430 |             name: param.lifetime.name.name().as_str(),
[00:13:31]      |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `syntax::symbol::InternedString`, found struct `syntax::symbol::LocalInternedString`
---
[00:13:31]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_typeck librustc_typeck/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=d7a9eeb5d1acb48b -C extra-filename=-d7a9eeb5d1acb48b --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-3706e912fdb98df1.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-93cb1ddd29ab61a4.so --extern rustc_const_math=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_const_math-895f7ddc4467bb8d.so --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-f2778bd6cd1c5bdd.so --extern rustc_platform_intrinsics=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_platform_intrinsics-6f259a2a9f59267f.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-23f5dcecdda8feb4.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-609e2421d03f9c9a.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-3e35efb9e5bc7a9a.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-d3b6fcf798f7d22a.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-9f3518d56a01456f.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-8b35e3c2ea935fab/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-63734d0048644b22/out` (exit code: 101)
---
121760 ./obj/build/bootstrap/debug/incremental/bootstrap-351vorei3hhuv/s-f059e4u5y0-1ocolbt-3ssff2uugm5rt
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:1322ddb0:start=1523812471589806466,finish=1523812471597800232,duration=7993766
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:1346d13a
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name '
