plain
travis_time:end:00ee4fc3:start=1541974644860955567,finish=1541974645908421961,duration=1047466394
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:05:59]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:06:06]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:20]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:07:30]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:56] error[E0599]: no function or associated item named `from_u32_const` found for type `ty::UniverseIndex` in the current scope
[00:07:56]      |
[00:07:56]      |
[00:07:56] 1492 | / newtype_index! {
[00:07:56] 1493 | |     pub struct UniverseIndex {
[00:07:56] 1494 | |         DEBUG_FORMAT = "U{}",
[00:07:56] 1496 | | }
[00:07:56] 1496 | | }
[00:07:56]      | |_- function or associated item `from_u32_const` not found for this
[00:07:56] ...
[00:07:56] 1501 |       pub const ROOT: UniverseIndex = UniverseIndex::from_u32_const(0);
[00:07:56]      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `ty::UniverseIndex`
[00:07:56]      = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:07:56] 
[00:08:00] error[E0308]: mismatched types
[00:08:00] error[E0308]: mismatched types
[00:08:00]   --> librustc/dep_graph/serialized.rs:17:1
[00:08:00]    |
[00:08:00] 17 | / newtype_index! {
[00:08:00] 18 | |     pub struct SerializedDepNodeIndex { .. }
[00:08:00] 19 | | }
[00:08:00]    | |_^ expected u32, found struct `std::num::NonZeroU32`
[00:08:00]    = note: expected type `u32`
[00:08:00]               found type `std::num::NonZeroU32`
[00:08:00]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:08:00] 
[00:08:00] 
[00:08:00] error[E0308]: mismatched types
[00:08:00]   --> librustc/hir/def_id.rs:18:1
[00:08:00]    |
[00:08:00] 18 | / newtype_index! {
[00:08:00] 19 | |     pub struct CrateId {
[00:08:00] 20 | |         ENCODABLE = custom
[00:08:00] 22 | | }
[00:08:00] 22 | | }
[00:08:00]    | |_^ expected u32, found struct `std::num::NonZeroU32`
[00:08:00]    = note: expected type `u32`
[00:08:00]               found type `std::num::NonZeroU32`
[00:08:00]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:08:00] 
[00:08:00] 
[00:08:01] error[E0308]: mismatched types
[00:08:01]    --> librustc/middle/region.rs:163:1
[00:08:01]     |
[00:08:01] 163 | / newtype_index! {
[00:08:01] 164 | |     pub struct FirstStatementIndex { .. }
[00:08:01] 165 | | }
[00:08:01]     | |_^ expected u32, found struct `std::num::NonZeroU32`
[00:08:01]     = note: expected type `u32`
[00:08:01]                found type `std::num::NonZeroU32`
[00:08:01]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:08:01] 
[00:08:01] 
[00:08:01] error[E0308]: mismatched types
[00:08:01]    --> librustc/mir/mod.rs:576:1
[00:08:01]     |
[00:08:01] 576 | / newtype_index! {
[00:08:01] 577 | |     pub struct Local {
[00:08:01] 578 | |         DEBUG_FORMAT = "_{}",
[00:08:01] 579 | |         const RETURN_PLACE = 0,
[00:08:01] 581 | | }
[00:08:01] 581 | | }
[00:08:01]     | |_^ expected u32, found struct `std::num::NonZeroU32`
[00:08:01]     = note: expected type `u32`
[00:08:01]                found type `std::num::NonZeroU32`
[00:08:01]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:08:01] 
[00:08:01] 
[00:08:01] error[E0308]: mismatched types
[00:08:01]    --> librustc/mir/mod.rs:986:1
[00:08:01]     |
[00:08:01] 986 | / newtype_index! {
[00:08:01] 987 | |     pub struct BasicBlock {
[00:08:01] 988 | |         DEBUG_FORMAT = "bb{}",
[00:08:01] 989 | |         const START_BLOCK = 0,
[00:08:01] 991 | | }
[00:08:01] 991 | | }
[00:08:01]     | |_^ expected u32, found struct `std::num::NonZeroU32`
[00:08:01]     = note: expected type `u32`
[00:08:01]                found type `std::num::NonZeroU32`
[00:08:01]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:08:01] 
[00:08:01] 
[00:08:01] error[E0308]: mismatched types
[00:08:01]     --> librustc/mir/mod.rs:1957:1
[00:08:01]      |
[00:08:01] 1957 | / newtype_index! {
[00:08:01] 1958 | |     pub struct Field {
[00:08:01] 1959 | |         DEBUG_FORMAT = "field[{}]"
[00:08:01] 1961 | | }
[00:08:01] 1961 | | }
[00:08:01]      | |_^ expected u32, found struct `std::num::NonZeroU32`
[00:08:01]      = note: expected type `u32`
[00:08:01]                 found type `std::num::NonZeroU32`
[00:08:01]      = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:08:01] 
[00:08:01] 
[00:08:01] error[E0308]: mismatched types
[00:08:01]     --> librustc/mir/mod.rs:2058:1
[00:08:01]      |
[00:08:01] | / newtype_index! {
[00:08:01] 2574 | |     pub struct Promoted {
[00:08:01] 2575 | |         DEBUG_FORMAT = "promoted[{}]"
[00:08:01] 2577 | | }
[00:08:01] 2577 | | }
[00:08:01]      | |_^ expected u32, found struct `std::num::NonZeroU32`
[00:08:01]      = note: expected type `u32`
[00:08:01]                 found type `std::num::NonZeroU32`
[00:08:01]      = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:08:01] 
[00:08:01] 
[00:08:01] error[E0308]: mismatched types
[00:08:01]     --> librustc/ty/sty.rs:1070:1
[00:08:01]      |
[00:08:01] 1070 | / newtype_index! {
[00:08:01] 1071 | |     pub struct DebruijnIndex {
[00:08:01] 1072 | |         DEBUG_FORMAT = "DebruijnIndex({})",
[00:08:01] 1073 | |         const INNERMOST = 0,
[00:08:01] 1075 | | }
[00:08:01] 1075 | | }
[00:08:01]      | |_^ expected u32, found struct `std::num::NonZeroU32`
[00:08:01]      = note: expected type `u32`
[00:08:01]                 found type `std::num::NonZeroU32`
[00:08:01]      = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:08:01] 
[00:08:01] 
[00:08:01] [38;5;
[00:08:02] 1235 | / newtype_index! {
[00:08:02] 1236 | |     pub struct BoundVar { .. }
[00:08:02] 1237 | | }
[00:08:02]      | |_^ expected u32, found struct `std::num::NonZeroU32`
[00:08:02]      = note: expected type `u32`
[00:08:02]                 found type `std::num::NonZeroU32`
[00:08:02]      = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:08:02] 
[00:08:02] 
[00:08:02] error[E0308]: mismatched types
[00:08:02]     --> librustc/ty/mod.rs:1492:1
[00:08:02]      |
[00:08:02] 1492 | / newtype_index! {
[00:08:02] 1493 | |     pub struct UniverseIndex {
[00:08:02] 1494 | |         DEBUG_FORMAT = "U{}",
[00:08:02] 1496 | | }
[00:08:02] 1496 | | }
[00:08:02]      | |_^ expected u32, found struct `std::num::NonZeroU32`
[00:08:02]      = note: expected type `u32`
[00:08:02]                 found type `std::num::NonZeroU32`
[00:08:02]      = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:08:02] 
[00:08:02] 
[00:08:02] error[E0599]: no method named `get` found for type `u32` in the current scope
[00:08:02]   --> librustc/dep_graph/graph.rs:42:1
[00:08:02]    |
[00:08:02] 42 | / newtype_index! {
[00:08:02] 43 | |     pub struct DepNodeIndex { .. }
[00:08:02]    | |_^
[00:08:02]    |
[00:08:02]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:08:02] 
[00:08:02] 
[00:08:02] error[E0599]: no method named `get` found for type `u32` in the current scope
[00:08:02]   --> librustc/dep_graph/serialized.rs:17:1
[00:08:02]    |
[00:08:02] 17 | / newtype_index! {
[00:08:02] 18 | |     pub struct SerializedDepNodeIndex { .. }
[00:08:02]    | |_^
[00:08:02]    |
[00:08:02]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:08:02] 
[00:08:02] 
[00:08:02] error[E0599]: no method named `get` found for type `u32` in the current scope
[00:08:02]   --> librustc/hir/def_id.rs:18:1
[00:08:02]    |
[00:08:02] 18 | / newtype_index! {
[00:08:02] 19 | |     pub struct CrateId {race for more info)
[00:08:02] error[E0599]: no method named `get` found for type `u32` in the current scope
[00:08:02]    --> librustc/mir/mod.rs:576:1
[00:08:02]     |
[00:08:02]     |
[00:08:02] 576 | / newtype_index! {
[00:08:02] 577 | |     pub struct Local {
[00:08:02] 578 | |         DEBUG_FORMAT = "_{}",
[00:08:02] 579 | |         const RETURN_PLACE = 0,
[00:08:02] 581 | | }
[00:08:02]     | |_^
[00:08:02]     |
[00:08:02]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:08:02]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:08:02] 
[00:08:02] error[E0599]: no method named `get` found for type `u32` in the current scope
[00:08:02]    --> librustc/mir/mod.rs:986:1
[00:08:02]     |
;12m1959 | |         DEBUG_FORMAT = "field[{}]"
[00:08:02] 1961 | | }
[00:08:02]      | |_^
[00:08:02]      |
[00:08:02]      = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:08:02]      = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:08:02] 
[00:08:02] error[E0599]: no method named `get` found for type `u32` in the current scope
[00:08:02]     --> librustc/mir/mod.rs:2058:1
[00:08:02]      |
[00:08:02] 2058 | / newtype_index! {
[00:08:02] 2059 | |     pub struct SourceScope {
[00:08:02] 2060 | |         DEBUG_FORMAT = "scope[{}]",
[00:08:02] 2061 | |         const OUTERMOST_SOURCE_SCOPE = 0,
[00:08:02] o)
[00:08:02] 
[00:08:02] error[E0599]: no method named `get` found for type `u32` in the current scope
[00:08:02]     --> librustc/ty/sty.rs:1070:1
[00:08:02]     --> librustc/ty/sty.rs:1070:1
[00:08:02]      |
[00:08:02] 1070 | / newtype_index! {
[00:08:02] 1071 | |     pub struct DebruijnIndex {
[00:08:02] 1072 | |         DEBUG_FORMAT = "DebruijnIndex({})",
[00:08:02] 1073 | |         const INNERMOST = 0,
[00:08:02] 1075 | | }
[00:08:02]      | |_^
[00:08:02]      |
[00:08:02]      = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:08:02]      = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:08:02] 
[00:08:02] error[E0599]: no method named `get` found for type `u32` in the current scope
[00:08:02]     --> librustc/ty/sty.rs:1209:1
[00:08:02]      |
[00:08:02] 1209 | / newtype_index! {
[00:08:02] 1210 | |     pub struct RegionVid {
[00:08:02] 1211 | |         DEBUG_FORMAT = custom,
[00:08:02] 1213 | | }
[00:08:02]      | |_^
[00:08:02]      |
[00:08:02]      = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:08:02]      = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:08:02] 
[00:08:02] error[E0599]: no method named `get` found for type `u32` in the current scope
[00:08:02]     --> librustc/ty/sty.rs:1235:1
[00:08:02]      |
[00:08:02] 1235 | / newtype_index! {
[00:08:02] 1236 | |     pub struct BoundVar { .. }
[00:08:02]      | |_^
[00:08:02]      |
[00:08:02]      = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:08:02] 
[00:08:02] 
[00:08:02] error[E0599]: no method named `get` found for type `u32` in the current scope
[00:08:02]     --> librustc/ty/mod.rs:1492:1
[00:08:02]      |
[00:08:02] 1492 | / newtype_index! {
[00:08:02] 1493 | |     pub struct UniverseIndex {
[00:08:02] 1494 | |         DEBUG_FORMAT = "U{}",
[00:08:02] 1496 | | }
[00:08:02]      | |_^
[00:08:02]      |
[00:08:02]      = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
---
[00:08:02] 
[00:08:02] To learn more, run the command again with --verbose.
[00:08:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:08:02] expected success, got: exit code: 101
[00:08:02] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1101:9
[00:08:02] travis_fold:end:stage0-rustc

[00:08:02] travis_time:end:stage0-rustc:start=1541974971256797120,finish=1541975138597368630,duration=167340571510


[00:08:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:08:02] Build completed unsuccessfully in 0:03:54
[00:08:02] Makefile:28: recipe for target 'all' failed
[00:08:02] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c080c3f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Nov 11 22:25:38 UTC 2018
