plain
 Documenting rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
error: unresolved link to `TRACKED`
   --> compiler/rustc_session/src/options.rs:61:12
    |
61  |           $( #[$top_level_attr] )*
...
...
97  | / top_level_options!(
99  | |     ///
99  | |     ///
100 | |     /// For each option, one has to specify how it behaves with regard to the
201 | |     }
202 | | );
    | |__- in this macro invocation
    |
    |
    = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
    = note: the link appears in this line:
            
            [TRACKED]
             ^^^^^^^
    = note: no item named `TRACKED` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `TRACKED_NO_CRATE_HASH`
   --> compiler/rustc_session/src/options.rs:61:12
    |
    |
61  |           $( #[$top_level_attr] )*
...
...
97  | / top_level_options!(
99  | |     ///
99  | |     ///
100 | |     /// For each option, one has to specify how it behaves with regard to the
201 | |     }
202 | | );
    | |__- in this macro invocation
    |
    |
    = note: the link appears in this line:
            
            [TRACKED_NO_CRATE_HASH]
             ^^^^^^^^^^^^^^^^^^^^^
    = note: no item named `TRACKED_NO_CRATE_HASH` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `TRACKED`
   --> compiler/rustc_session/src/options.rs:61:12
    |
    |
61  |           $( #[$top_level_attr] )*
...
...
97  | / top_level_options!(
99  | |     ///
99  | |     ///
100 | |     /// For each option, one has to specify how it behaves with regard to the
201 | |     }
202 | | );
    | |__- in this macro invocation
    |
    |
    = note: the link appears in this line:
            
            Same as [TRACKED], but will not affect the crate hash. This is useful for options that only
                     ^^^^^^^
    = note: no item named `TRACKED` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`


error: unresolved link to `UNTRACKED`
    |
    |
61  |           $( #[$top_level_attr] )*
...
...
97  | / top_level_options!(
99  | |     ///
99  | |     ///
100 | |     /// For each option, one has to specify how it behaves with regard to the
201 | |     }
202 | | );
    | |__- in this macro invocation
    |
    |
    = note: the link appears in this line:
            [UNTRACKED]
             ^^^^^^^^^
    = note: no item named `UNTRACKED` in scope
    = note: no item named `UNTRACKED` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`


error: unresolved link to `SUBSTRUCT`
    |
    |
61  |           $( #[$top_level_attr] )*
...
...
97  | / top_level_options!(
99  | |     ///
99  | |     ///
100 | |     /// For each option, one has to specify how it behaves with regard to the
201 | |     }
202 | | );
    | |__- in this macro invocation
    |
    |
    = note: the link appears in this line:
            
            [SUBSTRUCT]
             ^^^^^^^^^
    = note: no item named `SUBSTRUCT` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `TRACKED`
   --> compiler/rustc_session/src/options.rs:61:12
    |
    |
61  |           $( #[$top_level_attr] )*
...
...
97  | / top_level_options!(
99  | |     ///
99  | |     ///
100 | |     /// For each option, one has to specify how it behaves with regard to the
201 | |     }
202 | | );
    | |__- in this macro invocation
    |
    |
    = note: the link appears in this line:
            
            doubt, specify [TRACKED], which is always "correct" but might lead to
                            ^^^^^^^
    = note: no item named `TRACKED` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `TRACKED`
   --> compiler/rustc_session/src/options.rs:61:12
    |
    |
61  |           $( #[$top_level_attr] )*
...
...
97  | / top_level_options!(
99  | |     ///
99  | |     ///
100 | |     /// For each option, one has to specify how it behaves with regard to the
201 | |     }
202 | | );
    | |__- in this macro invocation
    |
    |
    = note: the link appears in this line:
            
            [TRACKED]
             ^^^^^^^
    = note: no item named `TRACKED` in scope
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: aborting due to 7 previous errors

error: could not document `rustc_session`
error: could not document `rustc_session`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name rustc_session compiler/rustc_session/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-7ec83cf720989e83.rmeta --extern getopts=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libgetopts-98b283cf3457ec89.rmeta --extern num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libnum_cpus-7a882609a9578f58.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-9ed4c4925d19683d.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-73ab04c4425fcfe5.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-db7bf136dae7b81f.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-2ff3400d23655c5c.rmeta --extern rustc_fs_util=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_fs_util-7e9ce5babf7ddb95.rmeta --extern rustc_lint_defs=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lint_defs-ae445b9118facaa9.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-1c0074d6af137153.so --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-3c5b56748ea278fc.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-8671b0878829b434.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-2297d59093599f27.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-d866109bfaeb9bb9.rmeta -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.53.0-nightly
  (ad1214d00
  2021-04-29)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs` (exit code: 1)
[RUSTC-TIMING] rustc_session test:false 1.288
[RUSTC-TIMING] rustc_hir_pretty test:false 0.386
[RUSTC-TIMING] chalk_solve test:false 2.771
error: build failed
error: build failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--no-deps" "-p" "rustc_fs_util" "-p" "rustc_error_codes" "-p" "rustc_middle" "-p" "rustc_data_structures" "-p" "rustc_lexer" "-p" "rustc_plugin_impl" "-p" "rustc_llvm" "-p" "rustc_interface" "-p" "rustc_privacy" "-p" "rustc_hir" "-p" "rustc_expand" "-p" "rustc_attr" "-p" "rustc_builtin_macros" "-p" "rustc_ast" "-p" "rustc_serialize" "-p" "rustc_ast_pretty" "-p" "rustc_parse" "-p" "rustc_codegen_ssa" "-p" "rustc_driver" "-p" "rustc_mir_build" "-p" "rustc_span" "-p" "rustc_save_analysis" "-p" "rustc_mir" "-p" "rustc_symbol_mangling" "-p" "rustc_trait_selection" "-p" "rustc_apfloat" "-p" "rustc_ast_passes" "-p" "rustc_infer" "-p" "rustc_ast_lowering" "-p" "rustc_passes" "-p" "rustc_hir_pretty" "-p" "rustc_query_impl" "-p" "rustc_codegen_llvm" "-p" "rustc_lint_defs" "-p" "rustc_query_system" "-p" "rustc_graphviz" "-p" "rustc_type_ir" "-p" "rustc_metadata" "-p" "rustc_target" "-p" "rustc_arena" "-p" "rustc_parse_format" "-p" "rustc_session" "-p" "rustc_incremental" "-p" "rustc_lint" "-p" "rustc_typeck" "-p" "rustc_resolve" "-p" "coverage_test_macros" "-p" "rustc_macros" "-p" "rustc_index" "-p" "rustc_errors" "-p" "rustc_traits" "-p" "rustc_ty_utils" "-p" "rustc_feature"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata
Build completed unsuccessfully in 0:06:31
