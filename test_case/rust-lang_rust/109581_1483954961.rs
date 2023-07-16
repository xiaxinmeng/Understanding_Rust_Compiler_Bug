plain
[RUSTC-TIMING] pathdiff test:false 0.220
[RUSTC-TIMING] rustc_demangle test:false 4.379
   Compiling cstr v0.2.8
[RUSTC-TIMING] syn test:false 20.779
rustc exited with signal: 11 (SIGSEGV)

Caused by:
Caused by:
  process didn't exit successfully: `/Users/runner/work/rust/rust/build/bootstrap/debug/rustc --crate-name syn --edition=2018 /Users/runner/.cargo/registry/src/index.crates.io-6f17d22bba15001f/syn-1.0.102/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=off --cfg 'feature="clone-impls"' --cfg 'feature="default"' --cfg 'feature="derive"' --cfg 'feature="extra-traits"' --cfg 'feature="fold"' --cfg 'feature="full"' --cfg 'feature="parsing"' --cfg 'feature="printing"' --cfg 'feature="proc-macro"' --cfg 'feature="quote"' --cfg 'feature="visit"' --cfg 'feature="visit-mut"' -Zunstable-options --check-cfg 'values(feature, "clone-impls", "default", "derive", "extra-traits", "fold", "full", "parsing", "printing", "proc-macro", "quote", "test", "visit", "visit-mut")' --check-cfg 'names()' --check-cfg 'values()' -C metadata=47e47c3c5b1ef108 -C extra-filename=-47e47c3c5b1ef108 --out-dir /Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/release/deps -L dependency=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/release/deps --extern proc_macro2=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/release/deps/libproc_macro2-be6ba8f82e325dcb.rmeta --extern quote=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/release/deps/libquote-026f507d17a7bdd6.rmeta --extern unicode_ident=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/release/deps/libunicode_ident-3590c8b0001c5439.rmeta --cap-lints allow -Z binary-dep-depinfo --cfg syn_disable_nightly_tests` (exit status: 254)
[RUSTC-TIMING] synstructure test:false 2.341
[RUSTC-TIMING] cstr test:false 0.900
[RUSTC-TIMING] regex_automata test:false 10.367
[RUSTC-TIMING] regex_syntax test:false 20.904
