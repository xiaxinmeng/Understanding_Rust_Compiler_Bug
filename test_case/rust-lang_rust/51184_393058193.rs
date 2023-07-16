plain
[00:06:36]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[00:06:39]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:06:39]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:06:44]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:06:58] error[E0614]: type `bool` cannot be dereferenced
[00:06:58]    --> libsyntax/parse/parser.rs:770:69
[00:06:58]     |
[00:06:58] 770 |             if self.token == token::Comma && self.look_ahead(1, |t| *t.is_ident()) {
[00:06:58] 
[00:07:01] error: aborting due to previous error
[00:07:01] 
[00:07:01] For more information about this error, try `rustc --explain E0614`.
[00:07:01] For more information about this error, try `rustc --explain E0614`.
[00:07:01] error: Could not compile `syntax`.
[00:07:01] 
[00:07:01] Caused by:
[00:07:01]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name syntax libsyntax/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=4074300b8e2af006 -C extra-filename=-4074300b8e2af006 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-99b4534960f92449.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-5ebb1e8ea909a7e0.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5d0a8a65bb9fe29f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5d0a8a65bb9fe29f.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/releaSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_time:end:19347018:start=1527665196084087004,finish=1527665196090672198,duration=6585194
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:08c7f451
