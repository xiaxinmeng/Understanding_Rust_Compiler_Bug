plain
   Compiling structopt-derive v0.4.9
   Compiling rustc-ap-rustc_macros v705.0.0
   Compiling thiserror v1.0.20
   Compiling rustc-rayon v0.3.0
thread 'rustc' panicked at 'assertion failed: variants.raw.is_sorted_by_key(|v| v.def_id)', compiler/rustc_middle/src/ty/mod.rs:2302:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (80de824aa 2021-02-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z tls-model=initial-exec -Z binary-dep-depinfo -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C linker=clang -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [adt_def] computing ADT definition for `diagnostic::DiagnosticLevel`
#1 [type_of] computing type of `diagnostic::DiagnosticLevel`
error: could not compile `cargo_metadata`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
---
   Compiling clippy v0.1.52 (/checkout/src/tools/clippy)
   Compiling semver-parser v0.10.1
   Compiling semver v0.11.0
   Compiling cargo_metadata v0.12.0
thread 'rustc' panicked at 'assertion failed: variants.raw.is_sorted_by_key(|v| v.def_id)', compiler/rustc_middle/src/ty/mod.rs:2302:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (80de824aa 2021-02-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z tls-model=initial-exec -Z binary-dep-depinfo -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C linker=clang -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [adt_def] computing ADT definition for `diagnostic::DiagnosticLevel`
#1 [type_of] computing type of `diagnostic::DiagnosticLevel`  |  = note: this failure-note originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)
end of query stack
error: could not compile `cargo_metadata`

To learn more, run the command again with --verbose.
---
   Compiling mio-uds v0.6.8
   Compiling cargo_metadata v0.12.0
   Compiling cargo_metadata v0.8.2
   Compiling lsp-types v0.60.0
thread 'rustc' panicked at 'assertion failed: variants.raw.is_sorted_by_key(|v| v.def_id)', compiler/rustc_middle/src/ty/mod.rs:2302:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (80de824aa 2021-02-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z tls-model=initial-exec -Z binary-dep-depinfo -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C linker=clang -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [adt_def] computing ADT definition for `diagnostic::DiagnosticLevel`
#1 [type_of] computing type of `diagnostic::DiagnosticLevel`
error: could not compile `cargo_metadata`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
warning: build failed, waiting for other jobs to finish...
thread 'rustc' panicked at 'assertion failed: variants.raw.is_sorted_by_key(|v| v.def_id)', compiler/rustc_middle/src/ty/mod.rs:2302:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (80de824aa 2021-02-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z tls-model=initial-exec -Z binary-dep-depinfo -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C linker=clang -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [adt_def] computing ADT definition for `diagnostic::DiagnosticLevel`
#1 [type_of] computing type of `diagnostic::DiagnosticLevel`  |  = note: this failure-note originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)
end of query stack
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rls/Cargo.toml" "--features" "clippy, rustc-workspace-hack/all-static" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
---
   Compiling cargo_metadata v0.12.3
   Compiling lsp-types v0.86.0
   Compiling lsp-server v0.5.0
   Compiling syntax v0.0.0 (/checkout/src/tools/rust-analyzer/crates/syntax)
thread 'rustc' panicked at 'assertion failed: variants.raw.is_sorted_by_key(|v| v.def_id)', compiler/rustc_middle/src/ty/mod.rs:2302:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (80de824aa 2021-02-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z tls-model=initial-exec -Z binary-dep-depinfo -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C linker=clang -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [adt_def] computing ADT definition for `diagnostic::DiagnosticLevel`
#1 [type_of] computing type of `diagnostic::DiagnosticLevel`  |  = note: this failure-note originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)
end of query stack
error: could not compile `cargo_metadata`

To learn more, run the command again with --verbose.
To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rust-analyzer/crates/rust-analyzer/Cargo.toml" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
thread 'main' panicked at 'rust-analyzer always builds', src/bootstrap/dist.rs:1083:14
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata
Build completed unsuccessfully in 0:16:14
