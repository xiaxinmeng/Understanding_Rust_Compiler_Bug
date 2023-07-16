plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
error[E0061]: this function takes 1 argument but 0 arguments were supplied
   --> compiler/rustc_interface/src/tests.rs:133:16
    |
133 |     assert!(v1.dep_tracking_hash() != v2.dep_tracking_hash());
    |                |
    |                expected 1 argument
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/compiler/rustc_session/src/options.rs:61:20
    |
49  | / macro_rules! top_level_options {
50  | |     (pub struct Options { $(
51  | |         $opt:ident : $t:ty [$dep_tracking_marker:ident $($warn_val:expr, $warn_text:expr)*],
52  | |     )* } ) => (
...   |
61  | |             pub fn dep_tracking_hash(&self, include_no_crate_hash_opts: bool) -> u64 {
...   |
79  | |     );
80  | | }
80  | | }
    | |_- in this expansion of `top_level_options!`
...
99  | / top_level_options!(
100 | |     pub struct Options {
101 | |         // The crate config requested for the session, which may be combined
102 | |         // with additional crate configurations during the compile process.
171 | |     }
172 | | );
    | |__- in this macro invocation


error[E0061]: this function takes 1 argument but 0 arguments were supplied
   --> compiler/rustc_interface/src/tests.rs:133:42
    |
133 |     assert!(v1.dep_tracking_hash() != v2.dep_tracking_hash());
    |                                          |
    |                                          expected 1 argument
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/compiler/rustc_session/src/options.rs:61:20
    |
49  | / macro_rules! top_level_options {
50  | |     (pub struct Options { $(
51  | |         $opt:ident : $t:ty [$dep_tracking_marker:ident $($warn_val:expr, $warn_text:expr)*],
52  | |     )* } ) => (
...   |
61  | |             pub fn dep_tracking_hash(&self, include_no_crate_hash_opts: bool) -> u64 {
...   |
79  | |     );
80  | | }
80  | | }
    | |_- in this expansion of `top_level_options!`
...
99  | / top_level_options!(
100 | |     pub struct Options {
101 | |         // The crate config requested for the session, which may be combined
102 | |         // with additional crate configurations during the compile process.
171 | |     }
172 | | );
    | |__- in this macro invocation


error[E0061]: this function takes 1 argument but 0 arguments were supplied
   --> compiler/rustc_interface/src/tests.rs:134:16
    |
134 |     assert!(v1.dep_tracking_hash() != v3.dep_tracking_hash());
    |                |
    |                expected 1 argument
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/compiler/rustc_session/src/options.rs:61:20
    |
49  | / macro_rules! top_level_options {
50  | |     (pub struct Options { $(
51  | |         $opt:ident : $t:ty [$dep_tracking_marker:ident $($warn_val:expr, $warn_text:expr)*],
52  | |     )* } ) => (
...   |
61  | |             pub fn dep_tracking_hash(&self, include_no_crate_hash_opts: bool) -> u64 {
...   |
79  | |     );
80  | | }
80  | | }
    | |_- in this expansion of `top_level_options!`
...
99  | / top_level_options!(
100 | |     pub struct Options {
101 | |         // The crate config requested for the session, which may be combined
102 | |         // with additional crate configurations during the compile process.
171 | |     }
172 | | );
    | |__- in this macro invocation


error[E0061]: this function takes 1 argument but 0 arguments were supplied
   --> compiler/rustc_interface/src/tests.rs:134:42
    |
134 |     assert!(v1.dep_tracking_hash() != v3.dep_tracking_hash());
    |                                          |
    |                                          expected 1 argument
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/compiler/rustc_session/src/options.rs:61:20
    |
49  | / macro_rules! top_level_options {
50  | |     (pub struct Options { $(
51  | |         $opt:ident : $t:ty [$dep_tracking_marker:ident $($warn_val:expr, $warn_text:expr)*],
52  | |     )* } ) => (
...   |
61  | |             pub fn dep_tracking_hash(&self, include_no_crate_hash_opts: bool) -> u64 {
...   |
79  | |     );
80  | | }
80  | | }
    | |_- in this expansion of `top_level_options!`
...
99  | / top_level_options!(
100 | |     pub struct Options {
101 | |         // The crate config requested for the session, which may be combined
102 | |         // with additional crate configurations during the compile process.
171 | |     }
172 | | );
    | |__- in this macro invocation


error[E0061]: this function takes 1 argument but 0 arguments were supplied
   --> compiler/rustc_interface/src/tests.rs:135:16
    |
135 |     assert!(v2.dep_tracking_hash() != v3.dep_tracking_hash());
    |                |
    |                expected 1 argument
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/compiler/rustc_session/src/options.rs:61:20
    |
49  | / macro_rules! top_level_options {
50  | |     (pub struct Options { $(
51  | |         $opt:ident : $t:ty [$dep_tracking_marker:ident $($warn_val:expr, $warn_text:expr)*],
52  | |     )* } ) => (
...   |
61  | |             pub fn dep_tracking_hash(&self, include_no_crate_hash_opts: bool) -> u64 {
...   |
79  | |     );
80  | | }
80  | | }
    | |_- in this expansion of `top_level_options!`
...
99  | / top_level_options!(
100 | |     pub struct Options {
101 | |         // The crate config requested for the session, which may be combined
102 | |         // with additional crate configurations during the compile process.
171 | |     }
172 | | );
    | |__- in this macro invocation


error[E0061]: this function takes 1 argument but 0 arguments were supplied
   --> compiler/rustc_interface/src/tests.rs:135:42
    |
135 |     assert!(v2.dep_tracking_hash() != v3.dep_tracking_hash());
    |                                          |
    |                                          expected 1 argument
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/compiler/rustc_session/src/options.rs:61:20
    |
49  | / macro_rules! top_level_options {
50  | |     (pub struct Options { $(
51  | |         $opt:ident : $t:ty [$dep_tracking_marker:ident $($warn_val:expr, $warn_text:expr)*],
52  | |     )* } ) => (
...   |
61  | |             pub fn dep_tracking_hash(&self, include_no_crate_hash_opts: bool) -> u64 {
...   |
79  | |     );
80  | | }
80  | | }
    | |_- in this expansion of `top_level_options!`
...
99  | / top_level_options!(
100 | |     pub struct Options {
101 | |         // The crate config requested for the session, which may be combined
102 | |         // with additional crate configurations during the compile process.
171 | |     }
172 | | );
    | |__- in this macro invocation


error[E0061]: this function takes 1 argument but 0 arguments were supplied
   --> compiler/rustc_interface/src/tests.rs:138:19
    |
138 |     assert_eq!(v1.dep_tracking_hash(), v1.clone().dep_tracking_hash());
    |                   |
    |                   expected 1 argument
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/compiler/rustc_session/src/options.rs:61:20
    |
49  | / macro_rules! top_level_options {
50  | |     (pub struct Options { $(
51  | |         $opt:ident : $t:ty [$dep_tracking_marker:ident $($warn_val:expr, $warn_text:expr)*],
52  | |     )* } ) => (
...   |
61  | |             pub fn dep_tracking_hash(&self, include_no_crate_hash_opts: bool) -> u64 {
...   |
79  | |     );
80  | | }
80  | | }
    | |_- in this expansion of `top_level_options!`
...
99  | / top_level_options!(
100 | |     pub struct Options {
101 | |         // The crate config requested for the session, which may be combined
102 | |         // with additional crate configurations during the compile process.
171 | |     }
172 | | );
    | |__- in this macro invocation


error[E0061]: this function takes 1 argument but 0 arguments were supplied
   --> compiler/rustc_interface/src/tests.rs:138:51
    |
138 |     assert_eq!(v1.dep_tracking_hash(), v1.clone().dep_tracking_hash());
    |                                                   |
    |                                                   expected 1 argument
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/compiler/rustc_session/src/options.rs:61:20
    |
49  | / macro_rules! top_level_options {
50  | |     (pub struct Options { $(
51  | |         $opt:ident : $t:ty [$dep_tracking_marker:ident $($warn_val:expr, $warn_text:expr)*],
52  | |     )* } ) => (
...   |
61  | |             pub fn dep_tracking_hash(&self, include_no_crate_hash_opts: bool) -> u64 {
...   |
79  | |     );
80  | | }
80  | | }
    | |_- in this expansion of `top_level_options!`
...
99  | / top_level_options!(
100 | |     pub struct Options {
101 | |         // The crate config requested for the session, which may be combined
102 | |         // with additional crate configurations during the compile process.
171 | |     }
172 | | );
    | |__- in this macro invocation


error[E0061]: this function takes 1 argument but 0 arguments were supplied
   --> compiler/rustc_interface/src/tests.rs:139:19
    |
139 |     assert_eq!(v2.dep_tracking_hash(), v2.clone().dep_tracking_hash());
    |                   |
    |                   expected 1 argument
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/compiler/rustc_session/src/options.rs:61:20
    |
49  | / macro_rules! top_level_options {
50  | |     (pub struct Options { $(
51  | |         $opt:ident : $t:ty [$dep_tracking_marker:ident $($warn_val:expr, $warn_text:expr)*],
52  | |     )* } ) => (
...   |
61  | |             pub fn dep_tracking_hash(&self, include_no_crate_hash_opts: bool) -> u64 {
...   |
79  | |     );
80  | | }
80  | | }
    | |_- in this expansion of `top_level_options!`
...
99  | / top_level_options!(
100 | |     pub struct Options {
101 | |         // The crate config requested for the session, which may be combined
102 | |         // with additional crate configurations during the compile process.
171 | |     }
172 | | );
    | |__- in this macro invocation


error[E0061]: this function takes 1 argument but 0 arguments were supplied
   --> compiler/rustc_interface/src/tests.rs:139:51
    |
139 |     assert_eq!(v2.dep_tracking_hash(), v2.clone().dep_tracking_hash());
    |                                                   |
    |                                                   expected 1 argument
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/compiler/rustc_session/src/options.rs:61:20
    |
49  | / macro_rules! top_level_options {
50  | |     (pub struct Options { $(
51  | |         $opt:ident : $t:ty [$dep_tracking_marker:ident $($warn_val:expr, $warn_text:expr)*],
52  | |     )* } ) => (
...   |
61  | |             pub fn dep_tracking_hash(&self, include_no_crate_hash_opts: bool) -> u64 {
...   |
79  | |     );
80  | | }
80  | | }
    | |_- in this expansion of `top_level_options!`
...
99  | / top_level_options!(
100 | |     pub struct Options {
101 | |         // The crate config requested for the session, which may be combined
102 | |         // with additional crate configurations during the compile process.
171 | |     }
172 | | );
    | |__- in this macro invocation


error[E0061]: this function takes 1 argument but 0 arguments were supplied
   --> compiler/rustc_interface/src/tests.rs:140:19
    |
140 |     assert_eq!(v3.dep_tracking_hash(), v3.clone().dep_tracking_hash());
    |                   |
    |                   expected 1 argument
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/compiler/rustc_session/src/options.rs:61:20
    |
49  | / macro_rules! top_level_options {
50  | |     (pub struct Options { $(
51  | |         $opt:ident : $t:ty [$dep_tracking_marker:ident $($warn_val:expr, $warn_text:expr)*],
52  | |     )* } ) => (
...   |
61  | |             pub fn dep_tracking_hash(&self, include_no_crate_hash_opts: bool) -> u64 {
...   |
79  | |     );
80  | | }
80  | | }
    | |_- in this expansion of `top_level_options!`
...
99  | / top_level_options!(
100 | |     pub struct Options {
101 | |         // The crate config requested for the session, which may be combined
102 | |         // with additional crate configurations during the compile process.
171 | |     }
172 | | );
    | |__- in this macro invocation


error[E0061]: this function takes 1 argument but 0 arguments were supplied
   --> compiler/rustc_interface/src/tests.rs:140:51
    |
140 |     assert_eq!(v3.dep_tracking_hash(), v3.clone().dep_tracking_hash());
    |                                                   |
    |                                                   expected 1 argument
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/compiler/rustc_session/src/options.rs:61:20
    |
49  | / macro_rules! top_level_options {
50  | |     (pub struct Options { $(
51  | |         $opt:ident : $t:ty [$dep_tracking_marker:ident $($warn_val:expr, $warn_text:expr)*],
52  | |     )* } ) => (
...   |
61  | |             pub fn dep_tracking_hash(&self, include_no_crate_hash_opts: bool) -> u64 {
...   |
79  | |     );
80  | | }
80  | | }
    | |_- in this expansion of `top_level_options!`
...
99  | / top_level_options!(
100 | |     pub struct Options {
101 | |         // The crate config requested for the session, which may be combined
102 | |         // with additional crate configurations during the compile process.
171 | |     }
172 | | );
    | |__- in this macro invocation


error[E0061]: this function takes 1 argument but 0 arguments were supplied
   --> compiler/rustc_interface/src/tests.rs:158:19
    |
158 |     assert_eq!(v1.dep_tracking_hash(), v2.dep_tracking_hash());
    |                   |
    |                   expected 1 argument
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/compiler/rustc_session/src/options.rs:61:20
    |
49  | / macro_rules! top_level_options {
50  | |     (pub struct Options { $(
51  | |         $opt:ident : $t:ty [$dep_tracking_marker:ident $($warn_val:expr, $warn_text:expr)*],
52  | |     )* } ) => (
...   |
61  | |             pub fn dep_tracking_hash(&self, include_no_crate_hash_opts: bool) -> u64 {
...   |
79  | |     );
80  | | }
80  | | }
    | |_- in this expansion of `top_level_options!`
...
99  | / top_level_options!(
100 | |     pub struct Options {
101 | |         // The crate config requested for the session, which may be combined
102 | |         // with additional crate configurations during the compile process.
171 | |     }
172 | | );
    | |__- in this macro invocation


error[E0061]: this function takes 1 argument but 0 arguments were supplied
   --> compiler/rustc_interface/src/tests.rs:158:43
    |
158 |     assert_eq!(v1.dep_tracking_hash(), v2.dep_tracking_hash());
    |                                           |
    |                                           expected 1 argument
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/compiler/rustc_session/src/options.rs:61:20
    |
49  | / macro_rules! top_level_options {
50  | |     (pub struct Options { $(
51  | |         $opt:ident : $t:ty [$dep_tracking_marker:ident $($warn_val:expr, $warn_text:expr)*],
52  | |     )* } ) => (
...   |
61  | |             pub fn dep_tracking_hash(&self, include_no_crate_hash_opts: bool) -> u64 {
...   |
79  | |     );
80  | | }
80  | | }
    | |_- in this expansion of `top_level_options!`
...
99  | / top_level_options!(
100 | |     pub struct Options {
101 | |         // The crate config requested for the session, which may be combined
102 | |         // with additional crate configurations during the compile process.
171 | |     }
172 | | );
    | |__- in this macro invocation


error[E0061]: this function takes 1 argument but 0 arguments were supplied
   --> compiler/rustc_interface/src/tests.rs:161:19
    |
161 |     assert_eq!(v1.dep_tracking_hash(), v1.clone().dep_tracking_hash());
    |                   |
    |                   expected 1 argument
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/compiler/rustc_session/src/options.rs:61:20
    |
49  | / macro_rules! top_level_options {
50  | |     (pub struct Options { $(
51  | |         $opt:ident : $t:ty [$dep_tracking_marker:ident $($warn_val:expr, $warn_text:expr)*],
52  | |     )* } ) => (
...   |
61  | |             pub fn dep_tracking_hash(&self, include_no_crate_hash_opts: bool) -> u64 {
...   |
79  | |     );
80  | | }
80  | | }
    | |_- in this expansion of `top_level_options!`
...
99  | / top_level_options!(
100 | |     pub struct Options {
101 | |         // The crate config requested for the session, which may be combined
102 | |         // with additional crate configurations during the compile process.
171 | |     }
172 | | );
    | |__- in this macro invocation


error[E0061]: this function takes 1 argument but 0 arguments were supplied
   --> compiler/rustc_interface/src/tests.rs:161:51
    |
161 |     assert_eq!(v1.dep_tracking_hash(), v1.clone().dep_tracking_hash());
    |                                                   |
    |                                                   expected 1 argument
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/compiler/rustc_session/src/options.rs:61:20
    |
49  | / macro_rules! top_level_options {
50  | |     (pub struct Options { $(
51  | |         $opt:ident : $t:ty [$dep_tracking_marker:ident $($warn_val:expr, $warn_text:expr)*],
52  | |     )* } ) => (
...   |
61  | |             pub fn dep_tracking_hash(&self, include_no_crate_hash_opts: bool) -> u64 {
...   |
79  | |     );
---

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_index" "-p" "rustc_macros" "-p" "rustc_save_analysis" "-p" "rustc_lexer" "-p" "rustc_plugin_impl" "-p" "rustc_mir_build" "-p" "rustc_infer" "-p" "rustc_attr" "-p" "rustc_apfloat" "-p" "rustc_arena" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_ast" "-p" "rustc_lint" "-p" "rustc_feature" "-p" "rustc_error_codes" "-p" "rustc_parse" "-p" "rustc_typeck" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_interface" "-p" "rustc_query_impl" "-p" "rustc_query_system" "-p" "rustc_expand" "-p" "rustc_ty_utils" "-p" "rustc_privacy" "-p" "rustc_ast_passes" "-p" "rustc_traits" "-p" "rustc_resolve" "-p" "rustc_symbol_mangling" "-p" "rustc_codegen_llvm" "-p" "rustc_fs_util" "-p" "rustc_llvm" "-p" "rustc_incremental" "-p" "rustc_passes" "-p" "rustc_builtin_macros" "-p" "rustc_ast_lowering" "-p" "rustc_hir_pretty" "-p" "rustc_metadata" "-p" "rustc_target" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_span" "-p" "rustc_session" "-p" "rustc_serialize" "-p" "rustc_hir" "-p" "rustc_ast_pretty" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:01:47
