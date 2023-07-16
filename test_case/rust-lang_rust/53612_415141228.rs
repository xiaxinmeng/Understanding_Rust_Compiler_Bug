plain
[00:06:44]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[00:06:47]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:06:47]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:06:50]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:07:06] error[E0502]: cannot borrow `self` as immutable because `*self` is also borrowed as mutable
[00:07:06]      |
[00:07:06]      |
[00:07:06] 1374 |             let d = self.parse_fn_decl_with_self(|p: &mut Parser<'a>| {
[00:07:06]      |                     |
[00:07:06]      |                     mutable borrow occurs here
[00:07:06] ...
[00:07:06] ...
[00:07:06] 1380 |                 if self.span.edition() >= Edition::Edition2018 {
[00:07:06]      |                    ---- borrow occurs due to use of `self` in closure
[00:07:06] 1385 |             })?;
[00:07:06] 1385 |             })?;
[00:07:06]      |              - mutable borrow ends here
[00:07:07] error: aborting due to previous error
[00:07:07] 
[00:07:07] For more information about this error, try `rustc --explain E0502`.
[00:07:07] error: Could not compile `syntax`.
[00:07:07] error: Could not compile `syntax`.
[00:07:07] 
[00:07:07] Caused by:
[00:07:07]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name syntax libsyntax/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=64bbe8e4870170a3 -C extra-filename=-64bbe8e4870170a3 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-3907cba388d41ef0.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-b6c566856a1e65b9.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-8b624a6d6082b2ff.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-89eed8215142aadd.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/dWed, 22 Aug 2018 18:55:12 GMT

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
