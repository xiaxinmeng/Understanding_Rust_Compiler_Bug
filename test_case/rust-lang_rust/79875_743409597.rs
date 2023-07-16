
INFO: From Compiling Rust bin build_script_script_ (1 files):
--
  | thread 'rustc' panicked at 'Failed to recover key for type_of(f441f7c09333dbee-181350e87790bb85) with hash f441f7c09333dbee-181350e87790bb85', compiler/rustc_middle/src/ty/query/mod.rs:235:5
  | note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
  |  
  | error: internal compiler error: unexpected panic
  |  
  | note: the compiler unexpectedly panicked. this is a bug.
  |  
  | note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
  |  
  | note: rustc 1.50.0-nightly (f0f68778f 2020-12-09) running on x86_64-unknown-linux-gnu
  |  
  | note: compiler flags: -C opt-level=3 -C debuginfo=0 -C linker=/usr/bin/gcc -C link-args=-fuse-ld=gold -Wl,-no-as-needed -Wl,-z,relro,-z,now -B/usr/bin -pass-exit-codes -lstdc++ -lm -Wl,--gc-sections -C incremental --crate-type bin
  |  
  | note: some of the compiler flags provided by cargo are hidden
  |  
  | query stack during panic:
  | end of query stack
  | thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  | left: `LLVMing`,
  | right: `Codegenning`', /rustc/f0f68778f798d6d34649745b41770829b17ba5b8/compiler/rustc_codegen_ssa/src/back/write.rs:1425:21
  |  
  | error: internal compiler error: unexpected panic
  |  
  | note: the compiler unexpectedly panicked. this is a bug.
  |  
  | note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
  |  
  | note: rustc 1.50.0-nightly (f0f68778f 2020-12-09) running on x86_64-unknown-linux-gnu
  |  
  | note: compiler flags: -C opt-level=3 -C debuginfo=0 -C linker=/usr/bin/gcc -C link-args=-fuse-ld=gold -Wl,-no-as-needed -Wl,-z,relro,-z,now -B/usr/bin -pass-exit-codes -lstdc++ -lm -Wl,--gc-sections -C incremental --crate-type bin
  |  
  | note: some of the compiler flags provided by cargo are hidden
  |  
  | query stack during panic:
  | end of query stack
  | ERROR: common/BUILD:6:19: output 'common/build_script_script_' was not created
  | ERROR: common/BUILD:6:19: Couldn't build file common/build_script_script_: not all outputs were created or valid
  | INFO: From Compiling Rust bin build_script_script_ (1 files):
  | thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_metadata/src/rmeta/decoder.rs:1565:75
  | note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
  |  
  | error: internal compiler error: unexpected panic
  |  
  | note: the compiler unexpectedly panicked. this is a bug.
  |  
  | note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
  |  
  | note: rustc 1.50.0-nightly (f0f68778f 2020-12-09) running on x86_64-unknown-linux-gnu
  |  
  | note: compiler flags: -C opt-level=3 -C debuginfo=0 -C linker=/usr/bin/gcc -C link-args=-fuse-ld=gold -Wl,-no-as-needed -Wl,-z,relro,-z,now -B/usr/bin -pass-exit-codes -lstdc++ -lm -Wl,--gc-sections -C incremental --crate-type bin
  |  
  | note: some of the compiler flags provided by cargo are hidden
  |  
  | query stack during panic:
  | #0 [normalize_generic_arg_after_erasing_regions] normalizing `log::STATIC_MAX_LEVEL`
  | #1 [collect_and_partition_mono_items] collect_and_partition_mono_items
  | end of query stack
