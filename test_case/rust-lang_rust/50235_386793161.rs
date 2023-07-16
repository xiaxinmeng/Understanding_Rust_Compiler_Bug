plain
[00:05:20]    Compiling nodrop v0.1.12
[00:05:20]    Compiling memoffset v0.2.1
[00:05:20]    Compiling lazy_static v1.0.0
[00:05:20]    Compiling scopeguard v0.3.3
[00:05:20]    Compiling rustc-rayon-core v1.4.0 (https://github.com/Zoxc/rayon.git?branch=rustc#7874a154)
[00:05:20]    Compiling stable_deref_trait v1.0.0
[00:05:20]    Compiling smallvec v0.6.0
[00:05:20]    Compiling bitflags v1.0.1
[00:05:20]    Compiling either v1.5.0
---
[00:05:53]    Compiling crossbeam-deque v0.2.0
[00:05:54]    Compiling rls-data v0.15.0
[00:05:54]    Compiling flate2 v1.0.1
[00:06:00]    Compiling backtrace v0.3.6
[00:06:07]    Compiling rustc-rayon v1.0.1 (https://github.com/Zoxc/rayon.git?branch=rustc#7874a154)
[00:06:10]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[00:06:13] error[E0277]: the trait bound `str: std::cmp::PartialOrd<&str>` is not satisfied
[00:06:13]    --> libsyntax_pos/symbol.rs:506:62
[00:06:13]     |
[00:06:13] 506 |         self.with(|self_str| other.with(|other_str| self_str.partial_cmp(&other_str)))
[00:06:13]     |                                                              ^^^^^^^^^^^ can't compare `str` with `&str`
[00:06:13]     |
[00:06:13]     = help: the trait `std::cmp::PartialOrd<&str>` is not implemented for `str`
[00:06:13] error: aborting due to previous error
[00:06:13] 
[00:06:13] For more information about this error, try `rustc --explain E0277`.
[00:06:13] error: Could not compile `syntax_pos`.
[00:06:13] error: Could not compile `syntax_pos`.
[00:06:13] 
[00:06:13] Caused by:
[00:06:13]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name syntax_pos libsyntax_pos/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 -C metadata=6be905c10433596b -C extra-filename=-6be905c10433596b --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-ru

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1935214c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
