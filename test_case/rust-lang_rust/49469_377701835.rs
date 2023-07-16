plain
Resolving deltas: 100% (613463/613463), completed with 4856 local objects.
---
[00:00:44] configure: rust.quiet-tests     := True
---
[00:05:35] error: no rules expected the token `)`
[00:05:35]    --> libsyntax/feature_gate.rs:452:60
[00:05:35]     |
[00:05:35] 452 |     (active, irrefutable_let_pattern, "1.27.0", Some(44495)),
---
[00:05:35]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name syntax libsyntax/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=7437cee1018df6d3 -C extra-filename=-7437cee1018df6d3 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-7eb604ad0d7f2749.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-7eebd272275b441b.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage03339082369,duration=52752305964
[00:05:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:05:35] Build completed unsuccessfully in 0:01:06
[00:05:35] make: *** [all] Error 1
[00:05:35] Makefile:28: recipe for target 'all' failed
