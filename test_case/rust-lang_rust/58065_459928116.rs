plain
travis_time:end:1a3a1ee2:start=1549074059680031926,finish=1549074060670582472,duration=990550546
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:20:03] 
[00:20:03] error[E0425]: cannot find value `path_str` in this scope
[00:20:03]     --> src/librustc_resolve/lib.rs:3140:36
[00:20:03]      |
[00:20:03] 3140 |                     format!("{}!", path_str),
[00:20:03] 
[00:20:03] error[E0425]: cannot find value `span` in this scope
[00:20:03]     --> src/librustc_resolve/lib.rs:3146:32
[00:20:03]      |
[00:20:03]      |
[00:20:03] 3146 |                 err.span_label(span, "type aliases cannot be used as traits");
[00:20:03] 
[00:20:03] error[E0425]: cannot find value `path_str` in this scope
[00:20:03]     --> src/librustc_resolve/lib.rs:3157:43
[00:20:03]      |
[00:20:03]      |
[00:20:03] 3157 |                         format!("{}::{}", path_str, ident),
[00:20:03] 
[00:20:03] error[E0425]: cannot find value `path_str` in this scope
[00:20:03]     --> src/librustc_resolve/lib.rs:3167:43
[00:20:03]      |
[00:20:03]      |
[00:20:03] 3167 |                         format!("{}::{}", path_str, segment.ident),
[00:20:03] 
[00:20:03] error[E0424]: expected value, found module `self`
[00:20:03]     --> src/librustc_resolve/lib.rs:3176:41
[00:20:03]      |
[00:20:03]      |
[00:20:03] 3176 |                 if let Some(variants) = self.collect_enum_variants(def) {
[00:20:03]      |                                         ^^^^ `self` value is a keyword only available in methods with `self` parameter
[00:20:03] 
[00:20:03] error[E0425]: cannot find value `ns` in this scope
[00:20:03]      |
[00:20:03]      |
[00:20:03] 3189 |             (Def::Struct(def_id), _) if ns == ValueNS => {
[00:20:03]      |                                         ^^ not found in this scope
[00:20:03] error[E0424]: expected value, found module `self`
[00:20:03]     --> src/librustc_resolve/lib.rs:3191:27
[00:20:03]      |
[00:20:03]      |
[00:20:03] 3191 |                         = self.struct_constructors.get(&def_id).cloned() {
[00:20:03]      |                           ^^^^ `self` value is a keyword only available in methods with `self` parameter
[00:20:03] error[E0424]: expected value, found module `self`
[00:20:03]     --> src/librustc_resolve/lib.rs:3192:43
[00:20:03]      |
[00:20:03]      |
[00:20:03] 3192 |                     let accessible_ctor = self.is_accessible(ctor_vis);
[00:20:03]      |                                           ^^^^ `self` value is a keyword only available in methods with `self` parameter
[00:20:03] error[E0425]: cannot find function `is_expected` in this scope
[00:20:03]     --> src/librustc_resolve/lib.rs:3193:24
[00:20:03]      |
[00:20:03]      |
[00:20:03] 3193 |                     if is_expected(ctor_def) && !accessible_ctor {
[00:20:03] 
[00:20:03] error[E0425]: cannot find value `span` in this scope
[00:20:03]     --> src/librustc_resolve/lib.rs:3194:40
[00:20:03]      |
[00:20:03]      |
[00:20:03] 3194 |                         err.span_label(span, format!("constructor is not visible \
[00:20:03] 
[00:20:03] error[E0424]: expected value, found module `self`
[00:20:03]     --> src/librustc_resolve/lib.rs:3202:30
[00:20:03]      |
[00:20:03]      |
[00:20:03] 3202 |                     let sm = self.session.source_map();
[00:20:03]      |                              ^^^^ `self` value is a keyword only available in methods with `self` parameter
[00:20:03] error[E0425]: cannot find value `span` in this scope
[00:20:03]     --> src/librustc_resolve/lib.rs:3203:34
[00:20:03]      |
[00:20:03] 3203 |                     let mut sp = span;
[00:20:03] 3203 |                     let mut sp = span;
[00:20:03]      |                                  ^^^^ not found in this scope
[00:20:03] 
[00:20:03] error[E0425]: cannot find value `span` in this scope
[00:20:03]     --> src/librustc_resolve/lib.rs:3228:46
[00:20:03]      |
[00:20:03] 3228 |                                     let sp = span.to(sp);
[00:20:03] 
[00:20:03] error[E0425]: cannot find value `path_str` in this scope
[00:20:03]     --> src/librustc_resolve/lib.rs:3253:49
[00:20:03]      |
---
[00:20:03]      |
[00:20:03] 3290 |                                         path_str),
[00:20:03]      |                                         ^^^^^^^^ not found in this scope
[00:20:03] 
[00:20:03] error[E0425]: cannot find value `ns` in this scope
[00:20:03]      |
[00:20:03]      |
[00:20:03] 3300 |             (Def::VariantCtor(_, CtorKind::Fictive), _) if ns == ValueNS => {
[00:20:03]      |                                                            ^^ not found in this scope
[00:20:03] error[E0425]: cannot find value `span` in this scope
[00:20:03]     --> src/librustc_resolve/lib.rs:3301:32
[00:20:03]      |
[00:20:03]      |
[00:20:03] 3301 |                 err.span_label(span, format!("did you mean `{} {{ /* fields */ }}`?",
[00:20:03] 
[00:20:03] error[E0425]: cannot find value `path_str` in this scope
[00:20:03]     --> src/librustc_resolve/lib.rs:3302:46
[00:20:03]      |
[00:20:03]      |
[00:20:03] 3302 |                                              path_str));
[00:20:03]      |                                              ^^^^^^^^ not found in this scope
[00:20:03] 
[00:20:03] error[E0425]: cannot find value `ns` in this scope
[00:20:03]      |
[00:20:03]      |
[00:20:03] 3305 |             (Def::SelfTy(..), _) if ns == ValueNS => {
[00:20:03]      |                                     ^^ not found in this scope
[00:20:03] error[E0425]: cannot find value `span` in this scope
[00:20:03]     --> src/librustc_resolve/lib.rs:3306:32
[00:20:03]      |
[00:20:03]      |
[00:20:03] 3306 |                 err.span_label(span, fallback_label);
[00:20:03] 
[00:20:03] 
[00:20:03] error[E0425]: cannot find value `fallback_label` in this scope
[00:20:03]      |
[00:20:03]      |
[00:20:03] 3306 |                 err.span_label(span, fallback_label);
[00:20:03] 
[00:20:03] 
[00:20:03] error[E0425]: cannot find value `ns` in this scope
[00:20:03]      |
[00:20:03]      |
[00:20:03] 3311 |             (Def::TyAlias(_), _) | (Def::AssociatedTy(..), _) if ns == ValueNS => {
[00:20:03]      |                                                                  ^^ not found in this scope
[00:20:03] 
[00:20:03] error[E0425]: cannot find function `smart_resolve_context_dep_help` in this scope
[00:20:03]      |
[00:20:03]      |
[00:20:03] 3512 |             if smart_resolve_context_dep_help(&mut err, &mut candidates, source, def) {
[00:20:03] 
[00:20:04] error[E0308]: mismatched types
[00:20:04]     --> src/librustc_resolve/lib.rs:3136:14
[00:20:04]      |
[00:20:04]      |
[00:20:04] 3135 |         match (def, source) {
[00:20:04]      |               ------------- this match expression has type `std::option::Option<rustc::hir::def::Def>`
[00:20:04] 3136 |             (Def::Macro(..), _) => {
[00:20:04]      |
[00:20:04]      = note: expected type `std::option::Option<rustc::hir::def::Def>`
[00:20:04]                 found type `rustc::hir::def::Def`
[00:20:04] 
[00:20:04] 
[00:20:04] error[E0308]: mismatched types
[00:20:04]     --> src/librustc_resolve/lib.rs:3145:14
[00:20:04]      |
[00:20:04] 3135 |         match (def, source) {
[00:20:04]      |               ------------- this match expression has type `std::option::Option<rustc::hir::def::Def>`
[00:20:04] ...
[00:20:04] 3145 |             (Def::TyAlias(..), PathSource::Trait(_)) => {
[00:20:04]      |
[00:20:04]      = note: expected type `std::option::Option<rustc::hir::def::Def>`
[00:20:04]                 found type `rustc::hir::def::Def`
[00:20:04] 
[00:20:04] 
[00:20:04] error[E0308]: mismatched types
[00:20:04]     --> src/librustc_resolve/lib.rs:3152:14
[00:20:04]      |
[00:20:04] 3135 |         match (def, source) {
[00:20:04]      |               ------------- this match expression has type `std::option::Option<rustc::hir::def::Def>`
[00:20:04] ...
[00:20:04] 3152 |             (Def::Mod(..), PathSource::Expr(Some(parent))) => match parent.node {
[00:20:04]      |
[00:20:04]      = note: expected type `std::option::Option<rustc::hir::def::Def>`
[00:20:04]                 found type `rustc::hir::def::Def`
[00:20:04] 
[00:20:04] 
[00:20:04] error[E0308]: mismatched types
[00:20:04]     --> src/librustc_resolve/lib.rs:3174:14
[00:20:04]      |
[00:20:04] 3135 |         match (def, source) {
[00:20:04]      |               ------------- this match expression has type `std::option::Option<rustc::hir::def::Def>`
[00:20:04] ...
[00:20:04] 3174 |             (Def::Enum(..), PathSource::TupleStruct)
[00:20:04]      |
[00:20:04]      = note: expected type `std::option::Option<rustc::hir::def::Def>`
[00:20:04]                 found type `rustc::hir::def::Def`
[00:20:04] 
[00:20:04] 
[00:20:04] error[E0308]: mismatched types
[00:20:04]     --> src/librustc_resolve/lib.rs:3175:20
[00:20:04]      |
[00:20:04] 3135 |         match (def, source) {
[00:20:04]      |               ------------- this match expression has type `std::option::Option<rustc::hir::def::Def>`
[00:20:04] ...
[00:20:04] 3175 |                 | (Def::Enum(..), PathSource::Expr(..))  => {
[00:20:04]      |
[00:20:04]      = note: expected type `std::option::Option<rustc::hir::def::Def>`
[00:20:04]                 found type `rustc::hir::def::Def`
[00:20:04] 
[00:20:04] 
[00:20:04] error[E0308]: mismatched types
[00:20:04]     --> src/librustc_resolve/lib.rs:3189:14
[00:20:04]      |
[00:20:04] 3135 |         match (def, source) {
[00:20:04]      |               ------------- this match expression has type `std::option::Option<rustc::hir::def::Def>`
[00:20:04] ...
[00:20:04] 3189 |             (Def::Struct(def_id), _) if ns == ValueNS => {
[00:20:04]      |
[00:20:04]      = note: expected type `std::option::Option<rustc::hir::def::Def>`
[00:20:04]                 found type `rustc::hir::def::Def`
[00:20:04] 
[00:20:04] 
[00:20:04] error[E0308]: mismatched types
[00:20:04]     --> src/librustc_resolve/lib.rs:3298:14
[00:20:04]      |
[00:20:04] 3135 |         match (def, source) {
[00:20:04]      |               ------------- this match expression has type `std::option::Option<rustc::hir::def::Def>`
[00:20:04] ...
[00:20:04] 3298 |             (Def::Union(..), _) |
[00:20:04]      |
[00:20:04]      = note: expected type `std::option::Option<rustc::hir::def::Def>`
[00:20:04]                 found type `rustc::hir::def::Def`
[00:20:04] 
[00:20:04] 
[00:20:04] error[E0308]: mismatched types
[00:20:04]     --> src/librustc_resolve/lib.rs:3299:14
[00:20:04]      |
[00:20:04] 3135 |         match (def, source) {
[00:20:04]      |               ------------- this match expression has type `std::option::Option<rustc::hir::def::Def>`
[00:20:04] ...
[00:20:04] 3299 |             (Def::Variant(..), _) |
[00:20:04]      |
[00:20:04]      = note: expected type `std::option::Option<rustc::hir::def::Def>`
[00:20:04]                 found type `rustc::hir::def::Def`
[00:20:04] 
[00:20:04] 
[00:20:04] error[E0308]: mismatched types
[00:20:04]     --> src/librustc_resolve/lib.rs:3300:14
[00:20:04]      |
[00:20:04] 3135 |         match (def, source) {
[00:20:04]      |               ------------- this match expression has type `std::option::Option<rustc::hir::def::Def>`
[00:20:04] ...
[00:20:04] 3300 |             (Def::VariantCtor(_, CtorKind::Fictive), _) if ns == ValueNS => {
[00:20:04]      |
[00:20:04]      = note: expected type `std::option::Option<rustc::hir::def::Def>`
[00:20:04]                 found type `rustc::hir::def::Def`
[00:20:04] 
[00:20:04] 
[00:20:04] error[E0308]: mismatched types
[00:20:04]     --> src/librustc_resolve/lib.rs:3305:14
[00:20:04]      |
[00:20:04] 3135 |         match (def, source) {
[00:20:04]      |               ------------- this match expression has type `std::option::Option<rustc::hir::def::Def>`
[00:20:04] ...
[00:20:04] 3305 |             (Def::SelfTy(..), _) if ns == ValueNS => {
[00:20:04]      |
[00:20:04]      = note: expected type `std::option::Option<rustc::hir::def::Def>`
[00:20:04]                 found type `rustc::hir::def::Def`
[00:20:04] 
[00:20:04] 
[00:20:04] error[E0308]: mismatched types
[00:20:04]     --> src/librustc_resolve/lib.rs:3311:14
[00:20:04]      |
[00:20:04] 3135 |         match (def, source) {
[00:20:04]      |               ------------- this match expression has type `std::option::Option<rustc::hir::def::Def>`
[00:20:04] ...
[00:20:04] 3311 |             (Def::TyAlias(_), _) | (Def::AssociatedTy(..), _) if ns == ValueNS => {
[00:20:04]      |
[00:20:04]      = note: expected type `std::option::Option<rustc::hir::def::Def>`
[00:20:04]                 found type `rustc::hir::def::Def`
[00:20:04] 
[00:20:04] 
[00:20:04] error[E0308]: mismatched types
[00:20:04]     --> src/librustc_resolve/lib.rs:3311:37
[00:20:04]      |
[00:20:04] 3135 |         match (def, source) {
[00:20:04]      |               ------------- this match expression has type `std::option::Option<rustc::hir::def::Def>`
[00:20:04] ...
[00:20:04] 3311 |             (Def::TyAlias(_), _) | (Def::AssociatedTy(..), _) if ns == ValueNS => {
[00:20:04]      |
[00:20:04]      = note: expected type `std::option::Option<rustc::hir::def::Def>`
[00:20:04]                 found type `rustc::hir::def::Def`
[00:20:04] 
---
travis_time:end:0270ebdd:start=1549075294256160977,finish=1549075294260974784,duration=4813807
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04155783
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:080132f6
travis_time:start:080132f6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:085211a6
$ dmesg | grep -i kill
