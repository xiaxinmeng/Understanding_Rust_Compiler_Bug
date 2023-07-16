plain
[00:22:33]    Compiling ar v0.3.1
[00:22:33]    Compiling rustc_driver v0.0.0 (file:///checkout/src/librustc_driver)
[00:22:34]    Compiling crossbeam-utils v0.2.2
[00:22:34]    Compiling log v0.4.1
[00:22:34] error[E0912]: transmutation from a type with an unspecified layout
[00:22:34]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.2.2/src/scoped.rs:152:38
[00:22:34]     |
[00:22:34] 152 |     let closure: Box<FnBox + Send> = mem::transmute(closure);
[00:22:34]     |
[00:22:34]     |
[00:22:34]     = note: std::boxed::Box<scoped::FnBox> has an unspecified layout
[00:22:34] 
[00:22:34] 
[00:22:34] error[E0912]: transmutation to a type with an unspecified layout
[00:22:34]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.2.2/src/scoped.rs:152:38
[00:22:34]     |
[00:22:34] 152 |     let closure: Box<FnBox + Send> = mem::transmute(closure);
[00:22:34]     |
[00:22:34]     |
[00:22:34]     = note: std::boxed::Box<scoped::FnBox + std::marker::Send + 'static> has an unspecified layout
[00:22:34] 
[00:22:34] error: aborting due to 2 previous errors
[00:22:34] 
[00:22:34] For more information about this error, try `rustc --explain E0912`.
[00:22:34] For more information about this error, try `rustc --explain E0912`.
[00:22:34] error: Could not compile `crossbeam-utils`.
[00:22:34] 
[00:22:34] Caused by:
[00:22:34]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name crossbeam_utils /cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.2.2/src/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 --cfg feature="default" --cfg feature="use_std" -C metadata=8ae83e3bf68c561a -C extra-filename=-8ae83e3bf68c561a --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libcfg_if-b6ab5ac7f345265d.rlib --cap-lints allow` (exit code: 101)
