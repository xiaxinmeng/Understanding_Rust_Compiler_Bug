plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 21'
Agent machine name: 'fv-az578'
Current agent version: '2.170.1'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200614.1
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200614.1/images/linux/Ubuntu1604-README.md
##[endgroup]
Agent running as: 'vsts'
Prepare build directory.
Set build variables.
Download all required tasks.
Download all required tasks.
Downloading task: Bash (3.171.1)
Checking job knob settings.
   Knob: AgentToolsDirectory = /opt/hostedtoolcache Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = /home/vsts/perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/77457833-c869-4c0e-b464-77b085ec296b.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73656/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73656/merge:refs/remotes/pull/73656/merge
---
 ---> 31fea614d2f3
Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
 ---> Using cache
 ---> 4195cadf126d
Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
 ---> 4e90f6b48f05
Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
 ---> Using cache
 ---> dfa0a356d899
---
Set({"src/librustc_parse_format"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_passes"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_plugin_impl"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_privacy"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_save_analysis"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_serialize"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_session"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_span"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
---
Set({"src/librustc_parse_format"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_serialize"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
   Compiling rustc_parse_format v0.0.0 (/checkout/src/librustc_parse_format)
   Compiling chalk-engine v0.11.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.11.0
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
   Compiling rustc_span v0.0.0 (/checkout/src/librustc_span)
error: expected one of `!` or `[`, found `{`
  --> src/librustc_span/source_map.rs:43:62
   |
43 | #[derive(Clone, RustcEncodable, RustcDecodable, Debug, Copy, HashStable_Generic)]
   |                                                              ^^^^^^^^^^^^^^^^^^ expected one of `!` or `[`
error: proc-macro derive produced unparseable tokens
  --> src/librustc_span/source_map.rs:43:62
   |
   |
43 | #[derive(Clone, RustcEncodable, RustcDecodable, Debug, Copy, HashStable_Generic)]

error: expected one of `!` or `[`, found `{`
 --> src/librustc_span/edition.rs:9:10
  |
---
    |
108 | #[derive(HashStable_Generic)]
    |          ^^^^^^^^^^^^^^^^^^

error: expected one of `(` or `<`, found `{`
   --> src/librustc_span/symbol.rs:22:1
22  | /     symbols! {
22  | /     symbols! {
23  | |         // After modifying this list adjust `is_special`, `is_used_keyword`/`is_unused_keyword`,
24  | |         // this should be rarely necessary though if the keywords are kept in alphabetic order.
25  | |         Keywords {
849 | |         }
850 | |     }
    | |     ^
    | |     |
    | |     |
    | |     expected one of `(` or `<`
    | |     while parsing this item list starting here
    | |_____the item list ends here
    | 
   ::: /checkout/src/librustc_macros/src/lib.rs:20:1
    |
    |
20  |     / pub fn symbols(input: TokenStream) -> TokenStream {
21  |     |     symbols::symbols(input)
    |     |_- in this expansion of `symbols!`

error: expected one of `!` or `[`, found `{`
   --> src/librustc_span/symbol.rs:852:27
   --> src/librustc_span/symbol.rs:852:27
    |
852 | #[derive(Copy, Clone, Eq, HashStable_Generic)]
    |                           ^^^^^^^^^^^^^^^^^^ expected one of `!` or `[`
error: proc-macro derive produced unparseable tokens
   --> src/librustc_span/symbol.rs:852:27
    |
    |
852 | #[derive(Copy, Clone, Eq, HashStable_Generic)]

error: expected one of `!` or `[`, found `{`
    --> src/librustc_span/symbol.rs:22:1
     |
     |
22   | / symbols! {
23   | |     // After modifying this list adjust `is_special`, `is_used_keyword`/`is_unused_keyword`,
24   | |     // this should be rarely necessary though if the keywords are kept in alphabetic order.
25   | |     Keywords {
849  | |     }
850  | | }
     | | ^
     | | |
---

error: expected one of `!` or `[`, found `{`
    --> src/librustc_span/symbol.rs:22:1
     |
22   | / symbols! {
23   | |     // After modifying this list adjust `is_special`, `is_used_keyword`/`is_unused_keyword`,
24   | |     // this should be rarely necessary though if the keywords are kept in alphabetic order.
25   | |     Keywords {
849  | |     }
850  | | }
     | | ^
     | | |
---
     |
1018 | #[derive(HashStable_Generic)]
     |          ^^^^^^^^^^^^^^^^^^

error[E0432]: unresolved import `super::kw::MacroRules`
    --> src/librustc_span/symbol.rs:1208:13
     |
1208 |     pub use super::kw::MacroRules as macro_rules;
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `MacroRules` in `symbol::kw`
error[E0425]: cannot find value `rust_2015_preview` in module `sym`
  --> src/librustc_span/edition.rs:51:42
   |
51 |             Edition::Edition2015 => sym::rust_2015_preview,
---
    |
135 |                 || expn_data.kind == ExpnKind::Macro(MacroKind::Bang, sym::include)
    |                                                                            ^^^^^^^ not found in `sym`

error[E0425]: cannot find value `DollarCrate` in module `kw`
    |
    |
171 |                 dollar_crate_name: kw::DollarCrate,
    |                                        ^^^^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `DollarCrate` in module `kw`
    |
    |
313 |                         dollar_crate_name: kw::DollarCrate,
    |                                                ^^^^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `DollarCrate` in module `kw`
    |
    |
333 |                         dollar_crate_name: kw::DollarCrate,
    |                                                ^^^^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `DollarCrate` in module `kw`
    |
    |
349 |                 dollar_crate_name: kw::DollarCrate,
    |                                        ^^^^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `DollarCrate` in module `kw`
    |
    |
372 |                 .take_while(|scdata| scdata.dollar_crate_name == kw::DollarCrate)
    |                                                                      ^^^^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `PathRoot` in module `kw`
    |
    |
743 |             ExpnKind::Root => kw::PathRoot.to_string(),
    |                                   ^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `Invalid` in module `kw`
   --> src/librustc_span/symbol.rs:873:36
    |
873 |         Ident::with_dummy_span(kw::Invalid)
    |                                    ^^^^^^^ not found in `kw`
help: consider importing this unit variant
    |
5   | use core::num::dec2flt::parse::ParseResult::Invalid;
    |
    |

error[E0425]: cannot find value `DollarCrate` in module `kw`
    --> src/librustc_span/symbol.rs:1008:35
     |
1008 |             if self.symbol == kw::DollarCrate {
     |                                   ^^^^^^^^^^^ not found in `kw`
error[E0425]: cannot find value `digits_array` in this scope
    --> src/librustc_span/symbol.rs:1214:42
     |
     |
1214 |             if let Option::Some(&sym_) = digits_array.get(idx) {

error[E0425]: cannot find value `Async` in module `kw`
    --> src/librustc_span/symbol.rs:1224:21
     |
     |
1224 |         self >= kw::Async && self <= kw::Dyn
     |                     ^^^^^ not found in `kw`
help: consider importing this unit variant
     |
5    | use crate::hygiene::DesugaringKind::Async;
     |
     |

error[E0425]: cannot find value `Dyn` in module `kw`
    --> src/librustc_span/symbol.rs:1224:42
     |
1224 |         self >= kw::Async && self <= kw::Dyn
     |                                          ^^^ not found in `kw`
error[E0425]: cannot find value `Try` in module `kw`
    --> src/librustc_span/symbol.rs:1228:21
     |
     |
1228 |         self == kw::Try
     |                     ^^^ not found in `kw`
error[E0425]: cannot find value `Union` in module `kw`
    --> src/librustc_span/symbol.rs:1233:21
     |
     |
1233 |         self <= kw::Union
     |                     ^^^^^ not found in `kw`
error[E0425]: cannot find value `Super` in module `kw`
    --> src/librustc_span/symbol.rs:1238:21
     |
     |
1238 |         self == kw::Super
     |                     ^^^^^ not found in `kw`

error[E0425]: cannot find value `SelfLower` in module `kw`
    --> src/librustc_span/symbol.rs:1239:28
     |
1239 |             || self == kw::SelfLower
     |                            ^^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `SelfUpper` in module `kw`
    --> src/librustc_span/symbol.rs:1240:28
     |
1240 |             || self == kw::SelfUpper
     |                            ^^^^^^^^^ not found in `kw`
error[E0425]: cannot find value `Crate` in module `kw`
    --> src/librustc_span/symbol.rs:1241:28
     |
     |
1241 |             || self == kw::Crate
     |                            ^^^^^ not found in `kw`

error[E0425]: cannot find value `PathRoot` in module `kw`
    --> src/librustc_span/symbol.rs:1242:28
     |
1242 |             || self == kw::PathRoot
     |                            ^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `DollarCrate` in module `kw`
    --> src/librustc_span/symbol.rs:1243:28
     |
1243 |             || self == kw::DollarCrate
     |                            ^^^^^^^^^^^ not found in `kw`
error[E0425]: cannot find value `True` in module `kw`
    --> src/librustc_span/symbol.rs:1248:21
     |
     |
1248 |         self == kw::True || self == kw::False
     |                     ^^^^ not found in `kw`
error[E0425]: cannot find value `False` in module `kw`
    --> src/librustc_span/symbol.rs:1248:41
     |
     |
1248 |         self == kw::True || self == kw::False
     |                                         ^^^^^ not found in `kw`

error[E0425]: cannot find value `Invalid` in module `kw`
    --> src/librustc_span/symbol.rs:1253:21
     |
1253 |         self != kw::Invalid && self != kw::Underscore && !self.is_path_segment_keyword()
     |                     ^^^^^^^ not found in `kw`
help: consider importing this unit variant
     |
5    | use core::num::dec2flt::parse::ParseResult::Invalid;
     |
     |

error[E0425]: cannot find value `Underscore` in module `kw`
    --> src/librustc_span/symbol.rs:1253:44
     |
1253 |         self != kw::Invalid && self != kw::Underscore && !self.is_path_segment_keyword()
     |                                            ^^^^^^^^^^ not found in `kw`
error[E0425]: cannot find value `Underscore` in module `kw`
    --> src/librustc_span/symbol.rs:1261:26
     |
     |
1261 |         self.name <= kw::Underscore
     |                          ^^^^^^^^^^ not found in `kw`
error[E0425]: cannot find value `As` in module `kw`
    --> src/librustc_span/symbol.rs:1267:26
     |
     |
1267 |         self.name >= kw::As && self.name <= kw::While
     |                          ^^ not found in `kw`
error[E0425]: cannot find value `While` in module `kw`
    --> src/librustc_span/symbol.rs:1267:49
     |
     |
1267 |         self.name >= kw::As && self.name <= kw::While
     |                                                 ^^^^^ not found in `kw`

error[E0425]: cannot find value `Abstract` in module `kw`
    --> src/librustc_span/symbol.rs:1274:26
     |
1274 |         self.name >= kw::Abstract && self.name <= kw::Yield
     |                          ^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `Yield` in module `kw`
    --> src/librustc_span/symbol.rs:1274:55
     |
1274 |         self.name >= kw::Abstract && self.name <= kw::Yield
     |                                                       ^^^^^ not found in `kw`
error[E0425]: cannot find value `allow_internal_unstable_backcompat_hack` in module `sym`
   --> src/librustc_span/lib.rs:471:53
    |
    |
471 |                 .any(|&f| f == feature || f == sym::allow_internal_unstable_backcompat_hack)

error: unused import: `super::Symbol`
    --> src/librustc_span/symbol.rs:1191:9
     |
---

error[E0599]: no function or associated item named `fresh` found for struct `symbol::Interner` in the current scope
    --> src/librustc_span/lib.rs:77:58
     |
77   |             symbol_interner: Lock::new(symbol::Interner::fresh()),
     |                                                          ^^^^^ function or associated item not found in `symbol::Interner`
    ::: src/librustc_span/symbol.rs:1143:1
     |
1143 | pub struct Interner {
1143 | pub struct Interner {
     | ------------------- function or associated item `fresh` not found for this
error[E0599]: no method named `hash_stable` found for struct `hygiene::ExpnData` in the current scope
    --> src/librustc_span/lib.rs:1811:37
     |
     |
1811 |                 expn_id.expn_data().hash_stable(ctx, &mut hasher);
     | 
    ::: src/librustc_span/hygiene.rs:645:1
     |
645  | pub struct ExpnData {
645  | pub struct ExpnData {
     | ------------------- method `hash_stable` not found for this
     |
     = help: items from traits can only be used if the trait is implemented and in scope
     = note: the following trait defines an item `hash_stable`, perhaps you need to implement it:
             candidate #1: `rustc_data_structures::stable_hasher::HashStable`
error: aborting due to 65 previous errors

Some errors have detailed explanations: E0425, E0432, E0599.
For more information about an error, try `rustc --explain E0425`.
---
  local time: Tue Jun 23 16:59:03 UTC 2020
  network time: Tue, 23 Jun 2020 16:59:03 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73656/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73656/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3642) (python)
##[section]Finishing: Finalize Job
