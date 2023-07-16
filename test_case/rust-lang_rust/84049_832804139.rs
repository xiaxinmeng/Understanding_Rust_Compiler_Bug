
Building build.rs (glutin_egl_sys)
Running rustc --crate-name build_script_build build.rs --crate-type bin -C opt-level=3 -C codegen-units=8 -C incremental=no --edition 2018 --cfg feature="default" --out-dir target/build/glutin_egl_sys --emit=dep-info,link -L dependency=target/buildDeps --extern gl_generator=/nix/store/m2axask7avh3fsb35wscsk4k9zmass56-rust_gl_generator-0.14.0-lib/lib/libgl_generator-637e653e86.rlib --cap-lints allow --color always
thread 'rustc' panicked at 'Failed to get crate data for crate18', compiler/rustc_metadata/src/creader.rs:136:32
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-nightly (f82664191 2021-03-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -C opt-level=3 -C codegen-units=8 -C incremental --crate-type bin

query stack during panic:
end of query stack
