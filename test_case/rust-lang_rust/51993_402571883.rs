plain
[00:22:17]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[00:22:20]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:22:20]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:22:23]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:22:41] error[E0597]: borrowed value does not live long enough
[00:22:41]     |
[00:22:41]     |
[00:22:41] 223 |                                     let expected = &[ $( stringify!($name) ),+ ];
[00:22:41]     |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ temporary value does not live long enough
[00:22:41] 229 |                                 }
[00:22:41] 229 |                                 }
[00:22:41]     |                                 - temporary value only lives until here
[00:22:41] ...
[00:22:41] 247 |                     get_meta!(since, reason);
[00:22:41]     |
[00:22:41]     |
[00:22:41]     = note: borrowed value must be valid for the static lifetime...
[00:22:41] 
[00:22:41] error[E0597]: borrowed value does not live long enough
[00:22:41]    --> libsyntax/at               &["feature", "reason", "issue"]
[00:22:41]     |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ temporary value does not live long enough
[00:22:41] 306 |                                     );
[00:22:41] 306 |                                     );
[00:22:41]     |                                      - temporary value only lives until here
[00:22:41]     |
[00:22:41]     = note: borrowed value must be valid for the static lifetime...
[00:22:41]     = note: consider using a `let` binding to increase its lifetime
[00:22:41] 
[00:22:41] error[E0597]: borrowed value does not live long enough
[00:22:41]     |
[00:22:41]     |
[00:22:41] 363 |                                         AttrError::UnknownMetaItem(mi.name(), &["since", "note"]),
[00:22:41]     |                                                                                ^^^^^^^^^^^^^^^^^ temporary value does0m= note: borrowed value must be valid for the static lifetime...
[00:22:41]     = note: consider using a `let` binding to increase its lifetime
[00:22:43] error: aborting due to 5 previous errors
[00:22:43] 
[00:22:43] For more information about this error, try `rustc --explain E0597`.
[00:22:43] error: Could not compile `syntax`.
[00:22:43] error: Could not compile `syntax`.
[00:22:43] 
[00:22:43] Caused by:
[00:22:43]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name syntax libsyntax/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=6520f21211d1c6e4 -C extra-filename=-6520f21211d1c6e4 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-1d65b3b5f2314396.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-babaf616bce40eec.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-6581c00e27569018.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-b18c765052ec79ac.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-98bbb0bd2515be61.so --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-a8440fe8f7c793aa.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-4b189baa036bcbee.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-4b189baa036bcbee.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-ec3cf21378a53dee.so` (exit code: 101)
[00:22:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:22:43] expected success, got: exit code: 101
[00:22:43] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1114:9
[00:22:43] travis_fold:end:stage1-rustc

[00:22:43] travis_time:end:stage1-rustc:start=1530747139412990227,finish=1530747205558881184,duration=66145890957


[00:22:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:22:43] Build completed unsuccessfully in 0:17:52
[00:22:43] Makefile:28: recipe for target 'all' failed
[00:22:43] make: *** [all] Error 1
