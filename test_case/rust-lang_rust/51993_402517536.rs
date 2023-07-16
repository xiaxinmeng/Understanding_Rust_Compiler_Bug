plain
[00:21:30]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[00:21:33]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:21:33]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:21:37]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:21:54] error[E0597]: borrowed value does not live long enough
[00:21:54]     |
[00:21:54]     |
[00:21:54] 223 |                                     let expected = &[ $( stringify!($name) ),+ ];
[00:21:54]     |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ temporary value does not live long enough
[00:21:54] 229 |                                 }
[00:21:54] 229 |                                 }
[00:21:54]     |                                 - temporary value only lives until here
[00:21:54] ...
[00:21:54] 247 |                     get_meta!(since, reason);
[00:21:54]     |
[00:21:54]     |
[00:21:54]     = note: borrowed value must be valid for the static lifetime...
[00:21:54] 
[00:21:54] error[E0597]: borrowed value does not live long enough
[00:21:54]     |
[00:21:54]     |
[00:21:54] 223 |                                     let expected = &[ $( stringify!($name) ),+ ];
[00:21:54]     |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ temporary value does not live long enough
[00:21:54] 229 |                                 }
[00:21:54] 229 |                                 }
[00:21:54]     |                                 - temporary value only lives until here
[00:21:54] ...
[00:21:54] 273 |                     get_meta!(feature);
[00:21:54]     |
[00:21:54]     |
[00:21:54]     = note: borrowed value must be valid for the static lifetime...
[00:21:54] 
[00:21:54] error[E0597]: borrowed value does not live long enough
[00:21:54]     |
[00:21:54]     |
[00:21:54] 304 |                                             &["feature", "reason", "issue"]
[00:21:54]     |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ temporary value does not live long enough
[00:21:54] 306 |                                     );
[00:21:54] 306 |                                     );
[00:21:54]     |                                      - temporary value only lives until here
[00:21:54]     |
[00:21:54]     = note: borrowed value must be valid for the static lifetime...
[00:21:54]     = note: consider using a `let` binding to increase its lifetime
[00:21:54] 
[00:21:54] error[E0597]: borrowed value does not live long enough
[00:21:54]     |
[00:21:54]     |
[00:21:54] 363 |                                         AttrError::UnknownMetaItem(mi.name(), &["since", "note"]),
[00:21:54]     |                                                                                ^^^^^^^^^^^^^^^^^ temporary value does not live long enough
[00:21:54] 364 |                                     );
[00:21:54]     |                                      - temporary value only lives until here
[00:21:54]     |
[00:21:54]     = note: borrowed value must be valid for the static lifetime...
[00:21:54]     = note: consider using a `let` binding to increase its lifetime
[00:21:54] 
[00:21:54] error[E0597]: borrowed value does not live long enough
[00:21:54]     |
[00:21:54]     |
[00:21:54] 545 |                                 AttrError::UnknownMetaItem(mi.name(), &["since", "note"]),
[00:21:54]     |                                                                        ^^^^^^^^^^^^^^^^^ temporary value does not live long enough
[00:21:54] 546 |                             );
[00:21:54]     |                              - temporary value only lives until here
[00:21:54]     |
[00:21:54]     = note: borrowed value must be valid for the static lifetime...
[00:21:54]     = note: consider using a `let` binding to increase its lifetime
[00:21:56] error: aborting due to 5 previous errors
[00:21:56] 
[00:21:56] For more information about this error, try `rustc --explain E0597`.
[00:21:56] error: Could not compile `syntax`.
[00:21:56] error: Could not compile `syntax`.
[00:21:56] 
[00:21:56] Caused by:
[00:21:56]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name syntax libsyntax/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=6520f21211d1c6e4 -C extra-filename=-6520f21211d1c6e4 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-1d65b3b5f2314396.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-babaf616bce40eec.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-6581c00e27569018.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-b18c765052ec79ac.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-98bbb0bd2515be61.so --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-a8440fe8f7c793aa.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-4b189baa036bcbee.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-4b189baa036bcbee.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-ec3cf21378a53dee.so` (exit code: 101)
[00:21:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:21:56] expected success, got: exit code: 101
[00:21:56] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1114:9
[00:21:56] travis_fold:end:stage1-rustc

[00:21:56] travis_time:end:stage1-rustc:start=1530720183059121983,finish=1530720247338541848,duration=64279419865


[00:21:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:21:56] Build completed unsuccessfully in 0:17:26
[00:21:56] make: *** [all] Error 1
[00:21:56] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:003b5e66
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
