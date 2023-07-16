plain
40 | extern crate rustc_ast;
   | ^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `rustc_ast`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_ast-6d7c193782263d89.rlib
           crate `rustc_ast`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_ast-e5d09eda5beb759c.rmeta
error[E0464]: multiple matching crates for `rustc_ast_lowering`
  --> src/librustdoc/lib.rs:41:1
   |
41 | extern crate rustc_ast_lowering;
---
42 | extern crate rustc_ast_pretty;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `rustc_ast_pretty`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_ast_pretty-327a918b844ab498.rlib
           crate `rustc_ast_pretty`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_ast_pretty-7a2462e1812eca9a.rmeta
error[E0464]: multiple matching crates for `rustc_attr`
  --> src/librustdoc/lib.rs:43:1
   |
43 | extern crate rustc_attr;
43 | extern crate rustc_attr;
   | ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `rustc_attr`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_attr-15281399a3c477c6.rlib
           crate `rustc_attr`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_attr-8ead6b2dfa6f8505.rmeta
error[E0464]: multiple matching crates for `rustc_const_eval`
  --> src/librustdoc/lib.rs:44:1
   |
44 | extern crate rustc_const_eval;
44 | extern crate rustc_const_eval;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `rustc_const_eval`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_const_eval-08ddca9201f7fda7.rlib
           crate `rustc_const_eval`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_const_eval-a526141767b7d837.rmeta
error[E0464]: multiple matching crates for `rustc_data_structures`
  --> src/librustdoc/lib.rs:45:1
   |
45 | extern crate rustc_data_structures;
45 | extern crate rustc_data_structures;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `rustc_data_structures`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_data_structures-3224499e6426f82c.rmeta
           crate `rustc_data_structures`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_data_structures-a45718898fb5b4a5.rlib
error[E0464]: multiple matching crates for `rustc_driver`
  --> src/librustdoc/lib.rs:46:1
   |
46 | extern crate rustc_driver;
46 | extern crate rustc_driver;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `rustc_driver`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_driver-571b914f371a6265.so
           crate `rustc_driver`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_driver-c84bc957e6d4a9d0.rmeta
error[E0464]: multiple matching crates for `rustc_errors`
  --> src/librustdoc/lib.rs:47:1
   |
47 | extern crate rustc_errors;
47 | extern crate rustc_errors;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `rustc_errors`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_errors-6387d6f64ee8936a.rlib
           crate `rustc_errors`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_errors-b82e0edcac3ffb92.rmeta
error[E0464]: multiple matching crates for `rustc_expand`
  --> src/librustdoc/lib.rs:48:1
   |
48 | extern crate rustc_expand;
48 | extern crate rustc_expand;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `rustc_expand`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_expand-3e549d57ad297e7d.rlib
           crate `rustc_expand`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_expand-45c9d931bfba1f6e.rmeta
error[E0464]: multiple matching crates for `rustc_feature`
  --> src/librustdoc/lib.rs:49:1
   |
49 | extern crate rustc_feature;
49 | extern crate rustc_feature;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `rustc_feature`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_feature-8012cea8ee30f35c.rmeta
           crate `rustc_feature`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_feature-80b20ae3f45ea4ca.rlib
error[E0464]: multiple matching crates for `rustc_hir`
  --> src/librustdoc/lib.rs:50:1
   |
50 | extern crate rustc_hir;
50 | extern crate rustc_hir;
   | ^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `rustc_hir`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_hir-0e9bf599c1dc440e.rlib
           crate `rustc_hir`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_hir-9a7a57ac5798f9e7.rmeta
error[E0464]: multiple matching crates for `rustc_hir_pretty`
  --> src/librustdoc/lib.rs:51:1
   |
51 | extern crate rustc_hir_pretty;
51 | extern crate rustc_hir_pretty;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `rustc_hir_pretty`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_hir_pretty-0cc05cc7131eeff8.rmeta
           crate `rustc_hir_pretty`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_hir_pretty-e23a83c7bf640954.rlib
error[E0464]: multiple matching crates for `rustc_index`
  --> src/librustdoc/lib.rs:52:1
   |
52 | extern crate rustc_index;
52 | extern crate rustc_index;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `rustc_index`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_index-3e8601870eb7cb85.rmeta
           crate `rustc_index`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_index-704fae4a02dc793f.rlib
error[E0464]: multiple matching crates for `rustc_infer`
  --> src/librustdoc/lib.rs:53:1
   |
53 | extern crate rustc_infer;
53 | extern crate rustc_infer;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `rustc_infer`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_infer-33f6b7535bb2a1b4.rlib
           crate `rustc_infer`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_infer-bfed6ae488f3ebe4.rmeta
error[E0464]: multiple matching crates for `rustc_interface`
  --> src/librustdoc/lib.rs:54:1
   |
54 | extern crate rustc_interface;
54 | extern crate rustc_interface;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `rustc_interface`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_interface-211cecff41002761.rlib
           crate `rustc_interface`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_interface-76ba03f932907bf1.rmeta
error[E0464]: multiple matching crates for `rustc_lexer`
  --> src/librustdoc/lib.rs:55:1
   |
55 | extern crate rustc_lexer;
55 | extern crate rustc_lexer;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `rustc_lexer`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_lexer-6c98193f2fbe8276.rlib
           crate `rustc_lexer`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_lexer-b4cecb79964239c0.rmeta
error[E0464]: multiple matching crates for `rustc_lint`
  --> src/librustdoc/lib.rs:56:1
   |
56 | extern crate rustc_lint;
56 | extern crate rustc_lint;
   | ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `rustc_lint`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_lint-1a28df8e2f1e3ee9.rlib
           crate `rustc_lint`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_lint-be117a9079a345fe.rmeta
error[E0464]: multiple matching crates for `rustc_lint_defs`
  --> src/librustdoc/lib.rs:57:1
   |
57 | extern crate rustc_lint_defs;
57 | extern crate rustc_lint_defs;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `rustc_lint_defs`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_lint_defs-2b2bfe001827d14d.rmeta
           crate `rustc_lint_defs`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_lint_defs-8626fbb27552e692.rlib
error[E0464]: multiple matching crates for `rustc_macros`
  --> src/librustdoc/lib.rs:58:1
   |
58 | extern crate rustc_macros;
58 | extern crate rustc_macros;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `rustc_macros`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_macros-207d921f8de2d509.rmeta
           crate `rustc_macros`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_macros-fd122bf744e8718e.so
error[E0464]: multiple matching crates for `rustc_metadata`
  --> src/librustdoc/lib.rs:59:1
   |
59 | extern crate rustc_metadata;
59 | extern crate rustc_metadata;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `rustc_metadata`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_metadata-1569c9976e0a19d7.rmeta
           crate `rustc_metadata`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_metadata-b09cf91e83b2bd48.rlib
error[E0464]: multiple matching crates for `rustc_middle`
  --> src/librustdoc/lib.rs:60:1
   |
60 | extern crate rustc_middle;
60 | extern crate rustc_middle;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `rustc_middle`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_middle-df5791b8066f95be.rlib
           crate `rustc_middle`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_middle-f5c9b4691fd03c0b.rmeta
error[E0464]: multiple matching crates for `rustc_parse`
  --> src/librustdoc/lib.rs:61:1
   |
61 | extern crate rustc_parse;
---
62 | extern crate rustc_passes;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `rustc_passes`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_passes-557c67a6b1ef8ca0.rlib
           crate `rustc_passes`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_passes-ad6ae43e4c933791.rmeta
error[E0464]: multiple matching crates for `rustc_resolve`
  --> src/librustdoc/lib.rs:63:1
   |
63 | extern crate rustc_resolve;
63 | extern crate rustc_resolve;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `rustc_resolve`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_resolve-5df096f7df0c022a.rlib
           crate `rustc_resolve`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_resolve-7f95aa48ceaa5053.rmeta
error[E0464]: multiple matching crates for `rustc_serialize`
  --> src/librustdoc/lib.rs:64:1
   |
64 | extern crate rustc_serialize;
64 | extern crate rustc_serialize;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `rustc_serialize`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_serialize-38275c5081697d12.rmeta
           crate `rustc_serialize`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_serialize-d12b5f8dbf8750e5.rlib
error[E0464]: multiple matching crates for `rustc_session`
  --> src/librustdoc/lib.rs:65:1
   |
65 | extern crate rustc_session;
---
66 | extern crate rustc_span;
   | ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `rustc_span`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_span-27eb0c9b8898888e.rmeta
           crate `rustc_span`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_span-dce4fa238115d17c.rlib
error[E0464]: multiple matching crates for `rustc_target`
  --> src/librustdoc/lib.rs:67:1
   |
67 | extern crate rustc_target;
67 | extern crate rustc_target;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `rustc_target`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_target-a218efee4f2444c8.rmeta
           crate `rustc_target`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_target-d1041d6155e37e95.rlib
error[E0464]: multiple matching crates for `rustc_trait_selection`
  --> src/librustdoc/lib.rs:68:1
   |
68 | extern crate rustc_trait_selection;
---
69 | extern crate rustc_typeck;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `rustc_typeck`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_typeck-4b138ddc7f5df42c.rlib
           crate `rustc_typeck`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_typeck-f98eb35b3780970c.rmeta
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
    --> src/librustdoc/clean/types.rs:1725:13
     |
     |
1725 |         use ty::fast_reject::SimplifiedTypeGen::*;
     |             ^^ use of undeclared crate or module `ty`

error[E0432]: unresolved imports `crate::lint::BROKEN_INTRA_DOC_LINKS`, `crate::lint::PRIVATE_INTRA_DOC_LINKS`, `ty::fast_reject::SimplifiedTypeGen::*`
    --> src/librustdoc/clean/types.rs:1725:13
     |
1725 |         use ty::fast_reject::SimplifiedTypeGen::*;
     |
    ::: src/librustdoc/passes/collect_intra_doc_links.rs:29:19
     |
     |
29   | use crate::lint::{BROKEN_INTRA_DOC_LINKS, PRIVATE_INTRA_DOC_LINKS};
     |                   ^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^ no `PRIVATE_INTRA_DOC_LINKS` in `lint`
     |                   |
     |                   no `BROKEN_INTRA_DOC_LINKS` in `lint`
error[E0432]: unresolved import `ty`
    --> src/librustdoc/clean/types.rs:1726:13
     |
     |
1726 |         use ty::{FloatTy, IntTy, UintTy};
     |             ^^ use of undeclared crate or module `ty`
error[E0432]: unresolved imports `DefKind`, `crate::passes::collect_intra_doc_links::preprocess_link`
   --> src/librustdoc/clean/utils.rs:393:9
    |
393 |     use DefKind::*;
---
     |
1962 |                     use DefKind::*;
     |                         ^^^^^^^ use of undeclared type `DefKind`

error: cannot find macro `bug` in this scope
   --> src/librustdoc/clean/mod.rs:109:21
    |
109 |                     bug!("clean: parenthesized `GenericBound::LangItemTrait`");

error: cannot find macro `span_bug` in this scope
   --> src/librustdoc/clean/mod.rs:140:9
    |
    |
140 |         span_bug!(cx.tcx.def_span(trait_ref.def_id), "`TraitRef` had unexpected kind {:?}", kind);


error: cannot find macro `bug` in this scope
    --> src/librustdoc/clean/mod.rs:1290:22
     |
1290 |                 _ => bug!("clean: expected associated type, found `{:?}`", ty),


error: cannot find macro `bug` in this scope
    --> src/librustdoc/clean/mod.rs:1301:37
     |
1301 |         hir::QPath::LangItem(..) => bug!("clean: requiring documentation of lang item"),

error: cannot find macro `span_bug` in this scope
    --> src/librustdoc/html/render/print_item.rs:1777:25
     |
     |
1777 |                         span_bug!(tcx.def_span(ty_def_id), "not an adt")

error: cannot find macro `span_bug` in this scope
    --> src/librustdoc/html/render/print_item.rs:1785:25
     |
     |
1785 |                         span_bug!(tcx.def_span(ty_def_id), "tag is neither niche nor int")

error: cannot find macro `declare_tool_lint` in this scope
  --> src/librustdoc/lint.rs:68:9
   |
   |
68 |           declare_tool_lint! {
   |           ^^^^^^^^^^^^^^^^^
...
74 | / declare_rustdoc_lint! {
75 | |     /// The `broken_intra_doc_links` lint detects failures in resolving
76 | |     /// intra-doc link targets. This is a `rustdoc` only lint, see the
77 | |     /// documentation in the [rustdoc book].
...  |
82 | |     "failures in resolving intra-doc link targets"
   | |_- in this macro invocation
   |
   = note: this error originates in the macro `declare_rustdoc_lint` (in Nightly builds, run with -Z macro-backtrace for more info)


error: cannot find macro `declare_tool_lint` in this scope
  --> src/librustdoc/lint.rs:68:9
   |
68 |           declare_tool_lint! {
   |           ^^^^^^^^^^^^^^^^^
...
85 | / declare_rustdoc_lint! {
86 | |     /// This is a subset of `broken_intra_doc_links` that warns when linking from
87 | |     /// a public item to a private one. This is a `rustdoc` only lint, see the
88 | |     /// documentation in the [rustdoc book].
...  |
93 | |     "linking from a public item to a private one"
   | |_- in this macro invocation
   |
   = note: this error originates in the macro `declare_rustdoc_lint` (in Nightly builds, run with -Z macro-backtrace for more info)


error: cannot find macro `declare_tool_lint` in this scope
   --> src/librustdoc/lint.rs:68:9
    |
68  |           declare_tool_lint! {
    |           ^^^^^^^^^^^^^^^^^
...
96  | / declare_rustdoc_lint! {
97  | |     /// The `invalid_codeblock_attributes` lint detects code block attributes
98  | |     /// in documentation examples that have potentially mis-typed values. This
99  | |     /// is a `rustdoc` only lint, see the documentation in the [rustdoc book].
...   |
104 | |     "codeblock attribute looks a lot like a known one"
    | |_- in this macro invocation
    |
    = note: this error originates in the macro `declare_rustdoc_lint` (in Nightly builds, run with -Z macro-backtrace for more info)


error: cannot find macro `declare_tool_lint` in this scope
   --> src/librustdoc/lint.rs:68:9
    |
68  |           declare_tool_lint! {
    |           ^^^^^^^^^^^^^^^^^
...
107 | / declare_rustdoc_lint! {
108 | |     /// The `missing_crate_level_docs` lint detects if documentation is
109 | |     /// missing at the crate root. This is a `rustdoc` only lint, see the
110 | |     /// documentation in the [rustdoc book].
...   |
115 | |     "detects crates with no crate-level documentation"
    | |_- in this macro invocation
    |
    = note: this error originates in the macro `declare_rustdoc_lint` (in Nightly builds, run with -Z macro-backtrace for more info)


error: cannot find macro `declare_tool_lint` in this scope
   --> src/librustdoc/lint.rs:68:9
    |
68  |           declare_tool_lint! {
    |           ^^^^^^^^^^^^^^^^^
...
118 | / declare_rustdoc_lint! {
119 | |     /// The `missing_doc_code_examples` lint detects publicly-exported items
120 | |     /// without code samples in their documentation. This is a `rustdoc` only
121 | |     /// lint, see the documentation in the [rustdoc book].
...   |
126 | |     "detects publicly-exported items without code samples in their documentation"
    | |_- in this macro invocation
    |
    = note: this error originates in the macro `declare_rustdoc_lint` (in Nightly builds, run with -Z macro-backtrace for more info)


error: cannot find macro `declare_tool_lint` in this scope
   --> src/librustdoc/lint.rs:68:9
    |
68  |           declare_tool_lint! {
    |           ^^^^^^^^^^^^^^^^^
...
129 | / declare_rustdoc_lint! {
130 | |     /// The `private_doc_tests` lint detects code samples in docs of private
131 | |     /// items not documented by `rustdoc`. This is a `rustdoc` only lint, see
132 | |     /// the documentation in the [rustdoc book].
...   |
137 | |     "detects code samples in docs of private items not documented by rustdoc"
    | |_- in this macro invocation
    |
    = note: this error originates in the macro `declare_rustdoc_lint` (in Nightly builds, run with -Z macro-backtrace for more info)


error: cannot find macro `declare_tool_lint` in this scope
   --> src/librustdoc/lint.rs:68:9
    |
68  |           declare_tool_lint! {
    |           ^^^^^^^^^^^^^^^^^
...
140 | / declare_rustdoc_lint! {
141 | |     /// The `invalid_html_tags` lint detects invalid HTML tags. This is a
142 | |     /// `rustdoc` only lint, see the documentation in the [rustdoc book].
...   |
...   |
147 | |     "detects invalid HTML tags in doc comments"
    | |_- in this macro invocation
    |
    = note: this error originates in the macro `declare_rustdoc_lint` (in Nightly builds, run with -Z macro-backtrace for more info)


error: cannot find macro `declare_tool_lint` in this scope
   --> src/librustdoc/lint.rs:68:9
    |
68  |           declare_tool_lint! {
    |           ^^^^^^^^^^^^^^^^^
...
150 | / declare_rustdoc_lint! {
151 | |     /// The `bare_urls` lint detects when a URL is not a hyperlink.
152 | |     /// This is a `rustdoc` only lint, see the documentation in the [rustdoc book].
...   |
...   |
157 | |     "detects URLs that are not hyperlinks"
    | |_- in this macro invocation
    |
    = note: this error originates in the macro `declare_rustdoc_lint` (in Nightly builds, run with -Z macro-backtrace for more info)


error: cannot find macro `declare_tool_lint` in this scope
   --> src/librustdoc/lint.rs:68:9
    |
68  |           declare_tool_lint! {
    |           ^^^^^^^^^^^^^^^^^
...
160 | / declare_rustdoc_lint! {
161 | |    /// The `invalid_rust_codeblocks` lint detects Rust code blocks in
162 | |    /// documentation examples that are invalid (e.g. empty, not parsable as
163 | |    /// Rust code). This is a `rustdoc` only lint, see the documentation in the
169 | |    "codeblock could not be parsed as valid Rust or is empty"
170 | | }
    | |_- in this macro invocation
    |
    |
    = note: this error originates in the macro `declare_rustdoc_lint` (in Nightly builds, run with -Z macro-backtrace for more info)

error: cannot find macro `bug` in this scope
    |
    |
156 |             other => bug!("unrecognized res {:?}", other),

error: cannot find macro `span_bug` in this scope
    --> src/librustdoc/passes/collect_intra_doc_links.rs:2170:17
     |
     |
2170 |                 span_bug!(sp, "anchors should be allowed now");


error: cannot find macro `bug` in this scope
     |
     |
2172 |                 bug!("anchors should be allowed now");

error: cannot find derive macro `Encodable` in this scope
  --> src/librustdoc/scrape_examples.rs:67:10
   |
---

error[E0433]: failed to resolve: use of undeclared crate or module `ty`
  --> src/librustdoc/clean/auto_trait.rs:36:20
   |
36 |         param_env: ty::ParamEnv<'tcx>,
   |                    ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/clean/auto_trait.rs:316:42
    |
    |
316 |     fn extract_for_generics(&self, pred: ty::Predicate<'tcx>) -> FxHashSet<GenericParamDef> {
    |                                          ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/clean/auto_trait.rs:426:20
    |
    |
426 |         param_env: ty::ParamEnv<'tcx>,
    |                    ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/clean/auto_trait.rs:428:34
    |
    |
428 |         vid_to_region: FxHashMap<ty::RegionVid, ty::Region<'tcx>>,
    |                                  ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/clean/auto_trait.rs:428:49
    |
    |
428 |         vid_to_region: FxHashMap<ty::RegionVid, ty::Region<'tcx>>,
    |                                                 ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/clean/auto_trait.rs:715:34
    |
    |
715 |     vid_to_region: &'a FxHashMap<ty::RegionVid, ty::Region<'tcx>>,
    |                                  ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/clean/auto_trait.rs:715:49
    |
    |
715 |     vid_to_region: &'a FxHashMap<ty::RegionVid, ty::Region<'tcx>>,
    |                                                 ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/clean/auto_trait.rs:724:34
    |
    |
724 |     fn fold_region(&mut self, r: ty::Region<'tcx>) -> ty::Region<'tcx> {
    |                                  ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/clean/auto_trait.rs:724:55
    |
    |
724 |     fn fold_region(&mut self, r: ty::Region<'tcx>) -> ty::Region<'tcx> {
    |                                                       ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
    --> src/librustdoc/clean/types.rs:1886:11
     |
     |
1886 | impl From<ty::IntTy> for PrimitiveType {
     |           ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
    --> src/librustdoc/clean/types.rs:1887:21
     |
1887 |     fn from(int_ty: ty::IntTy) -> PrimitiveType {
1887 |     fn from(int_ty: ty::IntTy) -> PrimitiveType {
     |                     ^^ use of undeclared crate or module `ty`

error[E0433]: failed to resolve: use of undeclared crate or module `ty`
    --> src/librustdoc/clean/types.rs:1899:11
     |
1899 | impl From<ty::UintTy> for PrimitiveType {
     |           ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
    --> src/librustdoc/clean/types.rs:1900:22
     |
     |
1900 |     fn from(uint_ty: ty::UintTy) -> PrimitiveType {
     |                      ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
    --> src/librustdoc/clean/types.rs:1912:11
     |
     |
1912 | impl From<ty::FloatTy> for PrimitiveType {
     |           ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
    --> src/librustdoc/clean/types.rs:1913:23
     |
     |
1913 |     fn from(float_ty: ty::FloatTy) -> PrimitiveType {
     |                       ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
    --> src/librustdoc/clean/types.rs:2261:21
     |
     |
2261 |     crate polarity: ty::ImplPolarity,
     |                     ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
  --> src/librustdoc/clean/utils.rs:80:15
   |
   |
80 |     substs: &[ty::subst::GenericArg<'_>],
   |               ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/clean/utils.rs:232:46
    |
    |
232 | crate fn print_const(cx: &DocContext<'_>, n: ty::Const<'_>) -> String {
    |                                              ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/clean/utils.rs:303:62
    |
    |
303 | fn print_const_with_custom_print_scalar(tcx: TyCtxt<'_>, ct: ty::Const<'_>) -> String {
    |                                                              ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/clean/utils.rs:477:30
    |
    |
477 | crate fn has_doc_flag(attrs: ty::Attributes<'_>, flag: Symbol) -> bool {
    |                              ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/clean/mod.rs:135:16
    |
    |
135 |     trait_ref: ty::TraitRef<'_>,
    |                ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/clean/mod.rs:150:22
    |
    |
150 | impl Clean<Path> for ty::TraitRef<'_> {
    |                      ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/clean/mod.rs:158:21
    |
    |
158 |     poly_trait_ref: ty::PolyTraitRef<'_>,
    |                     ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/clean/mod.rs:184:36
    |
    |
184 | impl<'tcx> Clean<GenericBound> for ty::PolyTraitRef<'tcx> {
    |                                    ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/clean/mod.rs:219:34
    |
    |
219 | impl Clean<Option<Lifetime>> for ty::Region<'_> {
    |                                  ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/clean/mod.rs:282:44
    |
    |
282 | impl<'a> Clean<Option<WherePredicate>> for ty::Predicate<'a> {
    |                                            ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/clean/mod.rs:303:44
    |
    |
303 | impl<'a> Clean<Option<WherePredicate>> for ty::PolyTraitPredicate<'a> {
    |                                            ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/clean/mod.rs:322:9
    |
    |
322 |     for ty::OutlivesPredicate<ty::Region<'tcx>, ty::Region<'tcx>>
    |         ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/clean/mod.rs:322:31
    |
    |
322 |     for ty::OutlivesPredicate<ty::Region<'tcx>, ty::Region<'tcx>>
    |                               ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/clean/mod.rs:322:49
    |
    |
322 |     for ty::OutlivesPredicate<ty::Region<'tcx>, ty::Region<'tcx>>
    |                                                 ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/clean/mod.rs:338:46
    |
    |
338 | impl<'tcx> Clean<Option<WherePredicate>> for ty::OutlivesPredicate<Ty<'tcx>, ty::Region<'tcx>> {
    |                                              ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/clean/mod.rs:338:78
    |
    |
338 | impl<'tcx> Clean<Option<WherePredicate>> for ty::OutlivesPredicate<Ty<'tcx>, ty::Region<'tcx>> {
    |                                                                              ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/clean/mod.rs:354:28
    |
    |
354 | impl<'tcx> Clean<Term> for ty::Term<'tcx> {
    |                            ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/clean/mod.rs:375:38
    |
    |
375 | impl<'tcx> Clean<WherePredicate> for ty::ProjectionPredicate<'tcx> {
    |                                      ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/clean/mod.rs:382:28
    |
    |
382 | impl<'tcx> Clean<Type> for ty::ProjectionTy<'tcx> {
    |                            ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/clean/mod.rs:396:35
    |
    |
396 | fn projection_to_path_segment(ty: ty::ProjectionTy<'_>, cx: &mut DocContext<'_>) -> PathSegment {
    |                                   ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/clean/mod.rs:408:33
    |
    |
408 | impl Clean<GenericParamDef> for ty::GenericParamDef {
    |                                 ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/clean/mod.rs:583:12
    |
    |
583 |     gens: &ty::Generics,
    |            ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/clean/mod.rs:584:12
    |
    |
584 |     preds: ty::GenericPredicates<'_>,
    |            ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/clean/mod.rs:898:10
    |
    |
898 |     sig: ty::PolyFnSig<'_>,
    |          ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
    --> src/librustdoc/clean/mod.rs:1050:22
     |
     |
1050 | impl Clean<Item> for ty::AssocItem {
     |                      ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
    --> src/librustdoc/clean/mod.rs:1676:32
     |
     |
1676 | impl<'tcx> Clean<Constant> for ty::Const<'tcx> {
     |                                ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
    --> src/librustdoc/clean/mod.rs:1693:22
     |
     |
1693 | impl Clean<Item> for ty::FieldDef {
     |                      ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
    --> src/librustdoc/clean/mod.rs:1722:28
     |
     |
1722 | impl Clean<Visibility> for ty::Visibility {
     |                            ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
    --> src/librustdoc/clean/mod.rs:1752:22
     |
     |
1752 | impl Clean<Item> for ty::VariantDef {
     |                      ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `getopts`
   --> src/librustdoc/config.rs:311:37
    |
    |
311 |     crate fn from_matches(matches: &getopts::Matches) -> Result<Options, i32> {
    |                                     ^^^^^^^ use of undeclared crate or module `getopts`
error[E0433]: failed to resolve: use of undeclared crate or module `getopts`
   --> src/librustdoc/config.rs:752:39
    |
    |
752 | fn check_deprecated_options(matches: &getopts::Matches, diag: &rustc_errors::Handler) {
    |                                       ^^^^^^^ use of undeclared crate or module `getopts`
error[E0433]: failed to resolve: use of undeclared crate or module `getopts`
   --> src/librustdoc/config.rs:791:15
    |
791 |     matches: &getopts::Matches,
791 |     matches: &getopts::Matches,
    |               ^^^^^^^ use of undeclared crate or module `getopts`

error[E0433]: failed to resolve: use of undeclared crate or module `interface`
  --> src/librustdoc/core.rs:52:32
   |
52 |     crate resolver: Rc<RefCell<interface::BoxedResolver>>,
   |                                ^^^^^^^^^ use of undeclared crate or module `interface`
error[E0433]: failed to resolve: use of undeclared crate or module `source_map`
   --> src/librustdoc/core.rs:145:28
    |
    |
145 |     source_map: Option<Lrc<source_map::SourceMap>>,
    |                            ^^^^^^^^^^ use of undeclared crate or module `source_map`

error[E0433]: failed to resolve: use of undeclared type `FxHashSet`
    |
    |
284 |                     SyncLazy::new(FxHashSet::default);
    |                                   ^^^^^^^^^ use of undeclared type `FxHashSet`
error[E0433]: failed to resolve: use of undeclared crate or module `interface`
   --> src/librustdoc/core.rs:311:26
    |
    |
311 |     resolver: Rc<RefCell<interface::BoxedResolver>>,
    |                          ^^^^^^^^^ use of undeclared crate or module `interface`
error[E0433]: failed to resolve: use of undeclared crate or module `nested_filter`
   --> src/librustdoc/core.rs:487:25
    |
    |
487 |     type NestedFilter = nested_filter::OnlyBodies;
    |                         ^^^^^^^^^^^^^ use of undeclared crate or module `nested_filter`
error[E0433]: failed to resolve: use of undeclared crate or module `intravisit`
    --> src/librustdoc/doctest.rs:1221:22
     |
     |
1221 | impl<'a, 'hir, 'tcx> intravisit::Visitor<'hir> for HirCollector<'a, 'hir, 'tcx> {
     |                      ^^^^^^^^^^ use of undeclared crate or module `intravisit`
error[E0433]: failed to resolve: use of undeclared crate or module `nested_filter`
    --> src/librustdoc/doctest.rs:1222:25
     |
     |
1222 |     type NestedFilter = nested_filter::All;
     |                         ^^^^^^^^^^^^^ use of undeclared crate or module `nested_filter`
error[E0433]: failed to resolve: use of undeclared crate or module `nested_filter`
  --> src/librustdoc/html/render/span_map.rs:97:25
   |
   |
97 |     type NestedFilter = nested_filter::All;
   |                         ^^^^^^^^^^^^^ use of undeclared crate or module `nested_filter`
error[E0433]: failed to resolve: use of undeclared crate or module `sym`
    --> src/librustdoc/html/render/mod.rs:1041:7
     |
     |
1041 |     &[sym::export_name, sym::link_section, sym::no_mangle, sym::repr, sym::non_exhaustive];
     |       ^^^ use of undeclared crate or module `sym`
error[E0433]: failed to resolve: use of undeclared crate or module `sym`
    --> src/librustdoc/html/render/mod.rs:1041:25
     |
     |
1041 |     &[sym::export_name, sym::link_section, sym::no_mangle, sym::repr, sym::non_exhaustive];
     |                         ^^^ use of undeclared crate or module `sym`
error[E0433]: failed to resolve: use of undeclared crate or module `sym`
    --> src/librustdoc/html/render/mod.rs:1041:44
     |
     |
1041 |     &[sym::export_name, sym::link_section, sym::no_mangle, sym::repr, sym::non_exhaustive];
     |                                            ^^^ use of undeclared crate or module `sym`
error[E0433]: failed to resolve: use of undeclared crate or module `sym`
    --> src/librustdoc/html/render/mod.rs:1041:60
     |
     |
1041 |     &[sym::export_name, sym::link_section, sym::no_mangle, sym::repr, sym::non_exhaustive];
     |                                                            ^^^ use of undeclared crate or module `sym`
error[E0433]: failed to resolve: use of undeclared crate or module `sym`
    --> src/librustdoc/html/render/mod.rs:1041:71
     |
     |
1041 |     &[sym::export_name, sym::link_section, sym::no_mangle, sym::repr, sym::non_exhaustive];
     |                                                                       ^^^ use of undeclared crate or module `sym`
error[E0433]: failed to resolve: use of undeclared crate or module `lint`
  --> src/librustdoc/lint.rs:26:12
   |
   |
26 |     F: Fn(&lint::Lint) -> Option<(String, lint::Level)>,
   |            ^^^^ use of undeclared crate or module `lint`
error[E0433]: failed to resolve: use of undeclared crate or module `lint`
  --> src/librustdoc/lint.rs:26:43
   |
   |
26 |     F: Fn(&lint::Lint) -> Option<(String, lint::Level)>,
   |                                           ^^^^ use of undeclared crate or module `lint`
error[E0433]: failed to resolve: use of undeclared crate or module `lint`
  --> src/librustdoc/lint.rs:22:29
   |
   |
22 |     lint_opts: Vec<(String, lint::Level)>,
   |                             ^^^^ use of undeclared crate or module `lint`
error[E0433]: failed to resolve: use of undeclared crate or module `lint`
  --> src/librustdoc/lint.rs:24:20
   |
   |
24 | ) -> (Vec<(String, lint::Level)>, FxHashMap<lint::LintId, lint::Level>)
   |                    ^^^^ use of undeclared crate or module `lint`
error[E0433]: failed to resolve: use of undeclared crate or module `lint`
  --> src/librustdoc/lint.rs:24:45
   |
   |
24 | ) -> (Vec<(String, lint::Level)>, FxHashMap<lint::LintId, lint::Level>)
   |                                             ^^^^ use of undeclared crate or module `lint`
error[E0433]: failed to resolve: use of undeclared crate or module `lint`
  --> src/librustdoc/lint.rs:24:59
   |
   |
24 | ) -> (Vec<(String, lint::Level)>, FxHashMap<lint::LintId, lint::Level>)
   |                                                           ^^^^ use of undeclared crate or module `lint`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/passes/collect_intra_doc_links.rs:305:31
    |
    |
305 |     fn from_assoc_item(item: &ty::AssocItem) -> Self {
    |                               ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/passes/collect_intra_doc_links.rs:917:13
    |
    |
917 | ) -> Option<ty::AssocItem> {
    |             ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `ty`
   --> src/librustdoc/passes/collect_intra_doc_links.rs:954:19
    |
    |
954 | ) -> Option<&'tcx ty::AssocItem> {
    |                   ^^ use of undeclared crate or module `ty`
error[E0433]: failed to resolve: use of undeclared crate or module `getopts`
  --> src/librustdoc/scrape_examples.rs:42:19
   |
42 |         matches: &getopts::Matches,
42 |         matches: &getopts::Matches,
   |                   ^^^^^^^ use of undeclared crate or module `getopts`

error[E0433]: failed to resolve: use of undeclared crate or module `nested_filter`
   --> src/librustdoc/scrape_examples.rs:128:25
    |
128 |     type NestedFilter = nested_filter::OnlyBodies;
    |                         ^^^^^^^^^^^^^ use of undeclared crate or module `nested_filter`
error[E0433]: failed to resolve: use of undeclared crate or module `interface`
   --> src/librustdoc/scrape_examples.rs:241:6
    |
241 | ) -> interface::Result<()> {
241 | ) -> interface::Result<()> {
    |      ^^^^^^^^^ use of undeclared crate or module `interface`

error[E0412]: cannot find type `Region` in this scope
  --> src/librustdoc/clean/auto_trait.rs:13:12
   |
13 |     Region(Region<'tcx>),
   |
   |
help: there is an enum variant `crate::clean::auto_trait::RegionTarget::Region`; try using the variant's enum
   |
13 |     Region(crate::clean::auto_trait::RegionTarget),


error[E0412]: cannot find type `RegionVid` in this scope
  --> src/librustdoc/clean/auto_trait.rs:14:15
   |
14 |     RegionVid(RegionVid),
   |
   |
help: there is an enum variant `crate::clean::auto_trait::RegionTarget::RegionVid`; try using the variant's enum
   |
14 |     RegionVid(crate::clean::auto_trait::RegionTarget),


error[E0412]: cannot find type `FxHashSet` in this scope
  --> src/librustdoc/clean/auto_trait.rs:19:13
   |
19 |     larger: FxHashSet<RegionTarget<'tcx>>,


error[E0412]: cannot find type `FxHashSet` in this scope
  --> src/librustdoc/clean/auto_trait.rs:20:14
   |
20 |     smaller: FxHashSet<RegionTarget<'tcx>>,

error[E0412]: cannot find type `Ty` in this scope
  --> src/librustdoc/clean/auto_trait.rs:34:13
   |
---
   |
35 |         trait_def_id: DefId,
   |                       ^^^^^ not found in this scope
   |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
   |
35 |         trait_def_id: crate::clean::ItemId,
   |                       ~~~~~~~~~~~~~~~~~~~~
35 |         trait_def_id: crate::core::ImplTraitParam,

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/clean/auto_trait.rs:37:22
   |
   |
37 |         item_def_id: DefId,
   |                      ^^^^^ not found in this scope
   |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
   |
37 |         item_def_id: crate::clean::ItemId,
   |                      ~~~~~~~~~~~~~~~~~~~~
37 |         item_def_id: crate::core::ImplTraitParam,

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/auto_trait.rs:133:59
    |
    |
133 |     crate fn get_auto_trait_impls(&mut self, item_def_id: DefId) -> Vec<Item> {
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
133 |     crate fn get_auto_trait_impls(&mut self, item_def_id: crate::clean::ItemId) -> Vec<Item> {
    |                                                           ~~~~~~~~~~~~~~~~~~~~
133 |     crate fn get_auto_trait_impls(&mut self, item_def_id: crate::core::ImplTraitParam) -> Vec<Item> {

error[E0412]: cannot find type `Region` in this scope
   --> src/librustdoc/clean/auto_trait.rs:159:29
    |
    |
159 |     fn get_lifetime(region: Region<'_>, names_map: &FxHashMap<Symbol, Lifetime>) -> Lifetime {
    |
    |
help: there is an enum variant `crate::clean::auto_trait::RegionTarget::Region`; try using the variant's enum
    |
159 |     fn get_lifetime(region: crate::clean::auto_trait::RegionTarget, names_map: &FxHashMap<Symbol, Lifetime>) -> Lifetime {


error[E0412]: cannot find type `FxHashMap` in this scope
   --> src/librustdoc/clean/auto_trait.rs:159:53
    |
159 |     fn get_lifetime(region: Region<'_>, names_map: &FxHashMap<Symbol, Lifetime>) -> Lifetime {

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/clean/auto_trait.rs:159:63
    |
    |
27  | impl<'a, 'tcx> AutoTraitFinder<'a, 'tcx> {
    |              - help: you might be missing a type parameter: `, Symbol`
...
159 |     fn get_lifetime(region: Region<'_>, names_map: &FxHashMap<Symbol, Lifetime>) -> Lifetime {

error[E0412]: cannot find type `RegionConstraintData` in this scope
   --> src/librustdoc/clean/auto_trait.rs:181:19
    |
    |
181 |         regions: &RegionConstraintData<'cx>,


error[E0412]: cannot find type `FxHashMap` in this scope
   --> src/librustdoc/clean/auto_trait.rs:182:21
    |
182 |         names_map: &FxHashMap<Symbol, Lifetime>,

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/clean/auto_trait.rs:182:31
    |
    |
27  | impl<'a, 'tcx> AutoTraitFinder<'a, 'tcx> {
    |              - help: you might be missing a type parameter: `, Symbol`
...
182 |         names_map: &FxHashMap<Symbol, Lifetime>,


error[E0412]: cannot find type `FxHashSet` in this scope
   --> src/librustdoc/clean/auto_trait.rs:316:66
    |
316 |     fn extract_for_generics(&self, pred: ty::Predicate<'tcx>) -> FxHashSet<GenericParamDef> {


error[E0412]: cannot find type `FxHashMap` in this scope
   --> src/librustdoc/clean/auto_trait.rs:347:23
    |
347 |         ty_to_bounds: FxHashMap<Type, FxHashSet<GenericBound>>,


error[E0412]: cannot find type `FxHashSet` in this scope
   --> src/librustdoc/clean/auto_trait.rs:347:39
    |
347 |         ty_to_bounds: FxHashMap<Type, FxHashSet<GenericBound>>,


error[E0412]: cannot find type `FxHashMap` in this scope
   --> src/librustdoc/clean/auto_trait.rs:348:19
    |
348 |         ty_to_fn: FxHashMap<Type, (Option<PolyTrait>, Option<Type>)>,


error[E0412]: cannot find type `FxHashMap` in this scope
   --> src/librustdoc/clean/auto_trait.rs:349:29
    |
349 |         lifetime_to_bounds: FxHashMap<Lifetime, FxHashSet<GenericBound>>,


error[E0412]: cannot find type `FxHashSet` in this scope
   --> src/librustdoc/clean/auto_trait.rs:349:49
    |
349 |         lifetime_to_bounds: FxHashMap<Lifetime, FxHashSet<GenericBound>>,

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/auto_trait.rs:425:22
    |
    |
425 |         item_def_id: DefId,
    |                      ^^^^^ not found in this scope
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
425 |         item_def_id: crate::clean::ItemId,
    |                      ~~~~~~~~~~~~~~~~~~~~
425 |         item_def_id: crate::core::ImplTraitParam,


error[E0412]: cannot find type `FxHashMap` in this scope
   --> src/librustdoc/clean/auto_trait.rs:428:24
    |
428 |         vid_to_region: FxHashMap<ty::RegionVid, ty::Region<'tcx>>,

error[E0412]: cannot find type `Region` in this scope
   --> src/librustdoc/clean/auto_trait.rs:706:24
    |
    |
706 | fn region_name(region: Region<'_>) -> Option<Symbol> {
    |
    |
help: there is an enum variant `crate::clean::auto_trait::RegionTarget::Region`; try using the variant's enum
    |
706 | fn region_name(region: crate::clean::auto_trait::RegionTarget) -> Option<Symbol> {

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/clean/auto_trait.rs:706:46
    |
    |
706 | fn region_name(region: Region<'_>) -> Option<Symbol> {
    |               -                              ^^^^^^ not found in this scope
    |               |
    |               help: you might be missing a type parameter: `<Symbol>`

error[E0412]: cannot find type `FxHashMap` in this scope
   --> src/librustdoc/clean/auto_trait.rs:715:24
    |
715 |     vid_to_region: &'a FxHashMap<ty::RegionVid, ty::Region<'tcx>>,

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/clean/auto_trait.rs:716:10
    |
    |
716 |     tcx: TyCtxt<'tcx>,
    |          ^^^^^^ not found in this scope

error[E0405]: cannot find trait `TypeFolder` in this scope
   --> src/librustdoc/clean/auto_trait.rs:719:16
    |
719 | impl<'a, 'tcx> TypeFolder<'tcx> for RegionReplacer<'a, 'tcx> {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/clean/auto_trait.rs:720:29
    |
    |
720 |     fn tcx<'b>(&'b self) -> TyCtxt<'tcx> {

error[E0412]: cannot find type `DefId` in this scope
error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/clean/blanket_impl.rs:16:56
   |
16 |     crate fn get_blanket_impls(&mut self, item_def_id: DefId) -> Vec<Item> {
   |
   |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
   |
16 |     crate fn get_blanket_impls(&mut self, item_def_id: crate::clean::ItemId) -> Vec<Item> {
   |                                                        ~~~~~~~~~~~~~~~~~~~~
16 |     crate fn get_blanket_impls(&mut self, item_def_id: crate::core::ImplTraitParam) -> Vec<Item> {

error[E0412]: cannot find type `Symbol` in this scope
  --> src/librustdoc/clean/cfg.rs:30:9
   |
   |
30 |     Cfg(Symbol, Option<Symbol>),

error[E0412]: cannot find type `Symbol` in this scope
  --> src/librustdoc/clean/cfg.rs:30:24
   |
   |
24 | crate enum Cfg {
   |               - help: you might be missing a type parameter: `<Symbol>`
...
30 |     Cfg(Symbol, Option<Symbol>),

error[E0412]: cannot find type `Span` in this scope
  --> src/librustdoc/clean/cfg.rs:42:17
   |
---
   |
6  | use tracing::Span;
   |

error[E0412]: cannot find type `NestedMetaItem` in this scope
  --> src/librustdoc/clean/cfg.rs:48:22
   |
48 |         nested_cfg: &NestedMetaItem,


error[E0412]: cannot find type `FxHashSet` in this scope
  --> src/librustdoc/clean/cfg.rs:49:19
   |
49 |         exclude: &FxHashSet<Cfg>,


error[E0412]: cannot find type `MetaItem` in this scope
  --> src/librustdoc/clean/cfg.rs:60:15
   |
60 |         cfg: &MetaItem,


error[E0412]: cannot find type `FxHashSet` in this scope
  --> src/librustdoc/clean/cfg.rs:61:19
   |
61 |         exclude: &FxHashSet<Cfg>,


error[E0412]: cannot find type `MetaItem` in this scope
   --> src/librustdoc/clean/cfg.rs:120:26
    |
120 |     crate fn parse(cfg: &MetaItem) -> Result<Cfg, InvalidCfgError> {

error[E0412]: cannot find type `ParseSess` in this scope
   --> src/librustdoc/clean/cfg.rs:128:42
    |
    |
128 |     crate fn matches(&self, parse_sess: &ParseSess, features: Option<&Features>) -> bool {

error[E0412]: cannot find type `Features` in this scope
   --> src/librustdoc/clean/cfg.rs:128:71
    |
    |
45  | impl Cfg {
    |     - help: you might be missing a type parameter: `<Features>`
...
128 |     crate fn matches(&self, parse_sess: &ParseSess, features: Option<&Features>) -> bool {

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/clean/inline.rs:43:20
   |
   |
43 |     parent_module: DefId,
   |                    ^^^^^ not found in this scope
   |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
43 |     parent_module: crate::clean::ItemId,
   |                    ~~~~~~~~~~~~~~~~~~~~
43 |     parent_module: crate::core::ImplTraitParam,
   |                    ~~~~~~~~~~~~~~~~~~~~~~~~~~~
   |                    ~~~~~~~~~~~~~~~~~~~~~~~~~~~

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/clean/inline.rs:44:27
   |
44 |     import_def_id: Option<DefId>,
   |
   |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
   |
44 |     import_def_id: Option<crate::clean::ItemId>,
   |                           ~~~~~~~~~~~~~~~~~~~~
44 |     import_def_id: Option<crate::core::ImplTraitParam>,
help: you might be missing a type parameter
   |
   |
41 | crate fn try_inline<DefId>(

error[E0412]: cannot find type `Res` in this scope
  --> src/librustdoc/clean/inline.rs:45:10
   |
   |
45 |     res: Res,
   |          ^^^ not found in this scope
   |
note: enum `crate::collect_intra_doc_links::Res` exists but is inaccessible
   |
62 | enum Res {
   | ^^^^^^^^ not accessible


error[E0412]: cannot find type `Symbol` in this scope
  --> src/librustdoc/clean/inline.rs:46:11
   |
46 |     name: Symbol,
   |           ^^^^^^ not found in this scope

error[E0412]: cannot find type `FxHashSet` in this scope
  --> src/librustdoc/clean/inline.rs:48:19
   |
48 |     visited: &mut FxHashSet<DefId>,

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/clean/inline.rs:48:29
   |
   |
48 |     visited: &mut FxHashSet<DefId>,
   |
   |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
   |
48 |     visited: &mut FxHashSet<crate::clean::ItemId>,
   |                             ~~~~~~~~~~~~~~~~~~~~
48 |     visited: &mut FxHashSet<crate::core::ImplTraitParam>,
help: you might be missing a type parameter
   |
   |
41 | crate fn try_inline<DefId>(

error[E0412]: cannot find type `Res` in this scope
   --> src/librustdoc/clean/inline.rs:139:10
    |
    |
139 |     res: Res,
    |          ^^^ not found in this scope
    |
note: enum `crate::collect_intra_doc_links::Res` exists but is inaccessible
    |
62  | enum Res {
    | ^^^^^^^^ not accessible


error[E0412]: cannot find type `FxHashSet` in this scope
   --> src/librustdoc/clean/inline.rs:140:19
    |
140 |     visited: &mut FxHashSet<DefId>,

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/inline.rs:140:29
    |
    |
140 |     visited: &mut FxHashSet<DefId>,
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
140 |     visited: &mut FxHashSet<crate::clean::ItemId>,
    |                             ~~~~~~~~~~~~~~~~~~~~
140 |     visited: &mut FxHashSet<crate::core::ImplTraitParam>,
help: you might be missing a type parameter
    |
    |
137 | crate fn try_inline_glob<DefId>(

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/inline.rs:157:55
    |
    |
157 | crate fn load_attrs<'hir>(cx: &DocContext<'hir>, did: DefId) -> Attrs<'hir> {
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
157 | crate fn load_attrs<'hir>(cx: &DocContext<'hir>, did: crate::clean::ItemId) -> Attrs<'hir> {
    |                                                       ~~~~~~~~~~~~~~~~~~~~
157 | crate fn load_attrs<'hir>(cx: &DocContext<'hir>, did: crate::core::ImplTraitParam) -> Attrs<'hir> {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/inline.rs:165:58
    |
    |
165 | crate fn record_extern_fqn(cx: &mut DocContext<'_>, did: DefId, kind: ItemType) {
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
165 | crate fn record_extern_fqn(cx: &mut DocContext<'_>, did: crate::clean::ItemId, kind: ItemType) {
    |                                                          ~~~~~~~~~~~~~~~~~~~~
165 | crate fn record_extern_fqn(cx: &mut DocContext<'_>, did: crate::core::ImplTraitParam, kind: ItemType) {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/inline.rs:193:61
    |
    |
193 | crate fn build_external_trait(cx: &mut DocContext<'_>, did: DefId) -> clean::Trait {
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
193 | crate fn build_external_trait(cx: &mut DocContext<'_>, did: crate::clean::ItemId) -> clean::Trait {
    |                                                             ~~~~~~~~~~~~~~~~~~~~
193 | crate fn build_external_trait(cx: &mut DocContext<'_>, did: crate::core::ImplTraitParam) -> clean::Trait {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/inline.rs:221:58
    |
    |
221 | fn build_external_function(cx: &mut DocContext<'_>, did: DefId) -> clean::Function {
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
221 | fn build_external_function(cx: &mut DocContext<'_>, did: crate::clean::ItemId) -> clean::Function {
    |                                                          ~~~~~~~~~~~~~~~~~~~~
221 | fn build_external_function(cx: &mut DocContext<'_>, did: crate::core::ImplTraitParam) -> clean::Function {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/inline.rs:234:45
    |
    |
234 | fn build_enum(cx: &mut DocContext<'_>, did: DefId) -> clean::Enum {
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
234 | fn build_enum(cx: &mut DocContext<'_>, did: crate::clean::ItemId) -> clean::Enum {
    |                                             ~~~~~~~~~~~~~~~~~~~~
234 | fn build_enum(cx: &mut DocContext<'_>, did: crate::core::ImplTraitParam) -> clean::Enum {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/inline.rs:244:47
    |
    |
244 | fn build_struct(cx: &mut DocContext<'_>, did: DefId) -> clean::Struct {
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
244 | fn build_struct(cx: &mut DocContext<'_>, did: crate::clean::ItemId) -> clean::Struct {
    |                                               ~~~~~~~~~~~~~~~~~~~~
244 | fn build_struct(cx: &mut DocContext<'_>, did: crate::core::ImplTraitParam) -> clean::Struct {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/inline.rs:256:46
    |
    |
256 | fn build_union(cx: &mut DocContext<'_>, did: DefId) -> clean::Union {
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
256 | fn build_union(cx: &mut DocContext<'_>, did: crate::clean::ItemId) -> clean::Union {
    |                                              ~~~~~~~~~~~~~~~~~~~~
256 | fn build_union(cx: &mut DocContext<'_>, did: crate::core::ImplTraitParam) -> clean::Union {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/inline.rs:265:51
    |
    |
265 | fn build_type_alias(cx: &mut DocContext<'_>, did: DefId) -> clean::Typedef {
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
265 | fn build_type_alias(cx: &mut DocContext<'_>, did: crate::clean::ItemId) -> clean::Typedef {
    |                                                   ~~~~~~~~~~~~~~~~~~~~
265 | fn build_type_alias(cx: &mut DocContext<'_>, did: crate::core::ImplTraitParam) -> clean::Typedef {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/inline.rs:279:27
    |
    |
279 |     parent_module: Option<DefId>,
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
279 |     parent_module: Option<crate::clean::ItemId>,
    |                           ~~~~~~~~~~~~~~~~~~~~
279 |     parent_module: Option<crate::core::ImplTraitParam>,
help: you might be missing a type parameter
    |
    |
277 | crate fn build_impls<DefId>(

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/inline.rs:280:10
    |
    |
280 |     did: DefId,
    |          ^^^^^ not found in this scope
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
280 |     did: crate::clean::ItemId,
    |          ~~~~~~~~~~~~~~~~~~~~
280 |     did: crate::core::ImplTraitParam,

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/inline.rs:296:27
    |
    |
296 |     parent_module: Option<DefId>,
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
296 |     parent_module: Option<crate::clean::ItemId>,
    |                           ~~~~~~~~~~~~~~~~~~~~
296 |     parent_module: Option<crate::core::ImplTraitParam>,
help: you might be missing a type parameter
    |
    |
294 | fn merge_attrs<DefId>(

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/inline.rs:323:27
    |
    |
323 |     parent_module: Option<DefId>,
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
323 |     parent_module: Option<crate::clean::ItemId>,
    |                           ~~~~~~~~~~~~~~~~~~~~
323 |     parent_module: Option<crate::core::ImplTraitParam>,
help: you might be missing a type parameter
    |
    |
321 | crate fn build_impl<DefId>(

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/inline.rs:324:10
    |
    |
324 |     did: DefId,
    |          ^^^^^ not found in this scope
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
324 |     did: crate::clean::ItemId,
    |          ~~~~~~~~~~~~~~~~~~~~
324 |     did: crate::core::ImplTraitParam,

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/inline.rs:515:10
    |
    |
515 |     did: DefId,
    |          ^^^^^ not found in this scope
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
515 |     did: crate::clean::ItemId,
    |          ~~~~~~~~~~~~~~~~~~~~
515 |     did: crate::core::ImplTraitParam,


error[E0412]: cannot find type `FxHashSet` in this scope
   --> src/librustdoc/clean/inline.rs:516:19
    |
516 |     visited: &mut FxHashSet<DefId>,

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/inline.rs:516:29
    |
    |
516 |     visited: &mut FxHashSet<DefId>,
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
516 |     visited: &mut FxHashSet<crate::clean::ItemId>,
    |                             ~~~~~~~~~~~~~~~~~~~~
516 |     visited: &mut FxHashSet<crate::core::ImplTraitParam>,
help: you might be missing a type parameter
    |
    |
513 | fn build_module<DefId>(

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/clean/inline.rs:568:35
    |
    |
568 | crate fn print_inlined_const(tcx: TyCtxt<'_>, did: DefId) -> String {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/inline.rs:568:52
    |
    |
568 | crate fn print_inlined_const(tcx: TyCtxt<'_>, did: DefId) -> String {
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
568 | crate fn print_inlined_const(tcx: TyCtxt<'_>, did: crate::clean::ItemId) -> String {
    |                                                    ~~~~~~~~~~~~~~~~~~~~
568 | crate fn print_inlined_const(tcx: TyCtxt<'_>, did: crate::core::ImplTraitParam) -> String {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/inline.rs:577:49
    |
    |
577 | fn build_const(cx: &mut DocContext<'_>, def_id: DefId) -> clean::Constant {
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
577 | fn build_const(cx: &mut DocContext<'_>, def_id: crate::clean::ItemId) -> clean::Constant {
    |                                                 ~~~~~~~~~~~~~~~~~~~~
577 | fn build_const(cx: &mut DocContext<'_>, def_id: crate::core::ImplTraitParam) -> clean::Constant {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/inline.rs:584:47
    |
    |
584 | fn build_static(cx: &mut DocContext<'_>, did: DefId, mutable: bool) -> clean::Static {
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
584 | fn build_static(cx: &mut DocContext<'_>, did: crate::clean::ItemId, mutable: bool) -> clean::Static {
    |                                               ~~~~~~~~~~~~~~~~~~~~
584 | fn build_static(cx: &mut DocContext<'_>, did: crate::core::ImplTraitParam, mutable: bool) -> clean::Static {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/inline.rs:594:13
    |
    |
594 |     def_id: DefId,
    |             ^^^^^ not found in this scope
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
594 |     def_id: crate::clean::ItemId,
    |             ~~~~~~~~~~~~~~~~~~~~
594 |     def_id: crate::core::ImplTraitParam,

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/clean/inline.rs:595:11
    |
    |
595 |     name: Symbol,
    |           ^^^^^^ not found in this scope

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/inline.rs:596:27
    |
596 |     import_def_id: Option<DefId>,
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
596 |     import_def_id: Option<crate::clean::ItemId>,
    |                           ~~~~~~~~~~~~~~~~~~~~
596 |     import_def_id: Option<crate::core::ImplTraitParam>,
help: you might be missing a type parameter
    |
    |
592 | fn build_macro<DefId>(

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/inline.rs:624:41
    |
    |
624 | fn filter_non_trait_generics(trait_did: DefId, mut g: clean::Generics) -> clean::Generics {
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
624 | fn filter_non_trait_generics(trait_did: crate::clean::ItemId, mut g: clean::Generics) -> clean::Generics {
    |                                         ~~~~~~~~~~~~~~~~~~~~
624 | fn filter_non_trait_generics(trait_did: crate::core::ImplTraitParam, mut g: clean::Generics) -> clean::Generics {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/inline.rs:673:60
    |
    |
673 | crate fn record_extern_trait(cx: &mut DocContext<'_>, did: DefId) {
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
673 | crate fn record_extern_trait(cx: &mut DocContext<'_>, did: crate::clean::ItemId) {
    |                                                            ~~~~~~~~~~~~~~~~~~~~
673 | crate fn record_extern_trait(cx: &mut DocContext<'_>, did: crate::core::ImplTraitParam) {

error[E0412]: cannot find type `TyCtxt` in this scope
  --> src/librustdoc/clean/render_macro_matchers.rs:13:41
   |
   |
13 | pub(super) fn render_macro_matcher(tcx: TyCtxt<'_>, matcher: &TokenTree) -> String {

error[E0412]: cannot find type `TokenTree` in this scope
  --> src/librustdoc/clean/render_macro_matchers.rs:13:63
   |
   |
13 | pub(super) fn render_macro_matcher(tcx: TyCtxt<'_>, matcher: &TokenTree) -> String {

error[E0412]: cannot find type `TyCtxt` in this scope
  --> src/librustdoc/clean/render_macro_matchers.rs:57:32
   |
   |
57 | fn snippet_equal_to_token(tcx: TyCtxt<'_>, matcher: &TokenTree) -> Option<String> {

error[E0412]: cannot find type `TokenTree` in this scope
  --> src/librustdoc/clean/render_macro_matchers.rs:57:54
   |
   |
---
   |
85 |     trait_did: DefId,
   |                ^^^^^ not found in this scope
   |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
   |
85 |     trait_did: crate::clean::ItemId,
   |                ~~~~~~~~~~~~~~~~~~~~
85 |     trait_did: crate::core::ImplTraitParam,

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/simplify.rs:121:60
    |
    |
121 | fn trait_is_same_or_supertrait(cx: &DocContext<'_>, child: DefId, trait_: DefId) -> bool {
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
121 | fn trait_is_same_or_supertrait(cx: &DocContext<'_>, child: crate::clean::ItemId, trait_: DefId) -> bool {
    |                                                            ~~~~~~~~~~~~~~~~~~~~
121 | fn trait_is_same_or_supertrait(cx: &DocContext<'_>, child: crate::core::ImplTraitParam, trait_: DefId) -> bool {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/simplify.rs:121:75
    |
    |
121 | fn trait_is_same_or_supertrait(cx: &DocContext<'_>, child: DefId, trait_: DefId) -> bool {
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
121 | fn trait_is_same_or_supertrait(cx: &DocContext<'_>, child: DefId, trait_: crate::clean::ItemId) -> bool {
    |                                                                           ~~~~~~~~~~~~~~~~~~~~
121 | fn trait_is_same_or_supertrait(cx: &DocContext<'_>, child: DefId, trait_: crate::core::ImplTraitParam) -> bool {


error[E0412]: cannot find type `FxHashSet` in this scope
  --> src/librustdoc/clean/types.rs:58:24
   |
58 | crate type ItemIdSet = FxHashSet<ItemId>;

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/clean/types.rs:63:11
   |
   |
63 |     DefId(DefId),
   |           ^^^^^ not found in this scope
   |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
   |
63 |     DefId(crate::clean::ItemId),
   |           ~~~~~~~~~~~~~~~~~~~~
63 |     DefId(crate::core::ImplTraitParam),

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/clean/types.rs:65:20
   |
   |
65 |     Auto { trait_: DefId, for_: DefId },
   |
   |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
   |
65 |     Auto { trait_: crate::clean::ItemId, for_: DefId },
   |                    ~~~~~~~~~~~~~~~~~~~~
65 |     Auto { trait_: crate::core::ImplTraitParam, for_: DefId },

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/clean/types.rs:65:33
   |
   |
65 |     Auto { trait_: DefId, for_: DefId },
   |
   |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
   |
65 |     Auto { trait_: DefId, for_: crate::clean::ItemId },
   |                                 ~~~~~~~~~~~~~~~~~~~~
65 |     Auto { trait_: DefId, for_: crate::core::ImplTraitParam },

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/clean/types.rs:67:24
   |
   |
67 |     Blanket { impl_id: DefId, for_: DefId },
   |
   |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
   |
67 |     Blanket { impl_id: crate::clean::ItemId, for_: DefId },
   |                        ~~~~~~~~~~~~~~~~~~~~
67 |     Blanket { impl_id: crate::core::ImplTraitParam, for_: DefId },

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/clean/types.rs:67:37
   |
   |
67 |     Blanket { impl_id: DefId, for_: DefId },
   |
   |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
   |
67 |     Blanket { impl_id: DefId, for_: crate::clean::ItemId },
   |                                     ~~~~~~~~~~~~~~~~~~~~
67 |     Blanket { impl_id: DefId, for_: crate::core::ImplTraitParam },

error[E0412]: cannot find type `CrateNum` in this scope
  --> src/librustdoc/clean/types.rs:69:30
   |
   |
69 |     Primitive(PrimitiveType, CrateNum),

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/clean/types.rs:85:37
   |
   |
85 |     crate fn expect_def_id(self) -> DefId {
   |
   |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
   |
85 |     crate fn expect_def_id(self) -> crate::clean::ItemId {
   |                                     ~~~~~~~~~~~~~~~~~~~~
85 |     crate fn expect_def_id(self) -> crate::core::ImplTraitParam {

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/clean/types.rs:91:40
   |
   |
91 |     crate fn as_def_id(self) -> Option<DefId> {
   |
   |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
   |
91 |     crate fn as_def_id(self) -> Option<crate::clean::ItemId> {
   |                                        ~~~~~~~~~~~~~~~~~~~~
91 |     crate fn as_def_id(self) -> Option<crate::core::ImplTraitParam> {
help: you might be missing a type parameter
   |
   |
72 | impl<DefId> ItemId {

error[E0412]: cannot find type `CrateNum` in this scope
  --> src/librustdoc/clean/types.rs:99:29
   |
   |
99 |     crate fn krate(self) -> CrateNum {
   |                             ^^^^^^^^ not found in this scope

error[E0412]: cannot find type `DefIndex` in this scope
   --> src/librustdoc/clean/types.rs:109:36
    |
72  | impl ItemId {
    |     - help: you might be missing a type parameter: `<DefIndex>`
...
109 |     crate fn index(self) -> Option<DefIndex> {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/types.rs:117:11
    |
    |
117 | impl From<DefId> for ItemId {
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
117 | impl From<crate::clean::ItemId> for ItemId {
    |           ~~~~~~~~~~~~~~~~~~~~
117 | impl From<crate::core::ImplTraitParam> for ItemId {
help: you might be missing a type parameter
    |
    |
117 | impl<DefId> From<DefId> for ItemId {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/types.rs:118:17
    |
    |
118 |     fn from(id: DefId) -> Self {
    |                 ^^^^^ not found in this scope
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
118 |     fn from(id: crate::clean::ItemId) -> Self {
    |                 ~~~~~~~~~~~~~~~~~~~~
118 |     fn from(id: crate::core::ImplTraitParam) -> Self {

error[E0412]: cannot find type `ThinVec` in this scope
   --> src/librustdoc/clean/types.rs:127:23
    |
    |
127 |     crate primitives: ThinVec<(DefId, PrimitiveType)>,

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/types.rs:127:32
    |
    |
127 |     crate primitives: ThinVec<(DefId, PrimitiveType)>,
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
127 |     crate primitives: ThinVec<(crate::clean::ItemId, PrimitiveType)>,
    |                                ~~~~~~~~~~~~~~~~~~~~
127 |     crate primitives: ThinVec<(crate::core::ImplTraitParam, PrimitiveType)>,
help: you might be missing a type parameter
    |
    |
125 | crate struct Crate<DefId> {


error[E0412]: cannot find type `FxHashMap` in this scope
   --> src/librustdoc/clean/types.rs:129:39
    |
129 |     crate external_traits: Rc<RefCell<FxHashMap<DefId, TraitWithExtraInfo>>>,

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/types.rs:129:49
    |
    |
129 |     crate external_traits: Rc<RefCell<FxHashMap<DefId, TraitWithExtraInfo>>>,
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
129 |     crate external_traits: Rc<RefCell<FxHashMap<crate::clean::ItemId, TraitWithExtraInfo>>>,
    |                                                 ~~~~~~~~~~~~~~~~~~~~
129 |     crate external_traits: Rc<RefCell<FxHashMap<crate::core::ImplTraitParam, TraitWithExtraInfo>>>,
help: you might be missing a type parameter
    |
    |
125 | crate struct Crate<DefId> {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/clean/types.rs:137:31
    |
    |
137 |     crate fn name(&self, tcx: TyCtxt<'_>) -> Symbol {

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/clean/types.rs:137:46
    |
    |
137 |     crate fn name(&self, tcx: TyCtxt<'_>) -> Symbol {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/clean/types.rs:141:30
    |
    |
141 |     crate fn src(&self, tcx: TyCtxt<'_>) -> FileName {

error[E0412]: cannot find type `FileName` in this scope
   --> src/librustdoc/clean/types.rs:141:45
    |
    |
141 |     crate fn src(&self, tcx: TyCtxt<'_>) -> FileName {

error[E0412]: cannot find type `CrateNum` in this scope
   --> src/librustdoc/clean/types.rs:155:22
    |
    |
155 |     crate crate_num: CrateNum,
    |                      ^^^^^^^^ not found in this scope

error[E0425]: cannot find value `LOCAL_CRATE` in this scope
   --> src/librustdoc/clean/types.rs:159:43
    |
159 |     const LOCAL: Self = Self { crate_num: LOCAL_CRATE };

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/types.rs:162:31
    |
    |
162 |     crate fn def_id(&self) -> DefId {
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
162 |     crate fn def_id(&self) -> crate::clean::ItemId {
    |                               ~~~~~~~~~~~~~~~~~~~~
162 |     crate fn def_id(&self) -> crate::core::ImplTraitParam {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/clean/types.rs:166:30
    |
    |
166 |     crate fn src(&self, tcx: TyCtxt<'_>) -> FileName {

error[E0412]: cannot find type `FileName` in this scope
   --> src/librustdoc/clean/types.rs:166:45
    |
    |
166 |     crate fn src(&self, tcx: TyCtxt<'_>) -> FileName {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/clean/types.rs:171:31
    |
    |
171 |     crate fn name(&self, tcx: TyCtxt<'_>) -> Symbol {

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/clean/types.rs:171:46
    |
    |
171 |     crate fn name(&self, tcx: TyCtxt<'_>) -> Symbol {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/clean/types.rs:175:35
    |
    |
175 |     crate fn src_root(&self, tcx: TyCtxt<'_>) -> PathBuf {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/clean/types.rs:192:14
    |
    |
192 |         tcx: TyCtxt<'_>,
    |              ^^^^^^ not found in this scope

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/clean/types.rs:231:35
    |
231 |     crate fn keywords(&self, tcx: TyCtxt<'_>) -> ThinVec<(DefId, Symbol)> {

error[E0412]: cannot find type `ThinVec` in this scope
   --> src/librustdoc/clean/types.rs:231:50
    |
    |
231 |     crate fn keywords(&self, tcx: TyCtxt<'_>) -> ThinVec<(DefId, Symbol)> {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/types.rs:231:59
    |
    |
231 |     crate fn keywords(&self, tcx: TyCtxt<'_>) -> ThinVec<(DefId, Symbol)> {
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
231 |     crate fn keywords(&self, tcx: TyCtxt<'_>) -> ThinVec<(crate::clean::ItemId, Symbol)> {
    |                                                           ~~~~~~~~~~~~~~~~~~~~
231 |     crate fn keywords(&self, tcx: TyCtxt<'_>) -> ThinVec<(crate::core::ImplTraitParam, Symbol)> {
help: you might be missing a type parameter
    |
    |
158 | impl<DefId> ExternalCrate {

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/clean/types.rs:231:66
    |
    |
158 | impl ExternalCrate {
    |     - help: you might be missing a type parameter: `<Symbol>`
...
231 |     crate fn keywords(&self, tcx: TyCtxt<'_>) -> ThinVec<(DefId, Symbol)> {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/clean/types.rs:276:37
    |
    |
276 |     crate fn primitives(&self, tcx: TyCtxt<'_>) -> ThinVec<(DefId, PrimitiveType)> {

error[E0412]: cannot find type `ThinVec` in this scope
   --> src/librustdoc/clean/types.rs:276:52
    |
    |
276 |     crate fn primitives(&self, tcx: TyCtxt<'_>) -> ThinVec<(DefId, PrimitiveType)> {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/types.rs:276:61
    |
    |
276 |     crate fn primitives(&self, tcx: TyCtxt<'_>) -> ThinVec<(DefId, PrimitiveType)> {
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
276 |     crate fn primitives(&self, tcx: TyCtxt<'_>) -> ThinVec<(crate::clean::ItemId, PrimitiveType)> {
    |                                                             ~~~~~~~~~~~~~~~~~~~~
276 |     crate fn primitives(&self, tcx: TyCtxt<'_>) -> ThinVec<(crate::core::ImplTraitParam, PrimitiveType)> {
help: you might be missing a type parameter
    |
    |
158 | impl<DefId> ExternalCrate {

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/clean/types.rs:363:24
    |
    |
360 | crate struct Item {
    |                  - help: you might be missing a type parameter: `<Symbol>`
363 |     crate name: Option<Symbol>,
    |                        ^^^^^^ not found in this scope

error[E0412]: cannot find type `DefId` in this scope
error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/types.rs:399:29
    |
399 | crate fn rustc_span(def_id: DefId, tcx: TyCtxt<'_>) -> Span {
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
399 | crate fn rustc_span(def_id: crate::clean::ItemId, tcx: TyCtxt<'_>) -> Span {
    |                             ~~~~~~~~~~~~~~~~~~~~
399 | crate fn rustc_span(def_id: crate::core::ImplTraitParam, tcx: TyCtxt<'_>) -> Span {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/clean/types.rs:399:41
    |
    |
399 | crate fn rustc_span(def_id: DefId, tcx: TyCtxt<'_>) -> Span {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/clean/types.rs:410:42
    |
    |
410 |     crate fn stability<'tcx>(&self, tcx: TyCtxt<'tcx>) -> Option<Stability> {

error[E0412]: cannot find type `Stability` in this scope
   --> src/librustdoc/clean/types.rs:410:66
    |
    |
409 | impl Item {
    |     - help: you might be missing a type parameter: `<Stability>`
410 |     crate fn stability<'tcx>(&self, tcx: TyCtxt<'tcx>) -> Option<Stability> {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/clean/types.rs:414:48
    |
    |
414 |     crate fn const_stability<'tcx>(&self, tcx: TyCtxt<'tcx>) -> Option<ConstStability> {

error[E0412]: cannot find type `ConstStability` in this scope
   --> src/librustdoc/clean/types.rs:414:72
    |
    |
409 | impl Item {
    |     - help: you might be missing a type parameter: `<ConstStability>`
...
414 |     crate fn const_stability<'tcx>(&self, tcx: TyCtxt<'tcx>) -> Option<ConstStability> {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/clean/types.rs:418:38
    |
    |
418 |     crate fn deprecation(&self, tcx: TyCtxt<'_>) -> Option<Deprecation> {

error[E0412]: cannot find type `Deprecation` in this scope
   --> src/librustdoc/clean/types.rs:418:60
    |
    |
418 |     crate fn deprecation(&self, tcx: TyCtxt<'_>) -> Option<Deprecation> {
    |
help: consider importing this struct
    |
1   | use rustdoc_json_types::Deprecation;
1   | use rustdoc_json_types::Deprecation;
    |

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/clean/types.rs:422:37
    |
422 |     crate fn inner_docs(&self, tcx: TyCtxt<'_>) -> bool {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/clean/types.rs:426:31
    |
    |
426 |     crate fn span(&self, tcx: TyCtxt<'_>) -> Span {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/clean/types.rs:447:36
    |
    |
447 |     crate fn attr_span(&self, tcx: TyCtxt<'_>) -> rustc_span::Span {

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/clean/types.rs:461:22
    |
    |
409 | impl Item {
    |     - help: you might be missing a type parameter: `<Symbol>`
461 |         name: Option<Symbol>,
    |                      ^^^^^^ not found in this scope

error[E0412]: cannot find type `DefId` in this scope
error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/types.rs:469:17
    |
469 |         def_id: DefId,
    |                 ^^^^^ not found in this scope
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
469 |         def_id: crate::clean::ItemId,
    |                 ~~~~~~~~~~~~~~~~~~~~
469 |         def_id: crate::core::ImplTraitParam,

---
     |
1051 |     crate did: DefId,
     |                ^^^^^ not found in this scope
     |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
     |
1051 |     crate did: crate::clean::ItemId,
     |                ~~~~~~~~~~~~~~~~~~~~
1051 |     crate did: crate::core::ImplTraitParam,

error[E0412]: cannot find type `Symbol` in this scope
    --> src/librustdoc/clean/types.rs:1076:33
     |
     |
1076 |     crate fn lists(&self, name: Symbol) -> impl Iterator<Item = ast::NestedMetaItem> + '_ {

error[E0412]: cannot find type `Symbol` in this scope
    --> src/librustdoc/clean/types.rs:1080:40
     |
     |
1080 |     crate fn has_doc_flag(&self, flag: Symbol) -> bool {

error[E0412]: cannot find type `DefId` in this scope
    --> src/librustdoc/clean/types.rs:1098:54
     |
     |
1098 |         additional_attrs: Option<(&[ast::Attribute], DefId)>,
     |
     |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
     |
1098 |         additional_attrs: Option<(&[ast::Attribute], crate::clean::ItemId)>,
     |                                                      ~~~~~~~~~~~~~~~~~~~~
1098 |         additional_attrs: Option<(&[ast::Attribute], crate::core::ImplTraitParam)>,
help: you might be missing a type parameter
     |
     |
1075 | impl<DefId> Attributes {


error[E0412]: cannot find type `FxHashMap` in this scope
    --> src/librustdoc/clean/types.rs:1151:60
     |
1151 |     crate fn collapsed_doc_value_by_module_level(&self) -> FxHashMap<Option<DefId>, String> {

error[E0412]: cannot find type `DefId` in this scope
    --> src/librustdoc/clean/types.rs:1151:77
     |
     |
1151 |     crate fn collapsed_doc_value_by_module_level(&self) -> FxHashMap<Option<DefId>, String> {
     |
     |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
     |
1151 |     crate fn collapsed_doc_value_by_module_level(&self) -> FxHashMap<Option<crate::clean::ItemId>, String> {
     |                                                                             ~~~~~~~~~~~~~~~~~~~~
1151 |     crate fn collapsed_doc_value_by_module_level(&self) -> FxHashMap<Option<crate::core::ImplTraitParam>, String> {
help: you might be missing a type parameter
     |
     |
1075 | impl<DefId> Attributes {

error[E0412]: cannot find type `Symbol` in this scope
    --> src/librustdoc/clean/types.rs:1178:45
     |
     |
1075 | impl Attributes {
     |     - help: you might be missing a type parameter: `<Symbol>`
...
1178 |     crate fn get_doc_aliases(&self) -> Box<[Symbol]> {

error[E0412]: cannot find type `Symbol` in this scope
    --> src/librustdoc/clean/types.rs:1257:27
     |
     |
1257 | crate struct Lifetime(pub Symbol);

error[E0412]: cannot find type `DefId` in this scope
    --> src/librustdoc/clean/types.rs:1289:17
     |
     |
1289 |     Type { did: DefId, bounds: Vec<GenericBound>, default: Option<Box<Type>>, synthetic: bool },
     |
     |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
     |
1289 |     Type { did: crate::clean::ItemId, bounds: Vec<GenericBound>, default: Option<Box<Type>>, synthetic: bool },
     |                 ~~~~~~~~~~~~~~~~~~~~
1289 |     Type { did: crate::core::ImplTraitParam, bounds: Vec<GenericBound>, default: Option<Box<Type>>, synthetic: bool },

error[E0412]: cannot find type `DefId` in this scope
    --> src/librustdoc/clean/types.rs:1290:18
     |
     |
1290 |     Const { did: DefId, ty: Box<Type>, default: Option<Box<String>> },
     |
     |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
     |
1290 |     Const { did: crate::clean::ItemId, ty: Box<Type>, default: Option<Box<String>> },
     |                  ~~~~~~~~~~~~~~~~~~~~
1290 |     Const { did: crate::core::ImplTraitParam, ty: Box<Type>, default: Option<Box<String>> },

error[E0412]: cannot find type `Symbol` in this scope
    --> src/librustdoc/clean/types.rs:1301:17
     |
---

error[E0412]: cannot find type `Mutability` in this scope
    --> src/librustdoc/clean/types.rs:1396:36
     |
1396 |     SelfBorrowed(Option<Lifetime>, Mutability),

error[E0412]: cannot find type `Symbol` in this scope
    --> src/librustdoc/clean/types.rs:1465:13
     |
     |
1465 |     Generic(Symbol),

error[E0412]: cannot find type `Mutability` in this scope
    --> src/librustdoc/clean/types.rs:1479:16
     |
     |
1479 |     RawPointer(Mutability, Box<Type>),

error[E0412]: cannot find type `Mutability` in this scope
    --> src/librustdoc/clean/types.rs:1481:59
     |
     |
1481 |     BorrowedRef { lifetime: Option<Lifetime>, mutability: Mutability, type_: Box<Type> },

error[E0412]: cannot find type `DefId` in this scope
    --> src/librustdoc/clean/types.rs:1490:29
     |
     |
1490 |         self_def_id: Option<DefId>,
     |
     |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
     |
1490 |         self_def_id: Option<crate::clean::ItemId>,
     |                             ~~~~~~~~~~~~~~~~~~~~
1490 |         self_def_id: Option<crate::core::ImplTraitParam>,
help: you might be missing a type parameter
     |
     |
1456 | crate enum Type<DefId> {

error[E0412]: cannot find type `DefId` in this scope
    --> src/librustdoc/clean/types.rs:1593:50
     |
     |
1593 |     crate fn projection(&self) -> Option<(&Type, DefId, PathSegment)> {
     |
     |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
     |
1593 |     crate fn projection(&self) -> Option<(&Type, crate::clean::ItemId, PathSegment)> {
     |                                                  ~~~~~~~~~~~~~~~~~~~~
1593 |     crate fn projection(&self) -> Option<(&Type, crate::core::ImplTraitParam, PathSegment)> {
help: you might be missing a type parameter
     |
     |
1505 | impl<DefId> Type {

error[E0412]: cannot find type `DefId` in this scope
    --> src/librustdoc/clean/types.rs:1601:61
     |
     |
1601 |     fn inner_def_id(&self, cache: Option<&Cache>) -> Option<DefId> {
     |
     |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
     |
1601 |     fn inner_def_id(&self, cache: Option<&Cache>) -> Option<crate::clean::ItemId> {
     |                                                             ~~~~~~~~~~~~~~~~~~~~
1601 |     fn inner_def_id(&self, cache: Option<&Cache>) -> Option<crate::core::ImplTraitParam> {
help: you might be missing a type parameter
     |
     |
1505 | impl<DefId> Type {

error[E0412]: cannot find type `DefId` in this scope
    --> src/librustdoc/clean/types.rs:1628:53
     |
     |
1628 |     crate fn def_id(&self, cache: &Cache) -> Option<DefId> {
     |
     |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
     |
1628 |     crate fn def_id(&self, cache: &Cache) -> Option<crate::clean::ItemId> {
     |                                                     ~~~~~~~~~~~~~~~~~~~~
1628 |     crate fn def_id(&self, cache: &Cache) -> Option<crate::core::ImplTraitParam> {
help: you might be missing a type parameter
     |
     |
1505 | impl<DefId> Type {


error[E0412]: cannot find type `FxHashMap` in this scope
    --> src/librustdoc/clean/types.rs:1668:24
     |
1668 | type SimplifiedTypes = FxHashMap<PrimitiveType, ArrayVec<SimplifiedType, 2>>;

error[E0412]: cannot find type `SimplifiedType` in this scope
    --> src/librustdoc/clean/types.rs:1668:58
     |
     |
1668 | type SimplifiedTypes = FxHashMap<PrimitiveType, ArrayVec<SimplifiedType, 2>>;
     |                     -                                    ^^^^^^^^^^^^^^ not found in this scope
     |                     |
     |                     help: you might be missing a type parameter: `<SimplifiedType>`
error[E0412]: cannot find type `Symbol` in this scope
    --> src/librustdoc/clean/types.rs:1693:29
     |
     |
1693 |     crate fn from_symbol(s: Symbol) -> Option<PrimitiveType> {

error[E0412]: cannot find type `TyCtxt` in this scope
    --> src/librustdoc/clean/types.rs:1769:38
     |
     |
1769 |     crate fn impls<'tcx>(&self, tcx: TyCtxt<'tcx>) -> impl Iterator<Item = DefId> + 'tcx {

error[E0412]: cannot find type `DefId` in this scope
    --> src/librustdoc/clean/types.rs:1769:76
     |
     |
1769 |     crate fn impls<'tcx>(&self, tcx: TyCtxt<'tcx>) -> impl Iterator<Item = DefId> + 'tcx {
     |
     |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
     |
1769 |     crate fn impls<'tcx>(&self, tcx: TyCtxt<'tcx>) -> impl Iterator<Item = crate::clean::ItemId> + 'tcx {
     |                                                                            ~~~~~~~~~~~~~~~~~~~~
1769 |     crate fn impls<'tcx>(&self, tcx: TyCtxt<'tcx>) -> impl Iterator<Item = crate::core::ImplTraitParam> + 'tcx {

error[E0412]: cannot find type `TyCtxt` in this scope
    --> src/librustdoc/clean/types.rs:1778:29
     |
     |
1778 |     crate fn all_impls(tcx: TyCtxt<'_>) -> impl Iterator<Item = DefId> + '_ {

error[E0412]: cannot find type `DefId` in this scope
    --> src/librustdoc/clean/types.rs:1778:65
     |
     |
1778 |     crate fn all_impls(tcx: TyCtxt<'_>) -> impl Iterator<Item = DefId> + '_ {
     |
     |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
     |
1778 |     crate fn all_impls(tcx: TyCtxt<'_>) -> impl Iterator<Item = crate::clean::ItemId> + '_ {
     |                                                                 ~~~~~~~~~~~~~~~~~~~~
1778 |     crate fn all_impls(tcx: TyCtxt<'_>) -> impl Iterator<Item = crate::core::ImplTraitParam> + '_ {

error[E0412]: cannot find type `Symbol` in this scope
    --> src/librustdoc/clean/types.rs:1786:31
     |
     |
1786 |     crate fn as_sym(&self) -> Symbol {

error[E0412]: cannot find type `TyCtxt` in this scope
    --> src/librustdoc/clean/types.rs:1824:39
     |
     |
1824 |     crate fn primitive_locations(tcx: TyCtxt<'_>) -> &FxHashMap<PrimitiveType, DefId> {


error[E0412]: cannot find type `FxHashMap` in this scope
    --> src/librustdoc/clean/types.rs:1824:55
     |
1824 |     crate fn primitive_locations(tcx: TyCtxt<'_>) -> &FxHashMap<PrimitiveType, DefId> {

error[E0412]: cannot find type `DefId` in this scope
    --> src/librustdoc/clean/types.rs:1824:80
     |
     |
1824 |     crate fn primitive_locations(tcx: TyCtxt<'_>) -> &FxHashMap<PrimitiveType, DefId> {
     |
     |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
     |
1824 |     crate fn primitive_locations(tcx: TyCtxt<'_>) -> &FxHashMap<PrimitiveType, crate::clean::ItemId> {
     |                                                                                ~~~~~~~~~~~~~~~~~~~~
1824 |     crate fn primitive_locations(tcx: TyCtxt<'_>) -> &FxHashMap<PrimitiveType, crate::core::ImplTraitParam> {
help: you might be missing a type parameter
     |
     |
1669 | impl<DefId> PrimitiveType {


error[E0412]: cannot find type `FxHashMap` in this scope
    --> src/librustdoc/clean/types.rs:1825:46
     |
1825 |         static PRIMITIVE_LOCATIONS: OnceCell<FxHashMap<PrimitiveType, DefId>> = OnceCell::new();

error[E0412]: cannot find type `DefId` in this scope
    --> src/librustdoc/clean/types.rs:1825:71
     |
     |
1825 |         static PRIMITIVE_LOCATIONS: OnceCell<FxHashMap<PrimitiveType, DefId>> = OnceCell::new();
     |
     |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
     |
1825 |         static PRIMITIVE_LOCATIONS: OnceCell<FxHashMap<PrimitiveType, crate::clean::ItemId>> = OnceCell::new();
     |                                                                       ~~~~~~~~~~~~~~~~~~~~
1825 |         static PRIMITIVE_LOCATIONS: OnceCell<FxHashMap<PrimitiveType, crate::core::ImplTraitParam>> = OnceCell::new();

error[E0412]: cannot find type `DefId` in this scope
    --> src/librustdoc/clean/types.rs:1943:16
     |
     |
1943 |     Restricted(DefId),
     |                ^^^^^ not found in this scope
     |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
     |
1943 |     Restricted(crate::clean::ItemId),
     |                ~~~~~~~~~~~~~~~~~~~~
1943 |     Restricted(crate::core::ImplTraitParam),

error[E0412]: cannot find type `CtorKind` in this scope
    --> src/librustdoc/clean/types.rs:1954:24
     |
     |
1954 |     crate struct_type: CtorKind,

error[E0412]: cannot find type `CtorKind` in this scope
    --> src/librustdoc/clean/types.rs:1972:24
     |
     |
1972 |     crate struct_type: CtorKind,

error[E0412]: cannot find type `IndexVec` in this scope
    --> src/librustdoc/clean/types.rs:1979:21
     |
     |
1979 |     crate variants: IndexVec<VariantIdx, Item>,

error[E0412]: cannot find type `VariantIdx` in this scope
    --> src/librustdoc/clean/types.rs:1979:30
     |
     |
1979 |     crate variants: IndexVec<VariantIdx, Item>,
...
1985 | crate enum Variant {
     | ------------------ similarly named enum `Variant` defined here
     |
     |
help: an enum with a similar name exists
     |
1979 |     crate variants: IndexVec<Variant, Item>,
help: you might be missing a type parameter
     |
     |
1978 | crate struct Enum<VariantIdx> {

error[E0412]: cannot find type `Session` in this scope
    --> src/librustdoc/clean/types.rs:2017:37
     |
---

error[E0412]: cannot find type `Session` in this scope
    --> src/librustdoc/clean/types.rs:2021:31
     |
2021 |     crate fn lo(&self, sess: &Session) -> Loc {

error[E0412]: cannot find type `Loc` in this scope
    --> src/librustdoc/clean/types.rs:2021:43
     |
     |
2021 |     crate fn lo(&self, sess: &Session) -> Loc {

error[E0412]: cannot find type `Session` in this scope
    --> src/librustdoc/clean/types.rs:2025:31
     |
     |
2025 |     crate fn hi(&self, sess: &Session) -> Loc {

error[E0412]: cannot find type `Loc` in this scope
    --> src/librustdoc/clean/types.rs:2025:43
     |
     |
2025 |     crate fn hi(&self, sess: &Session) -> Loc {

error[E0412]: cannot find type `Session` in this scope
    --> src/librustdoc/clean/types.rs:2029:33
     |
     |
2029 |     crate fn cnum(&self, sess: &Session) -> CrateNum {

error[E0412]: cannot find type `CrateNum` in this scope
    --> src/librustdoc/clean/types.rs:2029:45
     |
     |
2029 |     crate fn cnum(&self, sess: &Session) -> CrateNum {

error[E0412]: cannot find type `Res` in this scope
    --> src/librustdoc/clean/types.rs:2037:16
     |
     |
2037 |     crate res: Res,
     |                ^^^ not found in this scope
     |
note: enum `crate::collect_intra_doc_links::Res` exists but is inaccessible
     |
62   | enum Res {
     | ^^^^^^^^ not accessible


error[E0412]: cannot find type `DefId` in this scope
    --> src/librustdoc/clean/types.rs:2042:31
     |
2042 |     crate fn def_id(&self) -> DefId {
     |
     |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
     |
2042 |     crate fn def_id(&self) -> crate::clean::ItemId {
     |                               ~~~~~~~~~~~~~~~~~~~~
2042 |     crate fn def_id(&self) -> crate::core::ImplTraitParam {

error[E0412]: cannot find type `Symbol` in this scope
    --> src/librustdoc/clean/types.rs:2046:29
     |
     |
2046 |     crate fn last(&self) -> Symbol {

error[E0412]: cannot find type `ThinVec` in this scope
    --> src/librustdoc/clean/types.rs:2111:55
     |
     |
2111 |     AngleBracketed { args: Vec<GenericArg>, bindings: ThinVec<TypeBinding> },

error[E0412]: cannot find type `Symbol` in this scope
    --> src/librustdoc/clean/types.rs:2122:17
     |
---
     |
2161 |     crate mutability: Mutability,
     |                       ^^^^^^^^^^ not found in this scope

error[E0412]: cannot find type `BodyId` in this scope
    --> src/librustdoc/clean/types.rs:2162:24
2159 | crate struct Static {
2159 | crate struct Static {
     |                    - help: you might be missing a type parameter: `<BodyId>`
...
2162 |     crate expr: Option<BodyId>,


error[E0412]: cannot find type `BodyId` in this scope
    --> src/librustdoc/clean/types.rs:2200:23
     |
2200 |     Anonymous { body: BodyId },

error[E0412]: cannot find type `DefId` in this scope
    --> src/librustdoc/clean/types.rs:2202:22
     |
     |
2202 |     Extern { def_id: DefId },
     |
     |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
     |
2202 |     Extern { def_id: crate::clean::ItemId },
     |                      ~~~~~~~~~~~~~~~~~~~~
2202 |     Extern { def_id: crate::core::ImplTraitParam },

error[E0412]: cannot find type `DefId` in this scope
    --> src/librustdoc/clean/types.rs:2204:21
     |
     |
2204 |     Local { def_id: DefId, body: BodyId },
     |
     |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
     |
2204 |     Local { def_id: crate::clean::ItemId, body: BodyId },
     |                     ~~~~~~~~~~~~~~~~~~~~
2204 |     Local { def_id: crate::core::ImplTraitParam, body: BodyId },


error[E0412]: cannot find type `BodyId` in this scope
    --> src/librustdoc/clean/types.rs:2204:34
     |
2204 |     Local { def_id: DefId, body: BodyId },

error[E0412]: cannot find type `TyCtxt` in this scope
    --> src/librustdoc/clean/types.rs:2208:31
     |
     |
2208 |     crate fn expr(&self, tcx: TyCtxt<'_>) -> String {

error[E0412]: cannot find type `TyCtxt` in this scope
    --> src/librustdoc/clean/types.rs:2212:32
     |
     |
2212 |     crate fn value(&self, tcx: TyCtxt<'_>) -> Option<String> {

error[E0412]: cannot find type `TyCtxt` in this scope
    --> src/librustdoc/clean/types.rs:2216:37
     |
     |
2216 |     crate fn is_literal(&self, tcx: TyCtxt<'_>) -> bool {

error[E0412]: cannot find type `TyCtxt` in this scope
    --> src/librustdoc/clean/types.rs:2222:31
     |
     |
2222 |     crate fn expr(&self, tcx: TyCtxt<'_>) -> String {

error[E0412]: cannot find type `TyCtxt` in this scope
    --> src/librustdoc/clean/types.rs:2232:32
     |
     |
2232 |     crate fn value(&self, tcx: TyCtxt<'_>) -> Option<String> {

error[E0412]: cannot find type `TyCtxt` in this scope
    --> src/librustdoc/clean/types.rs:2241:37
     |
     |
2241 |     crate fn is_literal(&self, tcx: TyCtxt<'_>) -> bool {

error[E0412]: cannot find type `TyCtxt` in this scope
    --> src/librustdoc/clean/types.rs:2266:49
     |
     |
2266 |     crate fn provided_trait_methods(&self, tcx: TyCtxt<'_>) -> FxHashSet<Symbol> {


error[E0412]: cannot find type `FxHashSet` in this scope
    --> src/librustdoc/clean/types.rs:2266:64
     |
2266 |     crate fn provided_trait_methods(&self, tcx: TyCtxt<'_>) -> FxHashSet<Symbol> {

error[E0412]: cannot find type `Symbol` in this scope
    --> src/librustdoc/clean/types.rs:2266:74
     |
     |
2265 | impl Impl {
     |     - help: you might be missing a type parameter: `<Symbol>`
2266 |     crate fn provided_trait_methods(&self, tcx: TyCtxt<'_>) -> FxHashSet<Symbol> {

error[E0412]: cannot find type `Symbol` in this scope
    --> src/librustdoc/clean/types.rs:2307:31
     |
     |
2307 |     crate fn new_simple(name: Symbol, source: ImportSource, should_be_displayed: bool) -> Self {

error[E0412]: cannot find type `Symbol` in this scope
    --> src/librustdoc/clean/types.rs:2319:12
     |
     |
2319 |     Simple(Symbol),
     |            ^^^^^^ not found in this scope

error[E0412]: cannot find type `DefId` in this scope
    --> src/librustdoc/clean/types.rs:2327:23
     |
2327 |     crate did: Option<DefId>,
     |
     |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
     |
2327 |     crate did: Option<crate::clean::ItemId>,
     |                       ~~~~~~~~~~~~~~~~~~~~
2327 |     crate did: Option<crate::core::ImplTraitParam>,
help: you might be missing a type parameter
     |
     |
2325 | crate struct ImportSource<DefId> {

error[E0412]: cannot find type `MacroKind` in this scope
    --> src/librustdoc/clean/types.rs:2337:17
     |
     |
2337 |     crate kind: MacroKind,
     |
help: consider importing this enum
     |
1    | use rustdoc_json_types::MacroKind;
1    | use rustdoc_json_types::MacroKind;
     |

error[E0412]: cannot find type `Symbol` in this scope
    --> src/librustdoc/clean/types.rs:2338:24
     |
2336 | crate struct ProcMacro {
     |                       - help: you might be missing a type parameter: `<Symbol>`
2337 |     crate kind: MacroKind,
2338 |     crate helpers: Vec<Symbol>,

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/utils.rs:104:10
    |
    |
104 |     did: DefId,
    |          ^^^^^ not found in this scope
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
104 |     did: crate::clean::ItemId,
    |          ~~~~~~~~~~~~~~~~~~~~
104 |     did: crate::core::ImplTraitParam,


error[E0412]: cannot find type `SubstsRef` in this scope
   --> src/librustdoc/clean/utils.rs:107:13
    |
107 |     substs: SubstsRef<'_>,

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/utils.rs:132:10
    |
    |
132 |     did: DefId,
    |          ^^^^^ not found in this scope
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
132 |     did: crate::clean::ItemId,
    |          ~~~~~~~~~~~~~~~~~~~~
132 |     did: crate::core::ImplTraitParam,


error[E0412]: cannot find type `SubstsRef` in this scope
   --> src/librustdoc/clean/utils.rs:135:13
    |
135 |     substs: SubstsRef<'_>,

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/clean/utils.rs:199:45
    |
    |
199 | crate fn name_from_pat(p: &hir::Pat<'_>) -> Symbol {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/clean/utils.rs:262:37
    |
    |
262 | crate fn print_evaluated_const(tcx: TyCtxt<'_>, def_id: DefId) -> Option<String> {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/utils.rs:262:57
    |
    |
262 | crate fn print_evaluated_const(tcx: TyCtxt<'_>, def_id: DefId) -> Option<String> {
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
262 | crate fn print_evaluated_const(tcx: TyCtxt<'_>, def_id: crate::clean::ItemId) -> Option<String> {
    |                                                         ~~~~~~~~~~~~~~~~~~~~
262 | crate fn print_evaluated_const(tcx: TyCtxt<'_>, def_id: crate::core::ImplTraitParam) -> Option<String> {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/clean/utils.rs:303:46
    |
    |
303 | fn print_const_with_custom_print_scalar(tcx: TyCtxt<'_>, ct: ty::Const<'_>) -> String {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/clean/utils.rs:326:31
    |
    |
326 | crate fn is_literal_expr(tcx: TyCtxt<'_>, hir_id: hir::HirId) -> bool {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/clean/utils.rs:342:32
    |
    |
342 | crate fn print_const_expr(tcx: TyCtxt<'_>, body: hir::BodyId) -> String {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/utils.rs:372:18
    |
    |
372 |     item_def_id: DefId,
    |                  ^^^^^ not found in this scope
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
372 |     item_def_id: crate::clean::ItemId,
    |                  ~~~~~~~~~~~~~~~~~~~~
372 |     item_def_id: crate::core::ImplTraitParam,

error[E0412]: cannot find type `Res` in this scope
   --> src/librustdoc/clean/utils.rs:392:53
    |
    |
392 | crate fn register_res(cx: &mut DocContext<'_>, res: Res) -> DefId {
    |
    |
note: enum `crate::collect_intra_doc_links::Res` exists but is inaccessible
    |
62  | enum Res {
    | ^^^^^^^^ not accessible


error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/utils.rs:392:61
    |
392 | crate fn register_res(cx: &mut DocContext<'_>, res: Res) -> DefId {
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
392 | crate fn register_res(cx: &mut DocContext<'_>, res: Res) -> crate::clean::ItemId {
    |                                                             ~~~~~~~~~~~~~~~~~~~~
392 | crate fn register_res(cx: &mut DocContext<'_>, res: Res) -> crate::core::ImplTraitParam {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/clean/utils.rs:450:42
    |
    |
450 | crate fn find_nearest_parent_module(tcx: TyCtxt<'_>, def_id: DefId) -> Option<DefId> {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/utils.rs:450:62
    |
    |
450 | crate fn find_nearest_parent_module(tcx: TyCtxt<'_>, def_id: DefId) -> Option<DefId> {
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
450 | crate fn find_nearest_parent_module(tcx: TyCtxt<'_>, def_id: crate::clean::ItemId) -> Option<DefId> {
    |                                                              ~~~~~~~~~~~~~~~~~~~~
450 | crate fn find_nearest_parent_module(tcx: TyCtxt<'_>, def_id: crate::core::ImplTraitParam) -> Option<DefId> {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/utils.rs:450:79
    |
    |
450 | crate fn find_nearest_parent_module(tcx: TyCtxt<'_>, def_id: DefId) -> Option<DefId> {
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
450 | crate fn find_nearest_parent_module(tcx: TyCtxt<'_>, def_id: DefId) -> Option<crate::clean::ItemId> {
    |                                                                               ~~~~~~~~~~~~~~~~~~~~
450 | crate fn find_nearest_parent_module(tcx: TyCtxt<'_>, def_id: DefId) -> Option<crate::core::ImplTraitParam> {
help: you might be missing a type parameter
    |
    |
450 | crate fn find_nearest_parent_module<DefId>(tcx: TyCtxt<'_>, def_id: DefId) -> Option<DefId> {

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/clean/utils.rs:477:56
    |
    |
477 | crate fn has_doc_flag(attrs: ty::Attributes<'_>, flag: Symbol) -> bool {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/clean/utils.rs:493:10
    |
    |
493 |     tcx: TyCtxt<'_>,
    |          ^^^^^^ not found in this scope

error[E0412]: cannot find type `TokenTree` in this scope
   --> src/librustdoc/clean/utils.rs:494:40
    |
494 |     matchers: impl Iterator<Item = &'a TokenTree>,

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/clean/utils.rs:507:11
    |
---
    |
509 |     def_id: DefId,
    |             ^^^^^ not found in this scope
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
509 |     def_id: crate::clean::ItemId,
    |             ~~~~~~~~~~~~~~~~~~~~
509 |     def_id: crate::core::ImplTraitParam,

error[E0412]: cannot find type `Ty` in this scope
   --> src/librustdoc/clean/mod.rs:338:68
    |
    |
338 | impl<'tcx> Clean<Option<WherePredicate>> for ty::OutlivesPredicate<Ty<'tcx>, ty::Region<'tcx>> {

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/clean/mod.rs:752:16
    |
    |
752 |     name: &mut Symbol,
    |                ^^^^^^ not found in this scope

error[E0412]: cannot find type `Ident` in this scope
   --> src/librustdoc/clean/mod.rs:850:14
    |
850 |     names: &[Ident],
    |
    |
help: there is an enum variant `crate::html::highlight::Class::Ident`; try using the variant's enum
    |
850 |     names: &[crate::html::highlight::Class],

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/clean/mod.rs:897:17
    |
    |
897 |     did: Option<DefId>,
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
897 |     did: Option<crate::clean::ItemId>,
    |                 ~~~~~~~~~~~~~~~~~~~~
897 |     did: Option<crate::core::ImplTraitParam>,
help: you might be missing a type parameter
    |
    |
895 | fn clean_fn_decl_from_did_and_sig<DefId>(

error[E0412]: cannot find type `Ty` in this scope
    --> src/librustdoc/clean/mod.rs:1453:51
     |
     |
1453 | fn normalize<'tcx>(cx: &mut DocContext<'tcx>, ty: Ty<'_>) -> Option<Ty<'tcx>> {

error[E0412]: cannot find type `Ty` in this scope
    --> src/librustdoc/clean/mod.rs:1453:69
     |
     |
1453 | fn normalize<'tcx>(cx: &mut DocContext<'tcx>, ty: Ty<'_>) -> Option<Ty<'tcx>> {

error[E0412]: cannot find type `Ty` in this scope
    --> src/librustdoc/clean/mod.rs:1483:28
     |
     |
1483 | impl<'tcx> Clean<Type> for Ty<'tcx> {

error[E0412]: cannot find type `DefId` in this scope
    --> src/librustdoc/clean/mod.rs:1699:24
     |
     |
1699 | fn clean_field(def_id: DefId, name: Symbol, ty: Type, cx: &mut DocContext<'_>) -> Item {
     |
     |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
     |
1699 | fn clean_field(def_id: crate::clean::ItemId, name: Symbol, ty: Type, cx: &mut DocContext<'_>) -> Item {
     |                        ~~~~~~~~~~~~~~~~~~~~
1699 | fn clean_field(def_id: crate::core::ImplTraitParam, name: Symbol, ty: Type, cx: &mut DocContext<'_>) -> Item {

error[E0412]: cannot find type `Symbol` in this scope
    --> src/librustdoc/clean/mod.rs:1699:37
     |
     |
1699 | fn clean_field(def_id: DefId, name: Symbol, ty: Type, cx: &mut DocContext<'_>) -> Item {

error[E0412]: cannot find type `TyCtxt` in this scope
    --> src/librustdoc/clean/mod.rs:1710:32
     |
     |
1710 | fn is_field_vis_inherited(tcx: TyCtxt<'_>, def_id: DefId) -> bool {

error[E0412]: cannot find type `DefId` in this scope
    --> src/librustdoc/clean/mod.rs:1710:52
     |
     |
1710 | fn is_field_vis_inherited(tcx: TyCtxt<'_>, def_id: DefId) -> bool {
     |
     |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
     |
1710 | fn is_field_vis_inherited(tcx: TyCtxt<'_>, def_id: crate::clean::ItemId) -> bool {
     |                                                    ~~~~~~~~~~~~~~~~~~~~
1710 | fn is_field_vis_inherited(tcx: TyCtxt<'_>, def_id: crate::core::ImplTraitParam) -> bool {

error[E0412]: cannot find type `Symbol` in this scope
    --> src/librustdoc/clean/mod.rs:1838:21
     |
     |
1835 | fn clean_maybe_renamed_item(
     |                            - help: you might be missing a type parameter: `<Symbol>`
...
1838 |     renamed: Option<Symbol>,

error[E0412]: cannot find type `Symbol` in this scope
    --> src/librustdoc/clean/mod.rs:1971:11
     |
     |
1971 |     name: Symbol,
     |           ^^^^^^ not found in this scope

error[E0412]: cannot find type `Symbol` in this scope
    --> src/librustdoc/clean/mod.rs:1972:23
     |
1969 | fn clean_extern_crate(
     |                      - help: you might be missing a type parameter: `<Symbol>`
...
1972 |     orig_name: Option<Symbol>,

error[E0412]: cannot find type `Symbol` in this scope
    --> src/librustdoc/clean/mod.rs:2021:11
     |
     |
2021 |     name: Symbol,
     |           ^^^^^^ not found in this scope

error[E0412]: cannot find type `Symbol` in this scope
    --> src/librustdoc/clean/mod.rs:2135:21
     |
2132 | fn clean_maybe_renamed_foreign_item(
     |                                    - help: you might be missing a type parameter: `<Symbol>`
...
2135 |     renamed: Option<Symbol>,


error[E0412]: cannot find type `ErrorOutputType` in this scope
   |
   |
72 |     crate error_format: ErrorOutputType,


error[E0412]: cannot find type `SearchPath` in this scope
   |
63 | crate struct Options {
63 | crate struct Options {
   |                     - help: you might be missing a type parameter: `<SearchPath>`
...
74 |     crate libs: Vec<SearchPath>,

error[E0412]: cannot find type `Externs` in this scope
   --> src/librustdoc/config.rs:78:20
    |
    |
78  |     crate externs: Externs,
    |                    ^^^^^^^ help: a trait with a similar name exists: `Extend`
    |
   ::: /checkout/library/core/src/iter/traits/collect.rs:351:1
    |
351 | pub trait Extend<A> {
    | ------------------- similarly named trait `Extend` defined here
error[E0412]: cannot find type `CodegenOptions` in this scope
  --> src/librustdoc/config.rs:86:28
   |
86 |     crate codegen_options: CodegenOptions,
---
   |
90 |     crate debugging_opts: DebuggingOptions,
   |                           ^^^^^^^^^^^^^^^^ not found in this scope

error[E0412]: cannot find type `TargetTriple` in this scope
   |
94 |     crate target: TargetTriple,
   |                   ^^^^^^^^^^^^ not found in this scope


error[E0412]: cannot find type `Edition` in this scope
  --> src/librustdoc/config.rs:97:20
   |
97 |     crate edition: Edition,
   |                    ^^^^^^^ not found in this scope

error[E0412]: cannot find type `Level` in this scope
    |
    |
101 |     crate lint_opts: Vec<(String, Level)>,
    |
help: consider importing this struct
    |
1   | use tracing::Level;
1   | use tracing::Level;
    |

error[E0412]: cannot find type `Level` in this scope
    |
    |
105 |     crate lint_cap: Option<Level>,
    |
help: consider importing this struct
    |
1   | use tracing::Level;
1   | use tracing::Level;
    |

error[E0412]: cannot find type `Externs` in this scope
   --> src/librustdoc/config.rs:161:35
    |
161 |         struct FmtExterns<'a>(&'a Externs);
    |
   ::: /checkout/library/core/src/iter/traits/collect.rs:351:1
    |
    |
351 | pub trait Extend<A> {
    | ------------------- similarly named trait `Extend` defined here

error[E0412]: cannot find type `FxHashMap` in this scope
    |
235 |     crate default_settings: FxHashMap<String, String>,
    |                             ^^^^^^^^^ not found in this scope


error[E0412]: cannot find type `DefIdMap` in this scope
  --> src/librustdoc/core.rs:39:28
   |
39 |     crate traits_in_scope: DefIdMap<Vec<TraitCandidate>>,

error[E0412]: cannot find type `TraitCandidate` in this scope
  --> src/librustdoc/core.rs:39:41
   |
   |
36 | crate struct ResolverCaches {
   |                            - help: you might be missing a type parameter: `<TraitCandidate>`
...
39 |     crate traits_in_scope: DefIdMap<Vec<TraitCandidate>>,

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/core.rs:40:34
   |
   |
40 |     crate all_traits: Option<Vec<DefId>>,
   |
   |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
   |
40 |     crate all_traits: Option<Vec<crate::core::ImplTraitParam>>,
   |                                  ~~~~~~~~~~~~~~~~~~~~~~~~~~~
40 |     crate all_traits: Option<Vec<crate::core::ItemId>>,
help: you might be missing a type parameter
   |
   |
36 | crate struct ResolverCaches<DefId> {

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/core.rs:41:39
   |
   |
41 |     crate all_trait_impls: Option<Vec<DefId>>,
   |
   |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
   |
41 |     crate all_trait_impls: Option<Vec<crate::core::ImplTraitParam>>,
   |                                       ~~~~~~~~~~~~~~~~~~~~~~~~~~~
41 |     crate all_trait_impls: Option<Vec<crate::core::ItemId>>,
help: you might be missing a type parameter
   |
   |
36 | crate struct ResolverCaches<DefId> {


error[E0412]: cannot find type `FxHashMap` in this scope
   |
   |
42 |     crate all_macro_rules: FxHashMap<Symbol, Res<NodeId>>,

error[E0412]: cannot find type `Symbol` in this scope
  --> src/librustdoc/core.rs:42:38
   |
   |
36 | crate struct ResolverCaches {
   |                            - help: you might be missing a type parameter: `<Symbol>`
...
42 |     crate all_macro_rules: FxHashMap<Symbol, Res<NodeId>>,

error[E0412]: cannot find type `Res` in this scope
  --> src/librustdoc/core.rs:42:46
   |
   |
42 |     crate all_macro_rules: FxHashMap<Symbol, Res<NodeId>>,
   |
   |
note: enum `crate::collect_intra_doc_links::Res` exists but is inaccessible
   |
62 | enum Res {
   | ^^^^^^^^ not accessible


error[E0412]: cannot find type `NodeId` in this scope
  --> src/librustdoc/core.rs:42:50
   |
36 | crate struct ResolverCaches {
   |                            - help: you might be missing a type parameter: `<NodeId>`
...
42 |     crate all_macro_rules: FxHashMap<Symbol, Res<NodeId>>,

error[E0412]: cannot find type `TyCtxt` in this scope
  --> src/librustdoc/core.rs:46:16
   |
   |
46 |     crate tcx: TyCtxt<'tcx>,


error[E0412]: cannot find type `ParamEnv` in this scope
   |
   |
57 |     crate param_env: ParamEnv<'tcx>,


error[E0412]: cannot find type `FxHashMap` in this scope
   |
   |
59 |     crate external_traits: Rc<RefCell<FxHashMap<DefId, clean::TraitWithExtraInfo>>>,

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/core.rs:59:49
   |
   |
59 |     crate external_traits: Rc<RefCell<FxHashMap<DefId, clean::TraitWithExtraInfo>>>,
   |
   |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
   |
59 |     crate external_traits: Rc<RefCell<FxHashMap<crate::core::ImplTraitParam, clean::TraitWithExtraInfo>>>,
   |                                                 ~~~~~~~~~~~~~~~~~~~~~~~~~~~
59 |     crate external_traits: Rc<RefCell<FxHashMap<crate::core::ItemId, clean::TraitWithExtraInfo>>>,
help: you might be missing a type parameter
   |
   |
45 | crate struct DocContext<'tcx, DefId> {


error[E0412]: cannot find type `FxHashSet` in this scope
   |
   |
62 |     crate active_extern_traits: FxHashSet<DefId>,

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/core.rs:62:43
   |
   |
62 |     crate active_extern_traits: FxHashSet<DefId>,
   |
   |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
   |
62 |     crate active_extern_traits: FxHashSet<crate::core::ImplTraitParam>,
   |                                           ~~~~~~~~~~~~~~~~~~~~~~~~~~~
62 |     crate active_extern_traits: FxHashSet<crate::core::ItemId>,
help: you might be missing a type parameter
   |
   |
45 | crate struct DocContext<'tcx, DefId> {


error[E0412]: cannot find type `FxHashMap` in this scope
   |
   |
66 |     crate substs: FxHashMap<DefId, clean::SubstParam>,

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/core.rs:66:29
   |
   |
66 |     crate substs: FxHashMap<DefId, clean::SubstParam>,
   |
   |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
   |
66 |     crate substs: FxHashMap<crate::core::ImplTraitParam, clean::SubstParam>,
   |                             ~~~~~~~~~~~~~~~~~~~~~~~~~~~
66 |     crate substs: FxHashMap<crate::core::ItemId, clean::SubstParam>,
help: you might be missing a type parameter
   |
   |
45 | crate struct DocContext<'tcx, DefId> {


error[E0412]: cannot find type `FxHashMap` in this scope
   |
   |
68 |     crate impl_trait_bounds: FxHashMap<ImplTraitParam, Vec<clean::GenericBound>>,


error[E0412]: cannot find type `FxHashSet` in this scope
   |
   |
71 |     crate generated_synthetics: FxHashSet<(Ty<'tcx>, DefId)>,

error[E0412]: cannot find type `Ty` in this scope
  --> src/librustdoc/core.rs:71:44
   |
   |
71 |     crate generated_synthetics: FxHashSet<(Ty<'tcx>, DefId)>,

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/core.rs:71:54
   |
   |
71 |     crate generated_synthetics: FxHashSet<(Ty<'tcx>, DefId)>,
   |
   |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
   |
71 |     crate generated_synthetics: FxHashSet<(Ty<'tcx>, crate::core::ImplTraitParam)>,
   |                                                      ~~~~~~~~~~~~~~~~~~~~~~~~~~~
71 |     crate generated_synthetics: FxHashSet<(Ty<'tcx>, crate::core::ItemId)>,
help: you might be missing a type parameter
   |
   |
45 | crate struct DocContext<'tcx, DefId> {

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/core.rs:72:28
   |
   |
72 |     crate auto_traits: Vec<DefId>,
   |
   |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
   |
72 |     crate auto_traits: Vec<crate::core::ImplTraitParam>,
   |                            ~~~~~~~~~~~~~~~~~~~~~~~~~~~
72 |     crate auto_traits: Vec<crate::core::ItemId>,
help: you might be missing a type parameter
   |
   |
45 | crate struct DocContext<'tcx, DefId> {


error[E0412]: cannot find type `FxHashSet` in this scope
   |
   |
78 |     crate inlined: FxHashSet<ItemId>,

error[E0412]: cannot find type `Session` in this scope
  --> src/librustdoc/core.rs:84:35
   |
   |
84 |     crate fn sess(&self) -> &'tcx Session {

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/core.rs:88:78
   |
   |
88 |     crate fn with_param_env<T, F: FnOnce(&mut Self) -> T>(&mut self, def_id: DefId, f: F) -> T {
   |
   |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
   |
88 |     crate fn with_param_env<T, F: FnOnce(&mut Self) -> T>(&mut self, def_id: crate::core::ImplTraitParam, f: F) -> T {
   |                                                                              ~~~~~~~~~~~~~~~~~~~~~~~~~~~
88 |     crate fn with_param_env<T, F: FnOnce(&mut Self) -> T>(&mut self, def_id: crate::core::ItemId, f: F) -> T {


error[E0412]: cannot find type `FxHashMap` in this scope
    |
    |
104 |     crate fn enter_alias<F, R>(&mut self, substs: FxHashMap<DefId, clean::SubstParam>, f: F) -> R

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/core.rs:104:61
    |
    |
104 |     crate fn enter_alias<F, R>(&mut self, substs: FxHashMap<DefId, clean::SubstParam>, f: F) -> R
    |
    |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
    |
104 |     crate fn enter_alias<F, R>(&mut self, substs: FxHashMap<crate::core::ImplTraitParam, clean::SubstParam>, f: F) -> R
    |                                                             ~~~~~~~~~~~~~~~~~~~~~~~~~~~
104 |     crate fn enter_alias<F, R>(&mut self, substs: FxHashMap<crate::core::ItemId, clean::SubstParam>, f: F) -> R
help: you might be missing a type parameter
    |
    |
83  | impl<'tcx, DefId> DocContext<'tcx> {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/core.rs:116:35
    |
    |
116 |     crate fn as_local_hir_id(tcx: TyCtxt<'_>, def_id: ItemId) -> Option<HirId> {

error[E0412]: cannot find type `HirId` in this scope
   --> src/librustdoc/core.rs:116:73
    |
    |
83  | impl<'tcx> DocContext<'tcx> {
    |          - help: you might be missing a type parameter: `, HirId`
...
116 |     crate fn as_local_hir_id(tcx: TyCtxt<'_>, def_id: ItemId) -> Option<HirId> {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/core.rs:126:69
    |
    |
126 |     crate fn with_all_traits(&mut self, f: impl FnOnce(&mut Self, &[DefId])) {
    |
    |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
    |
126 |     crate fn with_all_traits(&mut self, f: impl FnOnce(&mut Self, &[crate::core::ImplTraitParam])) {
    |                                                                     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
126 |     crate fn with_all_traits(&mut self, f: impl FnOnce(&mut Self, &[crate::core::ItemId])) {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/core.rs:132:74
    |
    |
132 |     crate fn with_all_trait_impls(&mut self, f: impl FnOnce(&mut Self, &[DefId])) {
    |
    |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
    |
132 |     crate fn with_all_trait_impls(&mut self, f: impl FnOnce(&mut Self, &[crate::core::ImplTraitParam])) {
    |                                                                          ~~~~~~~~~~~~~~~~~~~~~~~~~~~
132 |     crate fn with_all_trait_impls(&mut self, f: impl FnOnce(&mut Self, &[crate::core::ItemId])) {


error[E0412]: cannot find type `ErrorOutputType` in this scope
    |
    |
144 |     error_format: ErrorOutputType,


error[E0412]: cannot find type `Lrc` in this scope
    |
    |
145 |     source_map: Option<Lrc<source_map::SourceMap>>,

error[E0412]: cannot find type `DebuggingOptions` in this scope
   --> src/librustdoc/core.rs:146:22
    |
    |
146 |     debugging_opts: &DebuggingOptions,


error[E0412]: cannot find type `FxHashSet` in this scope
    |
    |
283 |                 static EMPTY_SET: SyncLazy<FxHashSet<LocalDefId>> =

error[E0412]: cannot find type `LocalDefId` in this scope
   --> src/librustdoc/core.rs:283:54
    |
    |
283 |                 static EMPTY_SET: SyncLazy<FxHashSet<LocalDefId>> =

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/core.rs:310:10
    |
    |
310 |     tcx: TyCtxt<'_>,
    |          ^^^^^^ not found in this scope

error[E0412]: cannot find type `Span` in this scope
   --> src/librustdoc/core.rs:403:77
    |
403 |     fn report_deprecated_attr(name: &str, diag: &rustc_errors::Handler, sp: Span) {
    |
help: consider importing one of these items
    |
1   | use crate::clean::Span;
---

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/core.rs:481:17
    |
481 |     fn new(tcx: TyCtxt<'tcx>) -> Self {

error[E0405]: cannot find trait `Visitor` in this scope
   --> src/librustdoc/core.rs:486:12
    |
    |
486 | impl<'tcx> Visitor<'tcx> for EmitIgnoredResolutionErrors<'tcx> {
    |
help: consider importing this trait
    |
1   | use serde::de::Visitor;
1   | use serde::de::Visitor;
    |

error[E0412]: cannot find type `Path` in this scope
   --> src/librustdoc/core.rs:495:42
    |
495 |     fn visit_path(&mut self, path: &'tcx Path<'_>, _id: HirId) {
    |
help: consider importing one of these items
    |
1   | use crate::clean::Path;
1   | use crate::clean::Path;
    |
1   | use std::path::Path;
    |

error[E0412]: cannot find type `HirId` in this scope
   --> src/librustdoc/core.rs:495:57
    |
495 |     fn visit_path(&mut self, path: &'tcx Path<'_>, _id: HirId) {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/core.rs:532:11
    |
    |
532 |     DefId(DefId),
    |           ^^^^^ not found in this scope
    |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
    |
532 |     DefId(crate::core::ImplTraitParam),
    |           ~~~~~~~~~~~~~~~~~~~~~~~~~~~
532 |     DefId(crate::core::ItemId),

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/core.rs:536:11
    |
    |
536 | impl From<DefId> for ImplTraitParam {
    |
    |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
    |
536 | impl From<crate::core::ImplTraitParam> for ImplTraitParam {
    |           ~~~~~~~~~~~~~~~~~~~~~~~~~~~
536 | impl From<crate::core::ItemId> for ImplTraitParam {
help: you might be missing a type parameter
    |
    |
536 | impl<DefId> From<DefId> for ImplTraitParam {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/core.rs:537:18
    |
    |
537 |     fn from(did: DefId) -> Self {
    |                  ^^^^^ not found in this scope
    |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
    |
537 |     fn from(did: crate::core::ImplTraitParam) -> Self {
    |                  ~~~~~~~~~~~~~~~~~~~~~~~~~~~
537 |     fn from(did: crate::core::ItemId) -> Self {

error[E0412]: cannot find type `ErrorGuaranteed` in this scope
  --> src/librustdoc/doctest.rs:50:53
   |
   |
50 | crate fn run(options: RustdocOptions) -> Result<(), ErrorGuaranteed> {
   |             -                                       ^^^^^^^^^^^^^^^ not found in this scope
   |             |
   |             help: you might be missing a type parameter: `<ErrorGuaranteed>`

error[E0412]: cannot find type `TargetTriple` in this scope
    |
302 |     target: TargetTriple,
    |             ^^^^^^^^^^^^ not found in this scope

---

error[E0412]: cannot find type `Edition` in this scope
   --> src/librustdoc/doctest.rs:720:53
    |
720 | fn check_if_attr_is_complete(source: &str, edition: Edition) -> bool {

error[E0412]: cannot find type `Edition` in this scope
   --> src/librustdoc/doctest.rs:752:39
    |
    |
752 | fn partition_source(s: &str, edition: Edition) -> (String, String, String) {

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/doctest.rs:879:17
    |
---
    |
1   | use tracing::Span;
    |

error[E0412]: cannot find type `Lrc` in this scope
    |
    |
882 |     source_map: Option<Lrc<SourceMap>>,
    |
   ::: /checkout/library/alloc/src/sync.rs:235:1
    |
    |
235 | pub struct Arc<T: ?Sized> {
    | ------------------------- similarly named struct `Arc` defined here
error[E0412]: cannot find type `SourceMap` in this scope
   --> src/librustdoc/doctest.rs:882:28
    |
851 | crate struct Collector {
851 | crate struct Collector {
    |                       - help: you might be missing a type parameter: `<SourceMap>`
...
882 |     source_map: Option<Lrc<SourceMap>>,


error[E0412]: cannot find type `FxHashMap` in this scope
    |
    |
884 |     visited_tests: FxHashMap<(String, usize), usize>,

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/doctest.rs:891:21
    |
    |
891 |         crate_name: Symbol,
    |                     ^^^^^^ not found in this scope

error[E0412]: cannot find type `Lrc` in this scope
    |
    |
895 |         source_map: Option<Lrc<SourceMap>>,
    |
   ::: /checkout/library/alloc/src/sync.rs:235:1
    |
    |
235 | pub struct Arc<T: ?Sized> {
    | ------------------------- similarly named struct `Arc` defined here
error[E0412]: cannot find type `SourceMap` in this scope
   --> src/librustdoc/doctest.rs:895:32
    |
889 | impl Collector {
889 | impl Collector {
    |     - help: you might be missing a type parameter: `<SourceMap>`
...
895 |         source_map: Option<Lrc<SourceMap>>,

error[E0412]: cannot find type `FileName` in this scope
   --> src/librustdoc/doctest.rs:916:53
    |
    |
916 |     fn generate_name(&self, line: usize, filename: &FileName) -> String {

error[E0412]: cannot find type `Span` in this scope
   --> src/librustdoc/doctest.rs:925:48
    |
    |
925 |     crate fn set_position(&mut self, position: Span) {
    |
help: consider importing one of these items
    |
1   | use crate::clean::Span;
---

error[E0412]: cannot find type `Map` in this scope
    --> src/librustdoc/doctest.rs:1163:10
     |
1163 |     map: Map<'hir>,
     |
help: consider importing one of these items
     |
1    | use core::iter::Map;
1    | use core::iter::Map;
     |
1    | use itertools::__std_iter::Map;
     |
1    | use rayon::iter::Map;
1    | use serde_json::Map;
     |
       and 1 other candidate

---

error[E0412]: cannot find type `HirId` in this scope
    --> src/librustdoc/doctest.rs:1172:17
     |
1172 |         hir_id: HirId,

error[E0412]: cannot find type `Span` in this scope
    --> src/librustdoc/doctest.rs:1173:13
     |
     |
1173 |         sp: Span,
     |
help: consider importing one of these items
     |
1    | use crate::clean::Span;
---
   |
32 |         edition: Edition,
   |                  ^^^^^^^ not found in this scope

error[E0412]: cannot find type `FxHashMap` in this scope
   |
   |
36 |     crate impls: FxHashMap<DefId, Vec<Impl>>,

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/formats/cache.rs:36:28
   |
   |
36 |     crate impls: FxHashMap<DefId, Vec<Impl>>,
   |
   |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
   |
36 |     crate impls: FxHashMap<crate::core::ImplTraitParam, Vec<Impl>>,
   |                            ~~~~~~~~~~~~~~~~~~~~~~~~~~~
36 |     crate impls: FxHashMap<crate::formats::cache::ItemId, Vec<Impl>>,
help: you might be missing a type parameter
   |
   |
29 | crate struct Cache<DefId> {


error[E0412]: cannot find type `FxHashMap` in this scope
   |
   |
43 |     crate paths: FxHashMap<DefId, (Vec<Symbol>, ItemType)>,

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/formats/cache.rs:43:28
   |
   |
43 |     crate paths: FxHashMap<DefId, (Vec<Symbol>, ItemType)>,
   |
   |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
   |
43 |     crate paths: FxHashMap<crate::core::ImplTraitParam, (Vec<Symbol>, ItemType)>,
   |                            ~~~~~~~~~~~~~~~~~~~~~~~~~~~
43 |     crate paths: FxHashMap<crate::formats::cache::ItemId, (Vec<Symbol>, ItemType)>,
help: you might be missing a type parameter
   |
   |
29 | crate struct Cache<DefId> {

error[E0412]: cannot find type `Symbol` in this scope
  --> src/librustdoc/formats/cache.rs:43:40
   |
   |
29 | crate struct Cache {
   |                   - help: you might be missing a type parameter: `<Symbol>`
...
43 |     crate paths: FxHashMap<DefId, (Vec<Symbol>, ItemType)>,


error[E0412]: cannot find type `FxHashMap` in this scope
   |
   |
47 |     crate external_paths: FxHashMap<DefId, (Vec<Symbol>, ItemType)>,

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/formats/cache.rs:47:37
   |
   |
47 |     crate external_paths: FxHashMap<DefId, (Vec<Symbol>, ItemType)>,
   |
   |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
   |
47 |     crate external_paths: FxHashMap<crate::core::ImplTraitParam, (Vec<Symbol>, ItemType)>,
   |                                     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
47 |     crate external_paths: FxHashMap<crate::formats::cache::ItemId, (Vec<Symbol>, ItemType)>,
help: you might be missing a type parameter
   |
   |
29 | crate struct Cache<DefId> {

error[E0412]: cannot find type `Symbol` in this scope
  --> src/librustdoc/formats/cache.rs:47:49
   |
   |
29 | crate struct Cache {
   |                   - help: you might be missing a type parameter: `<Symbol>`
...
47 |     crate external_paths: FxHashMap<DefId, (Vec<Symbol>, ItemType)>,


error[E0412]: cannot find type `FxHashMap` in this scope
   |
   |
59 |     crate exact_paths: FxHashMap<DefId, Vec<Symbol>>,

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/formats/cache.rs:59:34
   |
   |
59 |     crate exact_paths: FxHashMap<DefId, Vec<Symbol>>,
   |
   |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
   |
59 |     crate exact_paths: FxHashMap<crate::core::ImplTraitParam, Vec<Symbol>>,
   |                                  ~~~~~~~~~~~~~~~~~~~~~~~~~~~
59 |     crate exact_paths: FxHashMap<crate::formats::cache::ItemId, Vec<Symbol>>,
help: you might be missing a type parameter
   |
   |
29 | crate struct Cache<DefId> {

error[E0412]: cannot find type `Symbol` in this scope
  --> src/librustdoc/formats/cache.rs:59:45
   |
   |
29 | crate struct Cache {
   |                   - help: you might be missing a type parameter: `<Symbol>`
...
59 |     crate exact_paths: FxHashMap<DefId, Vec<Symbol>>,


error[E0412]: cannot find type `FxHashMap` in this scope
   |
   |
65 |     crate traits: FxHashMap<DefId, clean::TraitWithExtraInfo>,

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/formats/cache.rs:65:29
   |
   |
65 |     crate traits: FxHashMap<DefId, clean::TraitWithExtraInfo>,
   |
   |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
   |
65 |     crate traits: FxHashMap<crate::core::ImplTraitParam, clean::TraitWithExtraInfo>,
   |                             ~~~~~~~~~~~~~~~~~~~~~~~~~~~
65 |     crate traits: FxHashMap<crate::formats::cache::ItemId, clean::TraitWithExtraInfo>,
help: you might be missing a type parameter
   |
   |
29 | crate struct Cache<DefId> {


error[E0412]: cannot find type `FxHashMap` in this scope
   |
   |
70 |     crate implementors: FxHashMap<DefId, Vec<Impl>>,

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/formats/cache.rs:70:35
   |
   |
70 |     crate implementors: FxHashMap<DefId, Vec<Impl>>,
   |
   |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
   |
70 |     crate implementors: FxHashMap<crate::core::ImplTraitParam, Vec<Impl>>,
   |                                   ~~~~~~~~~~~~~~~~~~~~~~~~~~~
70 |     crate implementors: FxHashMap<crate::formats::cache::ItemId, Vec<Impl>>,
help: you might be missing a type parameter
   |
   |
29 | crate struct Cache<DefId> {


error[E0412]: cannot find type `FxHashMap` in this scope
   |
   |
73 |     crate extern_locations: FxHashMap<CrateNum, ExternalLocation>,

error[E0412]: cannot find type `CrateNum` in this scope
  --> src/librustdoc/formats/cache.rs:73:39
   |
   |
29 | crate struct Cache {
   |                   - help: you might be missing a type parameter: `<CrateNum>`
...
73 |     crate extern_locations: FxHashMap<CrateNum, ExternalLocation>,


error[E0412]: cannot find type `FxHashMap` in this scope
   |
   |
76 |     crate primitive_locations: FxHashMap<clean::PrimitiveType, DefId>,

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/formats/cache.rs:76:64
   |
   |
76 |     crate primitive_locations: FxHashMap<clean::PrimitiveType, DefId>,
   |
   |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
   |
76 |     crate primitive_locations: FxHashMap<clean::PrimitiveType, crate::core::ImplTraitParam>,
   |                                                                ~~~~~~~~~~~~~~~~~~~~~~~~~~~
76 |     crate primitive_locations: FxHashMap<clean::PrimitiveType, crate::formats::cache::ItemId>,
help: you might be missing a type parameter
   |
   |
29 | crate struct Cache<DefId> {


error[E0412]: cannot find type `AccessLevels` in this scope
   |
   |
81 |     crate access_levels: AccessLevels<DefId>,

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/formats/cache.rs:81:39
   |
   |
81 |     crate access_levels: AccessLevels<DefId>,
   |
   |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
   |
81 |     crate access_levels: AccessLevels<crate::core::ImplTraitParam>,
   |                                       ~~~~~~~~~~~~~~~~~~~~~~~~~~~
81 |     crate access_levels: AccessLevels<crate::formats::cache::ItemId>,
help: you might be missing a type parameter
   |
   |
29 | crate struct Cache<DefId> {


error[E0412]: cannot find type `FxHashSet` in this scope
   |
   |
93 |     crate masked_crates: FxHashSet<CrateNum>,

error[E0412]: cannot find type `CrateNum` in this scope
  --> src/librustdoc/formats/cache.rs:93:36
   |
   |
29 | crate struct Cache {
   |                   - help: you might be missing a type parameter: `<CrateNum>`
...
93 |     crate masked_crates: FxHashSet<CrateNum>,

error[E0412]: cannot find type `Symbol` in this scope
  --> src/librustdoc/formats/cache.rs:96:16
   |
   |
29 | crate struct Cache {
   |                   - help: you might be missing a type parameter: `<Symbol>`
96 |     stack: Vec<Symbol>,
   |                ^^^^^^ not found in this scope

error[E0412]: cannot find type `DefId` in this scope
error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/formats/cache.rs:97:23
   |
97 |     parent_stack: Vec<DefId>,
   |
   |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
   |
97 |     parent_stack: Vec<crate::core::ImplTraitParam>,
   |                       ~~~~~~~~~~~~~~~~~~~~~~~~~~~
97 |     parent_stack: Vec<crate::formats::cache::ItemId>,
help: you might be missing a type parameter
   |
   |
29 | crate struct Cache<DefId> {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/formats/cache.rs:108:35
    |
    |
108 |     crate orphan_impl_items: Vec<(DefId, clean::Item)>,
    |
    |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
    |
108 |     crate orphan_impl_items: Vec<(crate::core::ImplTraitParam, clean::Item)>,
    |                                   ~~~~~~~~~~~~~~~~~~~~~~~~~~~
108 |     crate orphan_impl_items: Vec<(crate::formats::cache::ItemId, clean::Item)>,
help: you might be missing a type parameter
    |
    |
29  | crate struct Cache<DefId> {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/formats/cache.rs:117:30
    |
    |
117 |     orphan_trait_impls: Vec<(DefId, FxHashSet<DefId>, Impl)>,
    |
    |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
    |
117 |     orphan_trait_impls: Vec<(crate::core::ImplTraitParam, FxHashSet<DefId>, Impl)>,
    |                              ~~~~~~~~~~~~~~~~~~~~~~~~~~~
117 |     orphan_trait_impls: Vec<(crate::formats::cache::ItemId, FxHashSet<DefId>, Impl)>,
help: you might be missing a type parameter
    |
    |
29  | crate struct Cache<DefId> {


error[E0412]: cannot find type `FxHashSet` in this scope
    |
    |
117 |     orphan_trait_impls: Vec<(DefId, FxHashSet<DefId>, Impl)>,

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/formats/cache.rs:117:47
    |
    |
117 |     orphan_trait_impls: Vec<(DefId, FxHashSet<DefId>, Impl)>,
    |
    |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
    |
117 |     orphan_trait_impls: Vec<(DefId, FxHashSet<crate::core::ImplTraitParam>, Impl)>,
    |                                               ~~~~~~~~~~~~~~~~~~~~~~~~~~~
117 |     orphan_trait_impls: Vec<(DefId, FxHashSet<crate::formats::cache::ItemId>, Impl)>,
help: you might be missing a type parameter
    |
    |
29  | crate struct Cache<DefId> {


error[E0412]: cannot find type `FxHashMap` in this scope
    |
    |
122 |     crate intra_doc_links: FxHashMap<ItemId, Vec<clean::ItemLink>>,


error[E0412]: cannot find type `FxHashSet` in this scope
    |
    |
124 |     crate hidden_cfg: FxHashSet<clean::cfg::Cfg>,

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/formats/cache.rs:130:10
    |
    |
130 |     tcx: TyCtxt<'tcx>,
    |          ^^^^^^ not found in this scope

error[E0412]: cannot find type `AccessLevels` in this scope
    |
    |
134 |     crate fn new(access_levels: AccessLevels<DefId>, document_private: bool) -> Self {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/formats/cache.rs:134:46
    |
    |
134 |     crate fn new(access_levels: AccessLevels<DefId>, document_private: bool) -> Self {
    |
    |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
    |
134 |     crate fn new(access_levels: AccessLevels<crate::core::ImplTraitParam>, document_private: bool) -> Self {
    |                                              ~~~~~~~~~~~~~~~~~~~~~~~~~~~
134 |     crate fn new(access_levels: AccessLevels<crate::formats::cache::ItemId>, document_private: bool) -> Self {
help: you might be missing a type parameter
    |
    |
133 | impl<DefId> Cache {

error[E0412]: cannot find type `DefKind` in this scope
   --> src/librustdoc/formats/item_type.rs:107:11
    |
    |
107 | impl From<DefKind> for ItemType {
    |     -     ^^^^^^^ not found in this scope
    |     |
    |     help: you might be missing a type parameter: `<DefKind>`
error[E0412]: cannot find type `DefKind` in this scope
   --> src/librustdoc/formats/item_type.rs:108:20
    |
    |
108 |     fn from(other: DefKind) -> Self {

error[E0412]: cannot find type `TyCtxt` in this scope
  --> src/librustdoc/formats/renderer.rs:27:14
   |
---

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/formats/mod.rs:40:41
   |
40 |     crate fn trait_did(&self) -> Option<DefId> {
   |
   |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
   |
40 |     crate fn trait_did(&self) -> Option<crate::clean::ItemId> {
   |                                         ~~~~~~~~~~~~~~~~~~~~
40 |     crate fn trait_did(&self) -> Option<crate::core::ImplTraitParam> {
help: you might be missing a type parameter
   |
   |
32 | impl<DefId> Impl {

error[E0405]: cannot find trait `Captures` in this scope
   --> src/librustdoc/html/format.rs:164:31
    |
    |
164 | ) -> impl fmt::Display + 'a + Captures<'tcx> {

error[E0405]: cannot find trait `Captures` in this scope
   --> src/librustdoc/html/format.rs:182:35
    |
    |
182 |     ) -> impl fmt::Display + 'a + Captures<'tcx> {

error[E0405]: cannot find trait `Captures` in this scope
   --> src/librustdoc/html/format.rs:245:35
    |
    |
245 |     ) -> impl fmt::Display + 'a + Captures<'tcx> {

error[E0405]: cannot find trait `Captures` in this scope
   --> src/librustdoc/html/format.rs:270:31
    |
    |
270 | ) -> impl fmt::Display + 'a + Captures<'tcx> {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/html/format.rs:388:32
    |
    |
388 |     crate fn print(&self, tcx: TyCtxt<'_>) -> impl fmt::Display + '_ {

error[E0405]: cannot find trait `Captures` in this scope
   --> src/librustdoc/html/format.rs:402:35
    |
    |
402 |     ) -> impl fmt::Display + 'a + Captures<'tcx> {

error[E0405]: cannot find trait `Captures` in this scope
   --> src/librustdoc/html/format.rs:432:35
    |
    |
432 |     ) -> impl fmt::Display + 'a + Captures<'tcx> {

error[E0405]: cannot find trait `Captures` in this scope
   --> src/librustdoc/html/format.rs:456:35
    |
    |
456 |     ) -> impl fmt::Display + 'a + Captures<'tcx> {

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/html/format.rs:553:41
    |
    |
553 | crate fn join_with_double_colon(syms: &[Symbol]) -> String {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/html/format.rs:564:10
    |
    |
564 |     did: DefId,
    |          ^^^^^ not found in this scope
    |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
    |
564 |     did: crate::core::ImplTraitParam,
    |          ~~~~~~~~~~~~~~~~~~~~~~~~~~~
564 |     did: crate::html::format::ItemId,

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/html/format.rs:567:36
    |
    |
563 | crate fn href_with_root_path(
    |                             - help: you might be missing a type parameter: `<Symbol>`
...
567 | ) -> Result<(String, ItemType, Vec<Symbol>), HrefError> {

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/html/format.rs:579:48
    |
    |
579 |     fn to_module_fqp(shortty: ItemType, fqp: &[Symbol]) -> &[Symbol] {

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/html/format.rs:579:62
    |
    |
579 |     fn to_module_fqp(shortty: ItemType, fqp: &[Symbol]) -> &[Symbol] {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/html/format.rs:643:20
    |
    |
643 | crate fn href(did: DefId, cx: &Context<'_>) -> Result<(String, ItemType, Vec<Symbol>), HrefError> {
    |
    |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
    |
643 | crate fn href(did: crate::core::ImplTraitParam, cx: &Context<'_>) -> Result<(String, ItemType, Vec<Symbol>), HrefError> {
    |                    ~~~~~~~~~~~~~~~~~~~~~~~~~~~
643 | crate fn href(did: crate::html::format::ItemId, cx: &Context<'_>) -> Result<(String, ItemType, Vec<Symbol>), HrefError> {

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/html/format.rs:643:78
    |
    |
643 | crate fn href(did: DefId, cx: &Context<'_>) -> Result<(String, ItemType, Vec<Symbol>), HrefError> {
    |              - help: you might be missing a type parameter: `<Symbol>`       ^^^^^^ not found in this scope
error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/html/format.rs:651:17
    |
    |
651 |     fqp: &'fqp [Symbol],

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/html/format.rs:652:24
    |
    |
652 |     relative_to_fqp: &[Symbol],

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/html/format.rs:653:30
    |
    |
650 | crate fn href_relative_parts<'fqp>(
    |                                  - help: you might be missing a type parameter: `, Symbol`
...
653 | ) -> Box<dyn Iterator<Item = Symbol> + 'fqp> {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/html/format.rs:680:10
    |
    |
680 |     did: DefId,
    |          ^^^^^ not found in this scope
    |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
    |
680 |     did: crate::core::ImplTraitParam,
    |          ~~~~~~~~~~~~~~~~~~~~~~~~~~~
680 |     did: crate::html::format::ItemId,

error[E0405]: cannot find trait `Captures` in this scope
   --> src/librustdoc/html/format.rs:779:31
    |
    |
779 | ) -> impl fmt::Display + 'a + Captures<'tcx> {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/html/format.rs:798:10
    |
    |
798 |     did: DefId,
    |          ^^^^^ not found in this scope
    |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
    |
798 |     did: crate::core::ImplTraitParam,
    |          ~~~~~~~~~~~~~~~~~~~~~~~~~~~
798 |     did: crate::html::format::ItemId,

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/html/format.rs:799:11
    |
    |
799 |     text: Symbol,
    |           ^^^^^^ not found in this scope

error[E0405]: cannot find trait `Captures` in this scope
    --> src/librustdoc/html/format.rs:1044:35
     |
1044 |     ) -> impl fmt::Display + 'b + Captures<'tcx> {

error[E0405]: cannot find trait `Captures` in this scope
    --> src/librustdoc/html/format.rs:1053:35
     |
     |
1053 |     ) -> impl fmt::Display + 'b + Captures<'tcx> {

error[E0405]: cannot find trait `Captures` in this scope
    --> src/librustdoc/html/format.rs:1063:35
     |
     |
1063 |     ) -> impl fmt::Display + 'a + Captures<'tcx> {

error[E0405]: cannot find trait `Captures` in this scope
    --> src/librustdoc/html/format.rs:1096:35
     |
     |
1096 |     ) -> impl fmt::Display + 'a + Captures<'tcx> {

error[E0405]: cannot find trait `Captures` in this scope
    --> src/librustdoc/html/format.rs:1120:35
     |
     |
1120 |     ) -> impl fmt::Display + 'a + Captures<'tcx> {

error[E0405]: cannot find trait `Captures` in this scope
    --> src/librustdoc/html/format.rs:1136:35
     |
     |
1136 |     ) -> impl fmt::Display + 'a + Captures<'tcx> {

error[E0405]: cannot find trait `Captures` in this scope
    --> src/librustdoc/html/format.rs:1155:35
     |
     |
1155 |     ) -> impl fmt::Display + 'b + Captures<'tcx> {

error[E0405]: cannot find trait `Captures` in this scope
    --> src/librustdoc/html/format.rs:1190:35
     |
     |
1190 |     ) -> impl fmt::Display + 'a + Captures<'tcx> {

error[E0405]: cannot find trait `Captures` in this scope
    --> src/librustdoc/html/format.rs:1305:35
     |
     |
1305 |     ) -> impl fmt::Display + 'a + Captures<'tcx> {

error[E0412]: cannot find type `TyCtxt` in this scope
    --> src/librustdoc/html/format.rs:1351:14
     |
---
     |
1352 |         item_did: DefId,
     |                   ^^^^^ not found in this scope
     |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
     |
1352 |         item_did: crate::core::ImplTraitParam,
     |                   ~~~~~~~~~~~~~~~~~~~~~~~~~~~
1352 |         item_did: crate::html::format::ItemId,

error[E0405]: cannot find trait `Captures` in this scope
    --> src/librustdoc/html/format.rs:1353:35
     |
     |
1353 |     ) -> impl fmt::Display + 'a + Captures<'tcx> {

error[E0412]: cannot find type `ConstStability` in this scope
    --> src/librustdoc/html/format.rs:1415:67
     |
     |
1415 | crate fn print_constness_with_space(c: &hir::Constness, s: Option<ConstStability>) -> &'static str {
     |                                    -                              ^^^^^^^^^^^^^^ not found in this scope
     |                                    |
     |                                    help: you might be missing a type parameter: `<ConstStability>`
error[E0405]: cannot find trait `Captures` in this scope
    --> src/librustdoc/html/format.rs:1432:35
     |
     |
1432 |     ) -> impl fmt::Display + 'a + Captures<'tcx> {

error[E0405]: cannot find trait `Captures` in this scope
    --> src/librustdoc/html/format.rs:1456:35
     |
     |
1456 |     ) -> impl fmt::Display + 'a + Captures<'tcx> {

error[E0405]: cannot find trait `Captures` in this scope
    --> src/librustdoc/html/format.rs:1479:35
     |
     |
1479 |     ) -> impl fmt::Display + 'a + Captures<'tcx> {

error[E0412]: cannot find type `Abi` in this scope
    --> src/librustdoc/html/format.rs:1510:36
     |
     |
1510 | crate fn print_abi_with_space(abi: Abi) -> impl fmt::Display {
     |
help: consider importing this enum
     |
8    | use rustdoc_json_types::Abi;
8    | use rustdoc_json_types::Abi;
     |

error[E0405]: cannot find trait `Captures` in this scope
    --> src/librustdoc/html/format.rs:1528:35
     |
1528 |     ) -> impl fmt::Display + 'a + Captures<'tcx> {

error[E0405]: cannot find trait `Captures` in this scope
    --> src/librustdoc/html/format.rs:1542:35
     |
     |
1542 |     ) -> impl fmt::Display + 'a + Captures<'tcx> {

error[E0412]: cannot find type `Span` in this scope
  --> src/librustdoc/html/highlight.rs:28:22
   |
---
   |
8  | use tracing::Span;
   |

error[E0412]: cannot find type `FxHashMap` in this scope
  --> src/librustdoc/html/highlight.rs:36:35
   |
36 | crate struct DecorationInfo(crate FxHashMap<&'static str, Vec<(u32, u32)>>);

error[E0412]: cannot find type `Edition` in this scope
  --> src/librustdoc/html/highlight.rs:44:29
   |
   |
39 | crate fn render_with_highlighting(
   |                                  - help: you might be missing a type parameter: `<Edition>`
...
44 |     tooltip: Option<(Option<Edition>, &str)>,

error[E0412]: cannot find type `Edition` in this scope
  --> src/librustdoc/html/highlight.rs:45:14
   |
---

error[E0412]: cannot find type `Span` in this scope
   --> src/librustdoc/html/highlight.rs:130:11
    |
130 |     Self_(Span),
    |
help: consider importing one of these items
    |
8   | use crate::clean::Span;
---

error[E0412]: cannot find type `Span` in this scope
   --> src/librustdoc/html/highlight.rs:172:33
    |
172 |     fn get_span(self) -> Option<Span> {
    |
help: consider importing one of these items
    |
8   | use crate::clean::Span;
---

error[E0412]: cannot find type `TokenKind` in this scope
   --> src/librustdoc/html/highlight.rs:191:18
    |
191 |     type Item = (TokenKind, &'a str);

error[E0412]: cannot find type `TokenKind` in this scope
   --> src/librustdoc/html/highlight.rs:192:35
    |
    |
190 | impl<'a> Iterator for TokenIter<'a> {
    |        - help: you might be missing a type parameter: `, TokenKind`
191 |     type Item = (TokenKind, &'a str);
192 |     fn next(&mut self) -> Option<(TokenKind, &'a str)> {

error[E0412]: cannot find type `Edition` in this scope
   --> src/librustdoc/html/highlight.rs:204:46
    |
    |
204 | fn get_real_ident_class(text: &str, edition: Edition, allow_path_keywords: bool) -> Option<Class> {

error[E0412]: cannot find type `TokenKind` in this scope
   --> src/librustdoc/html/highlight.rs:224:23
    |
    |
223 | struct PeekIter<'a> {
    |                   - help: you might be missing a type parameter: `, TokenKind`
224 |     stored: VecDeque<(TokenKind, &'a str)>,

error[E0412]: cannot find type `TokenKind` in this scope
   --> src/librustdoc/html/highlight.rs:235:36
    |
    |
230 | impl<'a> PeekIter<'a> {
    |        - help: you might be missing a type parameter: `, TokenKind`
...
235 |     fn peek(&mut self) -> Option<&(TokenKind, &'a str)> {

error[E0412]: cannot find type `TokenKind` in this scope
   --> src/librustdoc/html/highlight.rs:244:41
    |
    |
230 | impl<'a> PeekIter<'a> {
    |        - help: you might be missing a type parameter: `, TokenKind`
...
244 |     fn peek_next(&mut self) -> Option<&(TokenKind, &'a str)> {

error[E0412]: cannot find type `TokenKind` in this scope
   --> src/librustdoc/html/highlight.rs:258:18
    |
    |
258 |     type Item = (TokenKind, &'a str);

error[E0412]: cannot find type `Edition` in this scope
   --> src/librustdoc/html/highlight.rs:295:14
    |
---

error[E0412]: cannot find type `Span` in this scope
   --> src/librustdoc/html/highlight.rs:327:48
    |
327 |     fn new_span(&self, lo: u32, text: &str) -> Span {
    |
help: consider importing one of these items
    |
8   | use crate::clean::Span;
---

error[E0412]: cannot find type `TokenKind` in this scope
   --> src/librustdoc/html/highlight.rs:334:47
    |
302 | impl<'a> Classifier<'a> {
    |        - help: you might be missing a type parameter: `, TokenKind`
...
334 |     fn get_full_ident_path(&mut self) -> Vec<(TokenKind, usize, usize)> {

error[E0412]: cannot find type `TokenKind` in this scope
   --> src/librustdoc/html/highlight.rs:387:35
    |
    |
302 | impl<'a> Classifier<'a> {
    |        - help: you might be missing a type parameter: `, TokenKind`
...
387 |     fn next(&mut self) -> Option<(TokenKind, &'a str, u32)> {

error[E0412]: cannot find type `TokenKind` in this scope
   --> src/librustdoc/html/highlight.rs:449:16
    |
    |
449 |         token: TokenKind,
    |                ^^^^^^^^^ not found in this scope

error[E0412]: cannot find type `TokenKind` in this scope
   --> src/librustdoc/html/highlight.rs:644:34
    |
302 | impl<'a> Classifier<'a> {
    |        - help: you might be missing a type parameter: `, TokenKind`
...
644 |     fn peek(&mut self) -> Option<TokenKind> {


error[E0412]: cannot find type `FxHashMap` in this scope
  --> src/librustdoc/html/layout.rs:17:29
17 |     crate default_settings: FxHashMap<String, String>,
   |                             ^^^^^^^^^ not found in this scope

error[E0412]: cannot find type `Edition` in this scope
error[E0412]: cannot find type `Edition` in this scope
  --> src/librustdoc/html/markdown.rs:99:18
   |
99 |     pub edition: Edition,

error[E0412]: cannot find type `Edition` in this scope
   --> src/librustdoc/html/markdown.rs:110:11
    |
---
    |
223 |         edition: Edition,
    |                  ^^^^^^^ not found in this scope

error[E0412]: cannot find type `FxHashMap` in this scope
    |
    |
647 |     footnotes: FxHashMap<String, (Vec<Event<'a>>, u16)>,

error[E0412]: cannot find type `Span` in this scope
   --> src/librustdoc/html/markdown.rs:792:9
    |
    |
792 |     sp: Span,
    |
help: consider importing one of these items
    |
28  | use crate::clean::Span;
---

error[E0412]: cannot find type `HirId` in this scope
   --> src/librustdoc/html/markdown.rs:797:9
    |
797 |     Hir(HirId),

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/html/markdown.rs:798:9
    |
    |
798 |     Def(DefId),
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
798 |     Def(crate::clean::ItemId),
    |         ~~~~~~~~~~~~~~~~~~~~
798 |     Def(crate::core::ImplTraitParam),

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/html/markdown.rs:802:23
    |
    |
802 |     crate fn new(tcx: TyCtxt<'tcx>, hir_id: HirId, sp: Span) -> ExtraInfo<'tcx> {

error[E0412]: cannot find type `HirId` in this scope
   --> src/librustdoc/html/markdown.rs:802:45
    |
    |
802 |     crate fn new(tcx: TyCtxt<'tcx>, hir_id: HirId, sp: Span) -> ExtraInfo<'tcx> {

error[E0412]: cannot find type `Span` in this scope
   --> src/librustdoc/html/markdown.rs:802:56
    |
    |
802 |     crate fn new(tcx: TyCtxt<'tcx>, hir_id: HirId, sp: Span) -> ExtraInfo<'tcx> {
    |
help: consider importing one of these items
    |
28  | use crate::clean::Span;
---

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/html/markdown.rs:806:27
    |
806 |     crate fn new_did(tcx: TyCtxt<'tcx>, did: DefId, sp: Span) -> ExtraInfo<'tcx> {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/html/markdown.rs:806:46
    |
    |
806 |     crate fn new_did(tcx: TyCtxt<'tcx>, did: DefId, sp: Span) -> ExtraInfo<'tcx> {
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
806 |     crate fn new_did(tcx: TyCtxt<'tcx>, did: crate::clean::ItemId, sp: Span) -> ExtraInfo<'tcx> {
    |                                              ~~~~~~~~~~~~~~~~~~~~
806 |     crate fn new_did(tcx: TyCtxt<'tcx>, did: crate::core::ImplTraitParam, sp: Span) -> ExtraInfo<'tcx> {

error[E0412]: cannot find type `Span` in this scope
   --> src/librustdoc/html/markdown.rs:806:57
    |
    |
806 |     crate fn new_did(tcx: TyCtxt<'tcx>, did: DefId, sp: Span) -> ExtraInfo<'tcx> {
    |
help: consider importing one of these items
    |
28  | use crate::clean::Span;
---

error[E0412]: cannot find type `Edition` in this scope
   --> src/librustdoc/html/markdown.rs:846:27
    |
837 | crate struct LangString {
    |                        - help: you might be missing a type parameter: `<Edition>`
846 |     crate edition: Option<Edition>,
    |                           ^^^^^^^ not found in this scope


error[E0412]: cannot find type `FxHashMap` in this scope
     |
1418 |     map: FxHashMap<String, usize>,
     |          ^^^^^^^^^ not found in this scope


error[E0412]: cannot find type `FxHashMap` in this scope
     |
     |
1421 | fn init_id_map() -> FxHashMap<String, usize> {

error[E0412]: cannot find type `TyCtxt` in this scope
  --> src/librustdoc/html/render/search_index.rs:18:74
   |
   |
18 | crate fn build_index<'tcx>(krate: &clean::Crate, cache: &mut Cache, tcx: TyCtxt<'tcx>) -> String {

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/html/render/search_index.rs:106:31
    |
    |
103 |     struct CrateData<'a> {
    |                        - help: you might be missing a type parameter: `, Symbol`
...
106 |         paths: Vec<(ItemType, Symbol)>,

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/html/render/search_index.rs:190:10
    |
    |
190 |     tcx: TyCtxt<'tcx>,
    |          ^^^^^^ not found in this scope

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/html/render/search_index.rs:213:60
    |
213 | fn get_index_type_name(clean_type: &clean::Type) -> Option<Symbol> {
    |                       -                                    ^^^^^^ not found in this scope
    |                       |
    |                       help: you might be missing a type parameter: `<Symbol>`
error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/html/render/search_index.rs:249:10
    |
249 |     tcx: TyCtxt<'tcx>,
---

error[E0412]: cannot find type `Symbol` in this scope
  --> src/librustdoc/html/render/context.rs:48:29
   |
45 | crate struct Context<'tcx> {
   |                          - help: you might be missing a type parameter: `, Symbol`
...
48 |     pub(crate) current: Vec<Symbol>,


error[E0412]: cannot find type `FxHashMap` in this scope
  --> src/librustdoc/html/render/context.rs:58:38
   |
58 |     pub(super) deref_id_map: RefCell<FxHashMap<DefId, String>>,

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/html/render/context.rs:58:48
   |
   |
58 |     pub(super) deref_id_map: RefCell<FxHashMap<DefId, String>>,
   |
   |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
   |
58 |     pub(super) deref_id_map: RefCell<FxHashMap<crate::core::ImplTraitParam, String>>,
   |                                                ~~~~~~~~~~~~~~~~~~~~~~~~~~~
58 |     pub(super) deref_id_map: RefCell<FxHashMap<crate::html::render::ItemId, String>>,
help: you might be missing a type parameter
   |
   |
45 | crate struct Context<'tcx, DefId> {

error[E0412]: cannot find type `TyCtxt` in this scope
  --> src/librustdoc/html/render/context.rs:79:16
   |
   |
79 |     crate tcx: TyCtxt<'tcx>,


error[E0412]: cannot find type `FxHashMap` in this scope
  --> src/librustdoc/html/render/context.rs:87:26
87 |     crate local_sources: FxHashMap<PathBuf, String>,
   |                          ^^^^^^^^^ not found in this scope


error[E0412]: cannot find type `FxHashSet` in this scope
  --> src/librustdoc/html/render/context.rs:95:27
   |
95 |     created_dirs: RefCell<FxHashSet<PathBuf>>,


error[E0412]: cannot find type `FxHashMap` in this scope
   --> src/librustdoc/html/render/context.rs:118:34
    |
118 |     redirections: Option<RefCell<FxHashMap<String, String>>>,


error[E0412]: cannot find type `FxHashMap` in this scope
   --> src/librustdoc/html/render/context.rs:122:36
    |
122 |     crate span_correspondance_map: FxHashMap<rustc_span::Span, LinkFromSrc>,

error[E0412]: cannot find type `Edition` in this scope
   --> src/librustdoc/html/render/context.rs:140:32
    |
    |
140 |     crate fn edition(&self) -> Edition {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/html/render/context.rs:146:33
    |
    |
146 |     pub(crate) fn tcx(&self) -> TyCtxt<'tcx> {

error[E0412]: cannot find type `Session` in this scope
   --> src/librustdoc/html/render/context.rs:154:40
    |
---

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/html/render/print_item.rs:425:67
    |
425 | fn extra_info_tags(item: &clean::Item, parent: &clean::Item, tcx: TyCtxt<'_>) -> String {

error[E0412]: cannot find type `TyCtxt` in this scope
    --> src/librustdoc/html/render/print_item.rs:1481:10
     |
     |
1481 |     tcx: TyCtxt<'_>,
     |          ^^^^^^ not found in this scope

error[E0412]: cannot find type `FxHashMap` in this scope
    --> src/librustdoc/html/render/print_item.rs:1505:24
     |
1505 |     implementor_dups: &FxHashMap<Symbol, (DefId, bool)>,

error[E0412]: cannot find type `Symbol` in this scope
    --> src/librustdoc/html/render/print_item.rs:1505:34
     |
     |
1500 | fn render_implementor(
     |                      - help: you might be missing a type parameter: `<Symbol>`
...
1505 |     implementor_dups: &FxHashMap<Symbol, (DefId, bool)>,

error[E0412]: cannot find type `DefId` in this scope
    --> src/librustdoc/html/render/print_item.rs:1505:43
     |
     |
1505 |     implementor_dups: &FxHashMap<Symbol, (DefId, bool)>,
     |
     |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
     |
1505 |     implementor_dups: &FxHashMap<Symbol, (crate::core::ImplTraitParam, bool)>,
     |                                           ~~~~~~~~~~~~~~~~~~~~~~~~~~~
1505 |     implementor_dups: &FxHashMap<Symbol, (crate::html::render::ItemId, bool)>,
help: you might be missing a type parameter
     |
     |
1500 | fn render_implementor<DefId>(

error[E0412]: cannot find type `CtorKind` in this scope
    --> src/librustdoc/html/render/print_item.rs:1586:9
     |
     |
1586 |     ty: CtorKind,
     |         ^^^^^^^^ not found in this scope

error[E0412]: cannot find type `DefId` in this scope
    --> src/librustdoc/html/render/print_item.rs:1733:70
     |
1733 | fn document_type_layout(w: &mut Buffer, cx: &Context<'_>, ty_def_id: DefId) {
     |
     |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
     |
1733 | fn document_type_layout(w: &mut Buffer, cx: &Context<'_>, ty_def_id: crate::core::ImplTraitParam) {
     |                                                                      ~~~~~~~~~~~~~~~~~~~~~~~~~~~
1733 | fn document_type_layout(w: &mut Buffer, cx: &Context<'_>, ty_def_id: crate::html::render::ItemId) {

error[E0412]: cannot find type `Layout` in this scope
    --> src/librustdoc/html/render/print_item.rs:1734:53
     |
     |
1734 |     fn write_size_of_layout(w: &mut Buffer, layout: Layout<'_>, tag_size: u64) {
     |
help: consider importing one of these items
     |
1    | use core::alloc::Layout;
---
   |
25 |     External(DefId),
   |              ^^^^^ not found in this scope
   |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
   |
25 |     External(crate::core::ImplTraitParam),
   |              ~~~~~~~~~~~~~~~~~~~~~~~~~~~
25 |     External(crate::html::render::ItemId),

error[E0412]: cannot find type `TyCtxt` in this scope
  --> src/librustdoc/html/render/span_map.rs:40:10
   |
   |
40 |     tcx: TyCtxt<'_>,
   |          ^^^^^^ not found in this scope

error[E0412]: cannot find type `FxHashMap` in this scope
  --> src/librustdoc/html/render/span_map.rs:45:7
   |
45 | ) -> (FxHashMap<PathBuf, String>, FxHashMap<Span, LinkFromSrc>) {


error[E0412]: cannot find type `FxHashMap` in this scope
  --> src/librustdoc/html/render/span_map.rs:45:35
   |
45 | ) -> (FxHashMap<PathBuf, String>, FxHashMap<Span, LinkFromSrc>) {

error[E0412]: cannot find type `Span` in this scope
  --> src/librustdoc/html/render/span_map.rs:45:45
   |
   |
45 | ) -> (FxHashMap<PathBuf, String>, FxHashMap<Span, LinkFromSrc>) {
   |
help: consider importing one of these items
   |
1  | use crate::clean::Span;
---

error[E0412]: cannot find type `TyCtxt` in this scope
  --> src/librustdoc/html/render/span_map.rs:60:16
   |
60 |     crate tcx: TyCtxt<'tcx>,


error[E0412]: cannot find type `FxHashMap` in this scope
  --> src/librustdoc/html/render/span_map.rs:61:20
   |
61 |     crate matches: FxHashMap<Span, LinkFromSrc>,

error[E0412]: cannot find type `Span` in this scope
  --> src/librustdoc/html/render/span_map.rs:61:30
   |
   |
61 |     crate matches: FxHashMap<Span, LinkFromSrc>,
   |
help: consider importing one of these items
   |
1  | use crate::clean::Span;
---

error[E0412]: cannot find type `Span` in this scope
  --> src/librustdoc/html/render/span_map.rs:66:77
   |
66 |     fn handle_path(&mut self, path: &rustc_hir::Path<'_>, path_span: Option<Span>) {
   |
help: consider importing one of these items
   |
1  | use crate::clean::Span;
---

error[E0405]: cannot find trait `Visitor` in this scope
  --> src/librustdoc/html/render/span_map.rs:96:12
   |
96 | impl<'tcx> Visitor<'tcx> for SpanMapVisitor<'tcx> {
   |
help: consider importing this trait
   |
1  | use serde::de::Visitor;
1  | use serde::de::Visitor;
   |

error[E0412]: cannot find type `GenericParam` in this scope
   --> src/librustdoc/html/render/span_map.rs:103:48
    |
103 |     fn visit_generic_param(&mut self, p: &'tcx GenericParam<'tcx>) {

error[E0412]: cannot find type `HirId` in this scope
   --> src/librustdoc/html/render/span_map.rs:114:70
    |
    |
114 |     fn visit_path(&mut self, path: &'tcx rustc_hir::Path<'tcx>, _id: HirId) {

error[E0412]: cannot find type `Mod` in this scope
   --> src/librustdoc/html/render/span_map.rs:119:38
    |
    |
119 |     fn visit_mod(&mut self, m: &'tcx Mod<'tcx>, span: Span, id: HirId) {

error[E0412]: cannot find type `Span` in this scope
   --> src/librustdoc/html/render/span_map.rs:119:55
    |
    |
119 |     fn visit_mod(&mut self, m: &'tcx Mod<'tcx>, span: Span, id: HirId) {
    |
help: consider importing one of these items
    |
1   | use crate::clean::Span;
---

error[E0412]: cannot find type `HirId` in this scope
   --> src/librustdoc/html/render/span_map.rs:119:65
    |
119 |     fn visit_mod(&mut self, m: &'tcx Mod<'tcx>, span: Span, id: HirId) {

error[E0412]: cannot find type `HirId` in this scope
   --> src/librustdoc/html/render/span_map.rs:158:68
    |
    |
158 |     fn visit_use(&mut self, path: &'tcx rustc_hir::Path<'tcx>, id: HirId) {


error[E0412]: cannot find type `FxHashMap` in this scope
  --> src/librustdoc/html/render/write_shared.rs:22:32
   |
22 | static FILES_UNVERSIONED: Lazy<FxHashMap<&str, &[u8]>> = Lazy::new(|| {


error[E0412]: cannot find type `FxHashMap` in this scope
   --> src/librustdoc/html/render/write_shared.rs:357:19
    |
357 |         children: FxHashMap<OsString, Hierarchy>,


error[E0412]: cannot find type `FxHashSet` in this scope
   --> src/librustdoc/html/render/write_shared.rs:358:16
    |
358 |         elems: FxHashSet<OsString>,

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/html/render/mod.rs:101:26
    |
    |
101 |     crate parent: Option<DefId>,
    |
    |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
    |
101 |     crate parent: Option<crate::core::ImplTraitParam>,
    |                          ~~~~~~~~~~~~~~~~~~~~~~~~~~~
101 |     crate parent: Option<crate::html::render::ItemId>,
help: you might be missing a type parameter
    |
    |
96  | crate struct IndexItem<DefId> {

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/html/render/mod.rs:104:25
    |
    |
96  | crate struct IndexItem {
    |                       - help: you might be missing a type parameter: `<Symbol>`
...
104 |     crate aliases: Box<[Symbol]>,


error[E0412]: cannot find type `FxHashSet` in this scope
   --> src/librustdoc/html/render/mod.rs:225:14
    |
225 |     structs: FxHashSet<ItemEntry>,


error[E0412]: cannot find type `FxHashSet` in this scope
   --> src/librustdoc/html/render/mod.rs:226:12
    |
226 |     enums: FxHashSet<ItemEntry>,


error[E0412]: cannot find type `FxHashSet` in this scope
   --> src/librustdoc/html/render/mod.rs:227:13
    |
227 |     unions: FxHashSet<ItemEntry>,


error[E0412]: cannot find type `FxHashSet` in this scope
   --> src/librustdoc/html/render/mod.rs:228:17
    |
228 |     primitives: FxHashSet<ItemEntry>,


error[E0412]: cannot find type `FxHashSet` in this scope
   --> src/librustdoc/html/render/mod.rs:229:13
    |
229 |     traits: FxHashSet<ItemEntry>,


error[E0412]: cannot find type `FxHashSet` in this scope
   --> src/librustdoc/html/render/mod.rs:230:13
    |
230 |     macros: FxHashSet<ItemEntry>,


error[E0412]: cannot find type `FxHashSet` in this scope
   --> src/librustdoc/html/render/mod.rs:231:16
    |
231 |     functions: FxHashSet<ItemEntry>,


error[E0412]: cannot find type `FxHashSet` in this scope
   --> src/librustdoc/html/render/mod.rs:232:15
    |
232 |     typedefs: FxHashSet<ItemEntry>,


error[E0412]: cannot find type `FxHashSet` in this scope
   --> src/librustdoc/html/render/mod.rs:233:17
    |
233 |     opaque_tys: FxHashSet<ItemEntry>,


error[E0412]: cannot find type `FxHashSet` in this scope
   --> src/librustdoc/html/render/mod.rs:234:14
    |
234 |     statics: FxHashSet<ItemEntry>,


error[E0412]: cannot find type `FxHashSet` in this scope
   --> src/librustdoc/html/render/mod.rs:235:16
    |
235 |     constants: FxHashSet<ItemEntry>,


error[E0412]: cannot find type `FxHashSet` in this scope
   --> src/librustdoc/html/render/mod.rs:236:17
    |
236 |     attributes: FxHashSet<ItemEntry>,


error[E0412]: cannot find type `FxHashSet` in this scope
   --> src/librustdoc/html/render/mod.rs:237:14
    |
237 |     derives: FxHashSet<ItemEntry>,


error[E0412]: cannot find type `FxHashSet` in this scope
   --> src/librustdoc/html/render/mod.rs:238:20
    |
238 |     trait_aliases: FxHashSet<ItemEntry>,


error[E0412]: cannot find type `FxHashSet` in this scope
   --> src/librustdoc/html/render/mod.rs:291:46
    |
291 |         fn print_entries(f: &mut Buffer, e: &FxHashSet<ItemEntry>, title: &str, class: &str) {

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/html/render/mod.rs:930:17
    |
    |
928 | fn render_stability_since_raw(
    |                              - help: you might be missing a type parameter: `<Symbol>`
929 |     w: &mut Buffer,
930 |     ver: Option<Symbol>,

error[E0412]: cannot find type `ConstStability` in this scope
   --> src/librustdoc/html/render/mod.rs:931:29
    |
    |
928 | fn render_stability_since_raw(
    |                              - help: you might be missing a type parameter: `<ConstStability>`
931 |     const_stability: Option<ConstStability>,
    |                             ^^^^^^^^^^^^^^ not found in this scope

error[E0412]: cannot find type `Symbol` in this scope
error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/html/render/mod.rs:932:28
    |
928 | fn render_stability_since_raw(
    |                              - help: you might be missing a type parameter: `<Symbol>`
932 |     containing_ver: Option<Symbol>,
    |                            ^^^^^^ not found in this scope

error[E0412]: cannot find type `Symbol` in this scope
error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/html/render/mod.rs:933:34
    |
928 | fn render_stability_since_raw(
    |                              - help: you might be missing a type parameter: `<Symbol>`
933 |     containing_const_ver: Option<Symbol>,
    |                                  ^^^^^^ not found in this scope

error[E0412]: cannot find type `Symbol` in this scope
error[E0412]: cannot find type `Symbol` in this scope
    --> src/librustdoc/html/render/mod.rs:1040:29
     |
1040 | const ALLOWED_ATTRIBUTES: &[Symbol] =


error[E0412]: cannot find type `FxHashSet` in this scope
    --> src/librustdoc/html/render/mod.rs:1081:28
     |
1081 |     GotoSource(ItemId, &'a FxHashSet<Symbol>),

error[E0412]: cannot find type `Symbol` in this scope
    --> src/librustdoc/html/render/mod.rs:1081:38
     |
     |
1079 | enum AssocItemLink<'a> {
     |                      - help: you might be missing a type parameter: `, Symbol`
1080 |     Anchor(Option<&'a str>),
1081 |     GotoSource(ItemId, &'a FxHashSet<Symbol>),

error[E0412]: cannot find type `DefId` in this scope
    --> src/librustdoc/html/render/mod.rs:1097:9
     |
     |
1097 |     it: DefId,
     |         ^^^^^ not found in this scope
     |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
1097 |     it: crate::core::ImplTraitParam,
     |         ~~~~~~~~~~~~~~~~~~~~~~~~~~~
     |         ~~~~~~~~~~~~~~~~~~~~~~~~~~~
1097 |     it: crate::html::render::ItemId,

error[E0412]: cannot find type `DefId` in this scope
    --> src/librustdoc/html/render/mod.rs:1109:9
     |
     |
1109 |     it: DefId,
     |         ^^^^^ not found in this scope
     |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
1109 |     it: crate::core::ImplTraitParam,
     |         ~~~~~~~~~~~~~~~~~~~~~~~~~~~
     |         ~~~~~~~~~~~~~~~~~~~~~~~~~~~
1109 |     it: crate::html::render::ItemId,


error[E0412]: cannot find type `FxHashSet` in this scope
    --> src/librustdoc/html/render/mod.rs:1111:18
     |
1111 |     derefs: &mut FxHashSet<DefId>,

error[E0412]: cannot find type `DefId` in this scope
    --> src/librustdoc/html/render/mod.rs:1111:28
     |
     |
1111 |     derefs: &mut FxHashSet<DefId>,
     |
     |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
     |
1111 |     derefs: &mut FxHashSet<crate::core::ImplTraitParam>,
     |                            ~~~~~~~~~~~~~~~~~~~~~~~~~~~
1111 |     derefs: &mut FxHashSet<crate::html::render::ItemId>,
help: you might be missing a type parameter
     |
     |
1105 | fn render_assoc_items_inner<DefId>(


error[E0412]: cannot find type `FxHashSet` in this scope
    --> src/librustdoc/html/render/mod.rs:1243:18
     |
1243 |     derefs: &mut FxHashSet<DefId>,

error[E0412]: cannot find type `DefId` in this scope
    --> src/librustdoc/html/render/mod.rs:1243:28
     |
     |
1243 |     derefs: &mut FxHashSet<DefId>,
     |
     |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
     |
1243 |     derefs: &mut FxHashSet<crate::core::ImplTraitParam>,
     |                            ~~~~~~~~~~~~~~~~~~~~~~~~~~~
1243 |     derefs: &mut FxHashSet<crate::html::render::ItemId>,
help: you might be missing a type parameter
     |
     |
1237 | fn render_deref_methods<DefId>(

error[E0412]: cannot find type `TyCtxt` in this scope
    --> src/librustdoc/html/render/mod.rs:1278:66
     |
     |
1278 | fn should_render_item(item: &clean::Item, deref_mut_: bool, tcx: TyCtxt<'_>) -> bool {


error[E0412]: cannot find type `FxHashSet` in this scope
    --> src/librustdoc/html/render/mod.rs:1913:34
     |
1913 | fn get_next_url(used_links: &mut FxHashSet<String>, url: String) -> String {

error[E0412]: cannot find type `Symbol` in this scope
    --> src/librustdoc/html/render/mod.rs:1925:11
     |
     |
1925 |     name: Symbol,
     |           ^^^^^^ not found in this scope

error[E0412]: cannot find type `FxHashSet` in this scope
    --> src/librustdoc/html/render/mod.rs:1958:22
1958 |     used_links: &mut FxHashSet<String>,
     |                      ^^^^^^^^^ not found in this scope

error[E0412]: cannot find type `TyCtxt` in this scope
error[E0412]: cannot find type `TyCtxt` in this scope
    --> src/librustdoc/html/render/mod.rs:1960:10
     |
1960 |     tcx: TyCtxt<'_>,
     |          ^^^^^^ not found in this scope

error[E0412]: cannot find type `FxHashSet` in this scope
    --> src/librustdoc/html/render/mod.rs:1982:22
1982 |     used_links: &mut FxHashSet<String>,
     |                      ^^^^^^^^^ not found in this scope


error[E0412]: cannot find type `FxHashSet` in this scope
    --> src/librustdoc/html/render/mod.rs:2151:18
     |
2151 |     derefs: &mut FxHashSet<DefId>,

error[E0412]: cannot find type `DefId` in this scope
    --> src/librustdoc/html/render/mod.rs:2151:28
     |
     |
2151 |     derefs: &mut FxHashSet<DefId>,
     |
     |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
     |
2151 |     derefs: &mut FxHashSet<crate::core::ImplTraitParam>,
     |                            ~~~~~~~~~~~~~~~~~~~~~~~~~~~
2151 |     derefs: &mut FxHashSet<crate::html::render::ItemId>,
help: you might be missing a type parameter
     |
     |
2146 | fn sidebar_deref_methods<DefId>(

error[E0412]: cannot find type `TyCtxt` in this scope
  --> src/librustdoc/html/sources.rs:31:10
   |
   |
31 |     tcx: TyCtxt<'tcx>,
   |          ^^^^^^ not found in this scope

error[E0412]: cannot find type `FxHashMap` in this scope
  --> src/librustdoc/html/sources.rs:34:6
34 | ) -> FxHashMap<PathBuf, String> {
   |      ^^^^^^^^^ not found in this scope

error[E0412]: cannot find type `TyCtxt` in this scope
error[E0412]: cannot find type `TyCtxt` in this scope
  --> src/librustdoc/html/sources.rs:41:10
   |
41 |     tcx: TyCtxt<'tcx>,
   |          ^^^^^^ not found in this scope

error[E0412]: cannot find type `FxHashMap` in this scope
  --> src/librustdoc/html/sources.rs:42:20
42 |     local_sources: FxHashMap<PathBuf, String>,
   |                    ^^^^^^^^^ not found in this scope

error[E0412]: cannot find type `Session` in this scope
error[E0412]: cannot find type `Session` in this scope
  --> src/librustdoc/html/sources.rs:46:48
   |
46 | fn is_real_and_local(span: clean::Span, sess: &Session) -> bool {


error[E0412]: cannot find type `FxHashSet` in this scope
  --> src/librustdoc/html/sources.rs:99:28
   |
99 |     emitted_local_sources: FxHashSet<PathBuf>,

error[E0412]: cannot find type `FileName` in this scope
   --> src/librustdoc/html/sources.rs:149:20
    |
---

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/html/url_parts_builder.rs:158:19
    |
158 | impl FromIterator<Symbol> for UrlPartsBuilder {
    |     -             ^^^^^^ not found in this scope
    |     |
    |     help: you might be missing a type parameter: `<Symbol>`
error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/html/url_parts_builder.rs:159:41
    |
    |
159 |     fn from_iter<T: IntoIterator<Item = Symbol>>(iter: T) -> Self {

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/html/url_parts_builder.rs:169:13
    |
    |
169 | impl Extend<Symbol> for UrlPartsBuilder {
    |     -       ^^^^^^ not found in this scope
    |     |
    |     help: you might be missing a type parameter: `<Symbol>`
error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/html/url_parts_builder.rs:170:38
    |
    |
170 |     fn extend<T: IntoIterator<Item = Symbol>>(&mut self, iter: T) {

error[E0412]: cannot find type `TyCtxt` in this scope
  --> src/librustdoc/json/conversions.rs:96:28
   |
   |
96 |     fn from_tcx(f: T, tcx: TyCtxt<'_>) -> Self;

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:100:28
    |
    |
100 |     fn into_tcx(self, tcx: TyCtxt<'_>) -> T;

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:107:28
    |
    |
107 |     fn into_tcx(self, tcx: TyCtxt<'_>) -> U {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:119:48
    |
    |
119 |     fn from_tcx(args: clean::GenericArgs, tcx: TyCtxt<'_>) -> Self {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:135:46
    |
    |
135 |     fn from_tcx(arg: clean::GenericArg, tcx: TyCtxt<'_>) -> Self {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:147:49
    |
    |
147 |     fn from_tcx(constant: clean::Constant, tcx: TyCtxt<'_>) -> Self {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:156:51
    |
    |
156 |     fn from_tcx(binding: clean::TypeBinding, tcx: TyCtxt<'_>) -> Self {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:166:52
    |
    |
166 |     fn from_tcx(kind: clean::TypeBindingKind, tcx: TyCtxt<'_>) -> Self {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/json/conversions.rs:178:25
    |
    |
178 |     struct DisplayDefId(DefId);
    |
    |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
    |
178 |     struct DisplayDefId(crate::core::ImplTraitParam);
    |                         ~~~~~~~~~~~~~~~~~~~~~~~~~~~
178 |     struct DisplayDefId(crate::json::conversions::ItemId);

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:198:44
    |
    |
198 | fn from_clean_item(item: clean::Item, tcx: TyCtxt<'_>) -> ItemEnum {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:252:46
    |
    |
252 |     fn from_tcx(struct_: clean::Struct, tcx: TyCtxt<'_>) -> Self {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:265:45
    |
    |
265 |     fn from_tcx(struct_: clean::Union, tcx: TyCtxt<'_>) -> Self {

error[E0412]: cannot find type `CtorKind` in this scope
   --> src/librustdoc/json/conversions.rs:276:38
    |
    |
276 | crate fn from_ctor_kind(struct_type: CtorKind) -> StructType {

error[E0412]: cannot find type `RustcAbi` in this scope
   --> src/librustdoc/json/conversions.rs:293:19
    |
    |
293 | fn convert_abi(a: RustcAbi) -> Abi {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:309:49
    |
    |
309 |     fn from_tcx(generics: clean::Generics, tcx: TyCtxt<'_>) -> Self {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:322:61
    |
    |
322 |     fn from_tcx(generic_param: clean::GenericParamDef, tcx: TyCtxt<'_>) -> Self {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:331:56
    |
    |
331 |     fn from_tcx(kind: clean::GenericParamDefKind, tcx: TyCtxt<'_>) -> Self {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:351:56
    |
    |
351 |     fn from_tcx(predicate: clean::WherePredicate, tcx: TyCtxt<'_>) -> Self {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:371:50
    |
    |
371 |     fn from_tcx(bound: clean::GenericBound, tcx: TyCtxt<'_>) -> Self {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:398:39
    |
    |
398 |     fn from_tcx(ty: clean::Type, tcx: TyCtxt<'_>) -> Self {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:463:41
    |
    |
463 |     fn from_tcx(term: clean::Term, tcx: TyCtxt<'_>) -> Term {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:472:58
    |
    |
472 |     fn from_tcx(bare_decl: clean::BareFunctionDecl, tcx: TyCtxt<'_>) -> Self {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:488:43
    |
    |
488 |     fn from_tcx(decl: clean::FnDecl, tcx: TyCtxt<'_>) -> Self {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:506:44
    |
    |
506 |     fn from_tcx(trait_: clean::Trait, tcx: TyCtxt<'_>) -> Self {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:520:42
    |
    |
520 |     fn from_tcx(impl_: clean::Impl, tcx: TyCtxt<'_>) -> Self {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:555:10
    |
---

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:581:42
    |
581 |     fn from_tcx(enum_: clean::Enum, tcx: TyCtxt<'_>) -> Self {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:593:54
    |
    |
593 |     fn from_tcx(struct_: clean::VariantStruct, _tcx: TyCtxt<'_>) -> Self {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:606:47
    |
    |
606 |     fn from_tcx(variant: clean::Variant, tcx: TyCtxt<'_>) -> Self {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:628:46
    |
    |
628 |     fn from_tcx(import: clean::Import, _tcx: TyCtxt<'_>) -> Self {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:648:46
    |
    |
648 |     fn from_tcx(mac: clean::ProcMacro, _tcx: TyCtxt<'_>) -> Self {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:666:47
    |
    |
666 |     fn from_tcx(typedef: clean::Typedef, tcx: TyCtxt<'_>) -> Self {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:673:47
    |
    |
673 |     fn from_tcx(opaque: clean::OpaqueTy, tcx: TyCtxt<'_>) -> Self {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:682:43
    |
    |
682 |     fn from_tcx(stat: clean::Static, tcx: TyCtxt<'_>) -> Self {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:692:48
    |
    |
692 |     fn from_tcx(alias: clean::TraitAlias, tcx: TyCtxt<'_>) -> Self {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/conversions.rs:701:39
    |
    |
701 |     fn from_tcx(kind: ItemType, _tcx: TyCtxt<'_>) -> Self {

error[E0412]: cannot find type `TyCtxt` in this scope
  --> src/librustdoc/json/mod.rs:33:10
   |
   |
33 |     tcx: TyCtxt<'tcx>,
   |          ^^^^^^ not found in this scope

error[E0412]: cannot find type `FxHashMap` in this scope
  --> src/librustdoc/json/mod.rs:36:23
   |
36 |     index: Rc<RefCell<FxHashMap<types::Id, types::Item>>>,

error[E0412]: cannot find type `Session` in this scope
  --> src/librustdoc/json/mod.rs:43:29
   |
   |
43 |     fn sess(&self) -> &'tcx Session {
   |                             ^^^^^^^ not found in this scope

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/json/mod.rs:47:46
   |
47 |     fn get_trait_implementors(&mut self, id: DefId) -> Vec<types::Id> {
   |
   |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
   |
47 |     fn get_trait_implementors(&mut self, id: crate::clean::ItemId) -> Vec<types::Id> {
   |                                              ~~~~~~~~~~~~~~~~~~~~
47 |     fn get_trait_implementors(&mut self, id: crate::core::ImplTraitParam) -> Vec<types::Id> {

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/json/mod.rs:64:33
   |
   |
64 |     fn get_impls(&mut self, id: DefId) -> Vec<types::Id> {
   |
   |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
   |
64 |     fn get_impls(&mut self, id: crate::clean::ItemId) -> Vec<types::Id> {
   |                                 ~~~~~~~~~~~~~~~~~~~~
64 |     fn get_impls(&mut self, id: crate::core::ImplTraitParam) -> Vec<types::Id> {

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/json/mod.rs:154:14
    |
    |
154 |         tcx: TyCtxt<'tcx>,
    |              ^^^^^^ not found in this scope

error[E0412]: cannot find type `FxHashMap` in this scope
   |
   |
24 | ) -> (Vec<(String, lint::Level)>, FxHashMap<lint::LintId, lint::Level>)

error[E0412]: cannot find type `Lint` in this scope
   --> src/librustdoc/lint.rs:172:47
    |
    |
172 | crate static RUSTDOC_LINTS: Lazy<Vec<&'static Lint>> = Lazy::new(|| {

error[E0412]: cannot find type `Session` in this scope
   --> src/librustdoc/lint.rs:186:33
    |
    |
186 | crate fn register_lints(_sess: &Session, lint_store: &mut LintStore) {

error[E0412]: cannot find type `LintStore` in this scope
   --> src/librustdoc/lint.rs:186:59
    |
    |
186 | crate fn register_lints(_sess: &Session, lint_store: &mut LintStore) {

error[E0412]: cannot find type `Edition` in this scope
  --> src/librustdoc/markdown.rs:42:14
   |
   |
42 |     edition: Edition,
   |              ^^^^^^^ not found in this scope

error[E0412]: cannot find type `AccessLevels` in this scope
  --> src/librustdoc/passes/stripper.rs:12:30
   |
12 |     crate access_levels: &'a AccessLevels<DefId>,

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/passes/stripper.rs:12:43
   |
   |
12 |     crate access_levels: &'a AccessLevels<DefId>,
   |
   |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
   |
12 |     crate access_levels: &'a AccessLevels<crate::clean::ItemId>,
   |                                           ~~~~~~~~~~~~~~~~~~~~
12 |     crate access_levels: &'a AccessLevels<crate::core::ImplTraitParam>,
help: you might be missing a type parameter
   |
   |
10 | crate struct Stripper<'a, DefId> {

error[E0412]: cannot find type `Resolver` in this scope
  --> src/librustdoc/passes/collect_intra_doc_links/early.rs:22:20
   |
   |
22 |     resolver: &mut Resolver<'_>,

error[E0412]: cannot find type `Externs` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links/early.rs:24:14
    |
    |
24  |     externs: Externs,
    |              ^^^^^^^ help: a trait with a similar name exists: `Extend`
    |
   ::: /checkout/library/core/src/iter/traits/collect.rs:351:1
    |
351 | pub trait Extend<A> {
    | ------------------- similarly named trait `Extend` defined here
error[E0412]: cannot find type `Resolver` in this scope
  --> src/librustdoc/passes/collect_intra_doc_links/early.rs:61:23
   |
   |
61 |     resolver: &'r mut Resolver<'ra>,

error[E0412]: cannot find type `LocalDefId` in this scope
  --> src/librustdoc/passes/collect_intra_doc_links/early.rs:62:18
   |
   |
62 |     current_mod: LocalDefId,
   |                  ^^^^^^^^^^ not found in this scope

error[E0412]: cannot find type `DefIdSet` in this scope
  --> src/librustdoc/passes/collect_intra_doc_links/early.rs:63:19
   |
63 |     visited_mods: DefIdSet,

error[E0412]: cannot find type `DefIdMap` in this scope
  --> src/librustdoc/passes/collect_intra_doc_links/early.rs:64:22
   |
   |
64 |     traits_in_scope: DefIdMap<Vec<TraitCandidate>>,

error[E0412]: cannot find type `TraitCandidate` in this scope
  --> src/librustdoc/passes/collect_intra_doc_links/early.rs:64:35
   |
   |
60 | struct EarlyDocLinkResolver<'r, 'ra> {
   |                                    - help: you might be missing a type parameter: `, TraitCandidate`
...
64 |     traits_in_scope: DefIdMap<Vec<TraitCandidate>>,

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/passes/collect_intra_doc_links/early.rs:65:21
   |
   |
65 |     all_traits: Vec<DefId>,
   |
   |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
   |
65 |     all_traits: Vec<crate::clean::ItemId>,
   |                     ~~~~~~~~~~~~~~~~~~~~
65 |     all_traits: Vec<crate::core::ImplTraitParam>,
help: you might be missing a type parameter
   |
   |
60 | struct EarlyDocLinkResolver<'r, 'ra, DefId> {

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/passes/collect_intra_doc_links/early.rs:66:26
   |
   |
66 |     all_trait_impls: Vec<DefId>,
   |
   |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
   |
66 |     all_trait_impls: Vec<crate::clean::ItemId>,
   |                          ~~~~~~~~~~~~~~~~~~~~
66 |     all_trait_impls: Vec<crate::core::ImplTraitParam>,
help: you might be missing a type parameter
   |
   |
60 | struct EarlyDocLinkResolver<'r, 'ra, DefId> {

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/passes/collect_intra_doc_links/early.rs:71:47
   |
   |
71 |     fn add_traits_in_scope(&mut self, def_id: DefId) {
   |                                               ^^^^^ not found in this scope
   |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
   |
71 |     fn add_traits_in_scope(&mut self, def_id: crate::clean::ItemId) {
   |                                               ~~~~~~~~~~~~~~~~~~~~
71 |     fn add_traits_in_scope(&mut self, def_id: crate::core::ImplTraitParam) {

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/passes/collect_intra_doc_links/early.rs:95:54
   |
   |
95 |     fn add_traits_in_parent_scope(&mut self, def_id: DefId) {
   |                                                      ^^^^^ not found in this scope
   |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
   |
95 |     fn add_traits_in_parent_scope(&mut self, def_id: crate::clean::ItemId) {
   |                                                      ~~~~~~~~~~~~~~~~~~~~
95 |     fn add_traits_in_parent_scope(&mut self, def_id: crate::core::ImplTraitParam) {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links/early.rs:187:67
    |
    |
187 |     fn process_module_children_or_reexports(&mut self, module_id: DefId) {
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
187 |     fn process_module_children_or_reexports(&mut self, module_id: crate::clean::ItemId) {
    |                                                                   ~~~~~~~~~~~~~~~~~~~~
187 |     fn process_module_children_or_reexports(&mut self, module_id: crate::core::ImplTraitParam) {

error[E0405]: cannot find trait `Visitor` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links/early.rs:220:6
    |
    |
220 | impl Visitor<'_> for EarlyDocLinkResolver<'_, '_> {
    |
help: consider importing this trait
    |
1   | use serde::de::Visitor;
1   | use serde::de::Visitor;
    |

error[E0412]: cannot find type `AssocCtxt` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links/early.rs:243:65
    |
243 |     fn visit_assoc_item(&mut self, item: &ast::AssocItem, ctxt: AssocCtxt) {

error[E0412]: cannot find type `DefKind` in this scope
  --> src/librustdoc/passes/collect_intra_doc_links.rs:63:9
   |
   |
63 |     Def(DefKind, DefId),

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/passes/collect_intra_doc_links.rs:63:18
   |
   |
63 |     Def(DefKind, DefId),
   |
   |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
   |
63 |     Def(DefKind, crate::clean::ItemId),
   |                  ~~~~~~~~~~~~~~~~~~~~
63 |     Def(DefKind, crate::core::ImplTraitParam),

error[E0412]: cannot find type `TyCtxt` in this scope
  --> src/librustdoc/passes/collect_intra_doc_links.rs:84:24
   |
   |
84 |     fn name(self, tcx: TyCtxt<'_>) -> Symbol {

error[E0412]: cannot find type `Symbol` in this scope
  --> src/librustdoc/passes/collect_intra_doc_links.rs:84:39
   |
   |
84 |     fn name(self, tcx: TyCtxt<'_>) -> Symbol {

error[E0412]: cannot find type `TyCtxt` in this scope
  --> src/librustdoc/passes/collect_intra_doc_links.rs:91:26
   |
   |
91 |     fn def_id(self, tcx: TyCtxt<'_>) -> DefId {

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/passes/collect_intra_doc_links.rs:91:41
   |
   |
91 |     fn def_id(self, tcx: TyCtxt<'_>) -> DefId {
   |
   |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
   |
91 |     fn def_id(self, tcx: TyCtxt<'_>) -> crate::clean::ItemId {
   |                                         ~~~~~~~~~~~~~~~~~~~~
91 |     fn def_id(self, tcx: TyCtxt<'_>) -> crate::core::ImplTraitParam {

error[E0412]: cannot find type `Namespace` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links.rs:172:22
    |
    |
172 |         expected_ns: Namespace,
    |                      ^^^^^^^^^ not found in this scope
    |
help: there is an enum variant `crate::collect_intra_doc_links::Disambiguator::Namespace`; try using the variant's enum
172 |         expected_ns: crate::collect_intra_doc_links::Disambiguator,
    |                      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

error[E0412]: cannot find type `DefId` in this scope
error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links.rs:180:20
    |
180 |         module_id: DefId,
    |                    ^^^^^ not found in this scope
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
180 |         module_id: crate::clean::ItemId,
    |                    ~~~~~~~~~~~~~~~~~~~~
180 |         module_id: crate::core::ImplTraitParam,

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links.rs:279:49
    |
    |
279 |     crate fn render(&self, s: &mut String, tcx: TyCtxt<'_>) -> std::fmt::Result {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links.rs:288:41
    |
    |
288 | crate struct ItemFragment(FragmentKind, DefId);
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
288 | crate struct ItemFragment(FragmentKind, crate::clean::ItemId);
    |                                         ~~~~~~~~~~~~~~~~~~~~
288 | crate struct ItemFragment(FragmentKind, crate::core::ImplTraitParam);

error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links.rs:321:49
    |
    |
321 |     crate fn render(&self, s: &mut String, tcx: TyCtxt<'_>) -> std::fmt::Result {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links.rs:346:16
    |
    |
346 |     module_id: DefId,
    |                ^^^^^ not found in this scope
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
346 |     module_id: crate::clean::ItemId,
    |                ~~~~~~~~~~~~~~~~~~~~
346 |     module_id: crate::core::ImplTraitParam,

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links.rs:371:18
    |
    |
371 |     mod_ids: Vec<DefId>,
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
371 |     mod_ids: Vec<crate::clean::ItemId>,
    |                  ~~~~~~~~~~~~~~~~~~~~
371 |     mod_ids: Vec<crate::core::ImplTraitParam>,
help: you might be missing a type parameter
    |
    |
365 | struct LinkCollector<'a, 'tcx, DefId> {


error[E0412]: cannot find type `FxHashMap` in this scope
    |
    |
374 |     visited_links: FxHashMap<ResolutionInfo, Option<CachedLink>>,

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links.rs:388:20
    |
    |
388 |         module_id: DefId,
    |                    ^^^^^ not found in this scope
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
388 |         module_id: crate::clean::ItemId,
    |                    ~~~~~~~~~~~~~~~~~~~~
388 |         module_id: crate::core::ImplTraitParam,

error[E0412]: cannot find type `Namespace` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links.rs:461:13
    |
    |
461 |         ns: Namespace,
    |
    |
help: there is an enum variant `crate::collect_intra_doc_links::Disambiguator::Namespace`; try using the variant's enum
461 |         ns: crate::collect_intra_doc_links::Disambiguator,
    |             ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

error[E0412]: cannot find type `Symbol` in this scope
---
    |
483 |         module_id: DefId,
    |                    ^^^^^ not found in this scope
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
483 |         module_id: crate::clean::ItemId,
    |                    ~~~~~~~~~~~~~~~~~~~~
483 |         module_id: crate::core::ImplTraitParam,

error[E0412]: cannot find type `Namespace` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links.rs:495:51
    |
    |
495 |     fn resolve_self_ty(&self, path_str: &str, ns: Namespace, item_id: ItemId) -> Option<Res> {
    |
    |
help: there is an enum variant `crate::collect_intra_doc_links::Disambiguator::Namespace`; try using the variant's enum
    |
495 |     fn resolve_self_ty(&self, path_str: &str, ns: crate::collect_intra_doc_links::Disambiguator, item_id: ItemId) -> Option<Res> {

error[E0412]: cannot find type `Namespace` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links.rs:527:55
    |
    |
527 |     fn resolve_macro_rules(&self, path_str: &str, ns: Namespace) -> Option<Res> {
    |
    |
help: there is an enum variant `crate::collect_intra_doc_links::Disambiguator::Namespace`; try using the variant's enum
    |
527 |     fn resolve_macro_rules(&self, path_str: &str, ns: crate::collect_intra_doc_links::Disambiguator) -> Option<Res> {

error[E0412]: cannot find type `Namespace` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links.rs:548:13
    |
    |
548 |         ns: Namespace,
    |
    |
help: there is an enum variant `crate::collect_intra_doc_links::Disambiguator::Namespace`; try using the variant's enum
548 |         ns: crate::collect_intra_doc_links::Disambiguator,
    |             ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

error[E0412]: cannot find type `DefId` in this scope
error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links.rs:550:20
    |
550 |         module_id: DefId,
    |                    ^^^^^ not found in this scope
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
550 |         module_id: crate::clean::ItemId,
    |                    ~~~~~~~~~~~~~~~~~~~~
550 |         module_id: crate::core::ImplTraitParam,

error[E0412]: cannot find type `Namespace` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links.rs:572:13
    |
    |
572 |         ns: Namespace,
    |
    |
help: there is an enum variant `crate::collect_intra_doc_links::Disambiguator::Namespace`; try using the variant's enum
572 |         ns: crate::collect_intra_doc_links::Disambiguator,
    |             ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

error[E0412]: cannot find type `DefId` in this scope
error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links.rs:574:20
    |
574 |         module_id: DefId,
    |                    ^^^^^ not found in this scope
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
574 |         module_id: crate::clean::ItemId,
    |                    ~~~~~~~~~~~~~~~~~~~~
574 |         module_id: crate::core::ImplTraitParam,

error[E0412]: cannot find type `Namespace` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links.rs:596:13
    |
    |
596 |         ns: Namespace,
    |
    |
help: there is an enum variant `crate::collect_intra_doc_links::Disambiguator::Namespace`; try using the variant's enum
596 |         ns: crate::collect_intra_doc_links::Disambiguator,
    |             ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

error[E0412]: cannot find type `DefId` in this scope
error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links.rs:598:20
    |
598 |         module_id: DefId,
    |                    ^^^^^ not found in this scope
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
598 |         module_id: crate::clean::ItemId,
    |                    ~~~~~~~~~~~~~~~~~~~~
598 |         module_id: crate::core::ImplTraitParam,

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links.rs:665:36
    |
    |
665 |     fn def_id_to_res(&self, ty_id: DefId) -> Option<Res> {
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
665 |     fn def_id_to_res(&self, ty_id: crate::clean::ItemId) -> Option<Res> {
    |                                    ~~~~~~~~~~~~~~~~~~~~
665 |     fn def_id_to_res(&self, ty_id: crate::core::ImplTraitParam) -> Option<Res> {

error[E0412]: cannot find type `Ty` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links.rs:703:71
    |
    |
703 |     fn primitive_type_to_ty(&mut self, prim: PrimitiveType) -> Option<Ty<'tcx>> {

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links.rs:738:20
    |
    |
738 |         item_name: Symbol,
    |                    ^^^^^^ not found in this scope

error[E0412]: cannot find type `Namespace` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links.rs:739:13
    |
739 |         ns: Namespace,
    |
    |
help: there is an enum variant `crate::collect_intra_doc_links::Disambiguator::Namespace`; try using the variant's enum
739 |         ns: crate::collect_intra_doc_links::Disambiguator,
    |             ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

error[E0412]: cannot find type `DefId` in this scope
error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links.rs:740:20
    |
740 |         module_id: DefId,
    |                    ^^^^^ not found in this scope
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
740 |         module_id: crate::clean::ItemId,
    |                    ~~~~~~~~~~~~~~~~~~~~
740 |         module_id: crate::core::ImplTraitParam,

error[E0412]: cannot find type `Namespace` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links.rs:873:13
    |
    |
873 |         ns: Namespace,
    |
    |
help: there is an enum variant `crate::collect_intra_doc_links::Disambiguator::Namespace`; try using the variant's enum
873 |         ns: crate::collect_intra_doc_links::Disambiguator,
    |             ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

error[E0412]: cannot find type `DefId` in this scope
error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links.rs:876:20
    |
876 |         module_id: DefId,
    |                    ^^^^^ not found in this scope
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
876 |         module_id: crate::clean::ItemId,
    |                    ~~~~~~~~~~~~~~~~~~~~
876 |         module_id: crate::core::ImplTraitParam,

error[E0412]: cannot find type `Ty` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links.rs:912:9
    |
---
    |
913 |     module: DefId,
    |             ^^^^^ not found in this scope
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
913 |     module: crate::clean::ItemId,
    |             ~~~~~~~~~~~~~~~~~~~~
913 |     module: crate::core::ImplTraitParam,
    |             ~~~~~~~~~~~~~~~~~~~~~~~~~~~
---

error[E0412]: cannot find type `Namespace` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links.rs:915:9
    |
915 |     ns: Namespace,
    |
    |
help: there is an enum variant `crate::collect_intra_doc_links::Disambiguator::Namespace`; try using the variant's enum
915 |     ns: crate::collect_intra_doc_links::Disambiguator,
    |         ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

error[E0412]: cannot find type `TyCtxt` in this scope
---
    |
952 |     impl_id: DefId,
    |              ^^^^^ not found in this scope
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
952 |     impl_id: crate::clean::ItemId,
    |              ~~~~~~~~~~~~~~~~~~~~
952 |     impl_id: crate::core::ImplTraitParam,

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links.rs:953:21
    |
    |
953 |     trait_assoc_id: DefId,
    |                     ^^^^^ not found in this scope
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
953 |     trait_assoc_id: crate::clean::ItemId,
    |                     ~~~~~~~~~~~~~~~~~~~~
953 |     trait_assoc_id: crate::core::ImplTraitParam,

error[E0412]: cannot find type `Ty` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links.rs:972:9
    |
---
    |
973 |     module: DefId,
    |             ^^^^^ not found in this scope
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
973 |     module: crate::clean::ItemId,
    |             ~~~~~~~~~~~~~~~~~~~~
973 |     module: crate::core::ImplTraitParam,
    |             ~~~~~~~~~~~~~~~~~~~~~~~~~~~
    |             ~~~~~~~~~~~~~~~~~~~~~~~~~~~

error[E0412]: cannot find type `FxHashSet` in this scope
    |
    |
974 | ) -> FxHashSet<(DefId, DefId)> {

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links.rs:974:17
    |
    |
974 | ) -> FxHashSet<(DefId, DefId)> {
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
974 | ) -> FxHashSet<(crate::clean::ItemId, DefId)> {
    |                 ~~~~~~~~~~~~~~~~~~~~
974 | ) -> FxHashSet<(crate::core::ImplTraitParam, DefId)> {
help: you might be missing a type parameter
    |
    |
970 | fn trait_impls_for<'a, DefId>(

error[E0412]: cannot find type `DefId` in this scope
   --> src/librustdoc/passes/collect_intra_doc_links.rs:974:24
    |
    |
974 | ) -> FxHashSet<(DefId, DefId)> {
    |
    |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
    |
974 | ) -> FxHashSet<(DefId, crate::clean::ItemId)> {
    |                        ~~~~~~~~~~~~~~~~~~~~
974 | ) -> FxHashSet<(DefId, crate::core::ImplTraitParam)> {
help: you might be missing a type parameter
    |
    |
970 | fn trait_impls_for<'a, DefId>(

error[E0412]: cannot find type `PerNS` in this scope
    --> src/librustdoc/passes/collect_intra_doc_links.rs:1014:38
     |
     |
1014 | fn is_derive_trait_collision<T>(ns: &PerNS<Result<(Res, T), ResolutionFailure<'_>>>) -> bool {

error[E0412]: cannot find type `DefId` in this scope
    --> src/librustdoc/passes/collect_intra_doc_links.rs:1183:29
     |
     |
1183 |         parent_node: Option<DefId>,
     |
     |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
     |
1183 |         parent_node: Option<crate::clean::ItemId>,
     |                             ~~~~~~~~~~~~~~~~~~~~
1183 |         parent_node: Option<crate::core::ImplTraitParam>,
help: you might be missing a type parameter
     |
     |
1175 | impl<DefId> LinkCollector<'_, '_> {

error[E0412]: cannot find type `DefKind` in this scope
    --> src/librustdoc/passes/collect_intra_doc_links.rs:1355:15
     |
---
     |
1356 |         id: DefId,
     |             ^^^^^ not found in this scope
     |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
     |
1356 |         id: crate::clean::ItemId,
     |             ~~~~~~~~~~~~~~~~~~~~
1356 |         id: crate::core::ImplTraitParam,

error[E0412]: cannot find type `DefKind` in this scope
    --> src/librustdoc/passes/collect_intra_doc_links.rs:1690:10
     |
---
     |
1692 |     Namespace(Namespace),
     |               ^^^^^^^^^ not found in this scope
     |
help: there is an enum variant `crate::collect_intra_doc_links::Disambiguator::Namespace`; try using the variant's enum
1692 |     Namespace(crate::collect_intra_doc_links::Disambiguator),
     |               ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

error[E0412]: cannot find type `Namespace` in this scope
error[E0412]: cannot find type `Namespace` in this scope
    --> src/librustdoc/passes/collect_intra_doc_links.rs:1743:20
     |
1743 |     fn ns(self) -> Namespace {
     |
     |
help: there is an enum variant `crate::collect_intra_doc_links::Disambiguator::Namespace`; try using the variant's enum
     |
1743 |     fn ns(self) -> crate::collect_intra_doc_links::Disambiguator {

error[E0412]: cannot find type `TyCtxt` in this scope
    --> src/librustdoc/passes/collect_intra_doc_links.rs:1861:10
     |
---
     |
1862 |     lint: &'static Lint,
     |                    ^^^^ not found in this scope

error[E0412]: cannot find type `Diagnostic` in this scope
     |
     |
1865 |     decorate: impl FnOnce(&mut Diagnostic, Option<rustc_span::Span>),


error[E0412]: cannot find type `Diagnostic` in this scope
     |
2243 |     diag: &mut Diagnostic,
     |                ^^^^^^^^^^ not found in this scope


error[E0412]: cannot find type `Namespace` in this scope
    --> src/librustdoc/passes/collect_intra_doc_links.rs:2307:42
     |
2307 | fn resolve_primitive(path_str: &str, ns: Namespace) -> Option<Res> {
     |
     |
help: there is an enum variant `crate::collect_intra_doc_links::Disambiguator::Namespace`; try using the variant's enum
     |
2307 | fn resolve_primitive(path_str: &str, ns: crate::collect_intra_doc_links::Disambiguator) -> Option<Res> {


error[E0412]: cannot find type `FxHashMap` in this scope
  --> src/librustdoc/passes/collect_trait_impls.rs:67:15
   |
67 |         map: &FxHashMap<DefId, &Type>,

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/passes/collect_trait_impls.rs:67:25
   |
   |
67 |         map: &FxHashMap<DefId, &Type>,
   |
   |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
   |
67 |         map: &FxHashMap<crate::core::ImplTraitParam, &Type>,
   |                         ~~~~~~~~~~~~~~~~~~~~~~~~~~~
67 |         map: &FxHashMap<crate::passes::collect_trait_impls::ItemId, &Type>,
help: you might be missing a type parameter
   |
   |
65 |     fn add_deref_target<DefId>(

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/passes/collect_trait_impls.rs:69:19
   |
   |
69 |         type_did: DefId,
   |                   ^^^^^ not found in this scope
   |
help: there is an enum variant `crate::core::ImplTraitParam::DefId` and 1 other; try using the variant's enum
   |
69 |         type_did: crate::core::ImplTraitParam,
   |                   ~~~~~~~~~~~~~~~~~~~~~~~~~~~
69 |         type_did: crate::passes::collect_trait_impls::ItemId,


error[E0412]: cannot find type `FxHashSet` in this scope
   --> src/librustdoc/passes/collect_trait_impls.rs:191:12
    |
191 |     items: FxHashSet<ItemId>,


error[E0412]: cannot find type `FxHashSet` in this scope
   --> src/librustdoc/passes/collect_trait_impls.rs:209:12
    |
209 |     prims: FxHashSet<PrimitiveType>,


error[E0412]: cannot find type `FxHashSet` in this scope
   --> src/librustdoc/passes/collect_trait_impls.rs:210:12
    |
210 |     items: FxHashSet<ItemId>,


error[E0412]: cannot find type `Lrc` in this scope
   --> src/librustdoc/passes/check_code_block_syntax.rs:175:13
    |
175 |     buffer: Lrc<Lock<Buffer>>,

error[E0412]: cannot find type `Lock` in this scope
   --> src/librustdoc/passes/check_code_block_syntax.rs:175:17
    |
    |
175 |     buffer: Lrc<Lock<Buffer>>,


error[E0412]: cannot find type `Lrc` in this scope
   --> src/librustdoc/passes/check_code_block_syntax.rs:176:22
    |
176 |     fallback_bundle: Lrc<rustc_errors::FluentBundle>,


error[E0405]: cannot find trait `Emitter` in this scope
   --> src/librustdoc/passes/check_code_block_syntax.rs:179:6
179 | impl Emitter for BufferEmitter {
    |      ^^^^^^^ not found in this scope


error[E0412]: cannot find type `Diagnostic` in this scope
   --> src/librustdoc/passes/check_code_block_syntax.rs:180:42
    |
180 |     fn emit_diagnostic(&mut self, diag: &Diagnostic) {


error[E0412]: cannot find type `Lrc` in this scope
   --> src/librustdoc/passes/check_code_block_syntax.rs:189:37
    |
189 |     fn source_map(&self) -> Option<&Lrc<SourceMap>> {

error[E0412]: cannot find type `SourceMap` in this scope
   --> src/librustdoc/passes/check_code_block_syntax.rs:189:41
    |
    |
179 | impl Emitter for BufferEmitter {
    |     - help: you might be missing a type parameter: `<SourceMap>`
...
189 |     fn source_map(&self) -> Option<&Lrc<SourceMap>> {


error[E0412]: cannot find type `Lrc` in this scope
   --> src/librustdoc/passes/check_code_block_syntax.rs:193:40
    |
193 |     fn fluent_bundle(&self) -> Option<&Lrc<rustc_errors::FluentBundle>> {


error[E0412]: cannot find type `Lrc` in this scope
   --> src/librustdoc/passes/check_code_block_syntax.rs:197:42
    |
197 |     fn fallback_fluent_bundle(&self) -> &Lrc<rustc_errors::FluentBundle> {

error[E0412]: cannot find type `FileName` in this scope
   --> src/librustdoc/passes/calculate_doc_coverage.rs:105:21
    |
    |
104 | struct CoverageCalculator<'a, 'b> {
    |                                 - help: you might be missing a type parameter: `, FileName`
105 |     items: BTreeMap<FileName, ItemCount>,

error[E0412]: cannot find type `Span` in this scope
   --> src/librustdoc/passes/mod.rs:134:61
    |
    |
134 | crate fn span_of_attrs(attrs: &clean::Attributes) -> Option<Span> {
    |
help: consider importing one of these items
    |
4   | use crate::clean::Span;
---

error[E0412]: cannot find type `Span` in this scope
   --> src/librustdoc/passes/mod.rs:156:13
    |
156 | ) -> Option<Span> {
    |
help: consider importing one of these items
    |
4   | use crate::clean::Span;
---

error[E0412]: cannot find type `SourceFile` in this scope
  --> src/librustdoc/scrape_examples.rs:74:43
   |
74 |     fn new(span: rustc_span::Span, file: &SourceFile) -> Self {

error[E0412]: cannot find type `SourceFile` in this scope
  --> src/librustdoc/scrape_examples.rs:95:23
   |
---
    |
109 |     crate edition: Edition,
    |                    ^^^^^^^ not found in this scope

error[E0412]: cannot find type `FxHashMap` in this scope
    |
    |
112 | crate type FnCallLocations = FxHashMap<PathBuf, CallData>;


error[E0412]: cannot find type `FxHashMap` in this scope
    |
    |
113 | crate type AllCallLocations = FxHashMap<DefPathHash, FnCallLocations>;


error[E0412]: cannot find type `DefPathHash` in this scope
    |
    |
113 | crate type AllCallLocations = FxHashMap<DefPathHash, FnCallLocations>;
    |                            -            ^^^^^^^^^^^ not found in this scope
    |                            |
    |                            help: you might be missing a type parameter: `<DefPathHash>`
error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/scrape_examples.rs:117:10
    |
117 |     tcx: TyCtxt<'tcx>,
117 |     tcx: TyCtxt<'tcx>,
    |          ^^^^^^ not found in this scope

error[E0412]: cannot find type `Map` in this scope
   --> src/librustdoc/scrape_examples.rs:118:10
    |
118 |     map: Map<'tcx>,
    |
help: consider importing one of these items
    |
3   | use core::iter::Map;
3   | use core::iter::Map;
    |
3   | use itertools::__std_iter::Map;
    |
3   | use rayon::iter::Map;
3   | use serde_json::Map;
    |
      and 1 other candidate


error[E0412]: cannot find type `CrateNum` in this scope
   --> src/librustdoc/scrape_examples.rs:120:24
    |
116 | struct FindCalls<'a, 'tcx> {
    |                          - help: you might be missing a type parameter: `, CrateNum`
120 |     target_crates: Vec<CrateNum>,
    |                        ^^^^^^^^ not found in this scope

error[E0405]: cannot find trait `Visitor` in this scope
error[E0405]: cannot find trait `Visitor` in this scope
   --> src/librustdoc/scrape_examples.rs:124:16
    |
124 | impl<'a, 'tcx> Visitor<'tcx> for FindCalls<'a, 'tcx>
    |
help: consider importing this trait
    |
3   | use serde::de::Visitor;
---
    |
239 |     tcx: TyCtxt<'_>,
    |          ^^^^^^ not found in this scope

error[E0412]: cannot find type `FxHashSet` in this scope
   |
   |
14 |     crate children: FxHashSet<CssPath>,


error[E0412]: cannot find type `FxHashSet` in this scope
    |
    |
191 | fn inner(v: &[u8], events: &[Events], pos: &mut usize) -> FxHashSet<CssPath> {

error[E0412]: cannot find type `Handler` in this scope
   --> src/librustdoc/theme.rs:256:12
    |
    |
256 |     diag: &Handler,

error[E0412]: cannot find type `Symbol` in this scope
  --> src/librustdoc/visit_ast.rs:25:17
   |
---

error[E0412]: cannot find type `Symbol` in this scope
  --> src/librustdoc/visit_ast.rs:30:53
   |
24 | crate struct Module<'hir> {
   |                         - help: you might be missing a type parameter: `, Symbol`
...
30 |     crate items: Vec<(&'hir hir::Item<'hir>, Option<Symbol>)>,

error[E0412]: cannot find type `Symbol` in this scope
  --> src/librustdoc/visit_ast.rs:31:63
   |
   |
24 | crate struct Module<'hir> {
   |                         - help: you might be missing a type parameter: `, Symbol`
...
31 |     crate foreigns: Vec<(&'hir hir::ForeignItem<'hir>, Option<Symbol>)>,

error[E0412]: cannot find type `Symbol` in this scope
  --> src/librustdoc/visit_ast.rs:35:24
   |
   |
35 |     crate fn new(name: Symbol, id: hir::HirId, where_inner: Span) -> Self {

error[E0412]: cannot find type `Span` in this scope
  --> src/librustdoc/visit_ast.rs:35:61
   |
   |
35 |     crate fn new(name: Symbol, id: hir::HirId, where_inner: Span) -> Self {
   |
help: consider importing one of these items
   |
4  | use crate::clean::Span;
---

error[E0412]: cannot find type `TyCtxt` in this scope
  --> src/librustdoc/visit_ast.rs:39:38
   |
39 |     crate fn where_outer(&self, tcx: TyCtxt<'_>) -> Span {

error[E0412]: cannot find type `Span` in this scope
  --> src/librustdoc/visit_ast.rs:39:53
   |
   |
39 |     crate fn where_outer(&self, tcx: TyCtxt<'_>) -> Span {
   |
help: consider importing one of these items
   |
4  | use crate::clean::Span;
---

error[E0412]: cannot find type `TyCtxt` in this scope
  --> src/librustdoc/visit_ast.rs:45:24
   |
45 | fn def_id_to_path(tcx: TyCtxt<'_>, did: DefId) -> Vec<Symbol> {

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/visit_ast.rs:45:41
   |
   |
45 | fn def_id_to_path(tcx: TyCtxt<'_>, did: DefId) -> Vec<Symbol> {
   |
   |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
   |
45 | fn def_id_to_path(tcx: TyCtxt<'_>, did: crate::clean::ItemId) -> Vec<Symbol> {
   |                                         ~~~~~~~~~~~~~~~~~~~~
45 | fn def_id_to_path(tcx: TyCtxt<'_>, did: crate::core::ImplTraitParam) -> Vec<Symbol> {

error[E0412]: cannot find type `Symbol` in this scope
  --> src/librustdoc/visit_ast.rs:45:55
   |
   |
45 | fn def_id_to_path(tcx: TyCtxt<'_>, did: DefId) -> Vec<Symbol> {
   |                  -                                    ^^^^^^ not found in this scope
   |                  |
   |                  help: you might be missing a type parameter: `<Symbol>`
error[E0412]: cannot find type `TyCtxt` in this scope
  --> src/librustdoc/visit_ast.rs:51:35
   |
   |
51 | crate fn inherits_doc_hidden(tcx: TyCtxt<'_>, mut node: hir::HirId) -> bool {


error[E0412]: cannot find type `FxHashSet` in this scope
   |
   |
66 |     view_item_stack: FxHashSet<hir::HirId>,


error[E0412]: cannot find type `FxHashMap` in this scope
   |
   |
70 |     exact_paths: FxHashMap<DefId, Vec<Symbol>>,

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/visit_ast.rs:70:28
   |
   |
70 |     exact_paths: FxHashMap<DefId, Vec<Symbol>>,
   |
   |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
   |
70 |     exact_paths: FxHashMap<crate::clean::ItemId, Vec<Symbol>>,
   |                            ~~~~~~~~~~~~~~~~~~~~
70 |     exact_paths: FxHashMap<crate::core::ImplTraitParam, Vec<Symbol>>,
help: you might be missing a type parameter
   |
   |
64 | crate struct RustdocVisitor<'a, 'tcx, DefId> {

error[E0412]: cannot find type `Symbol` in this scope
  --> src/librustdoc/visit_ast.rs:70:39
   |
   |
64 | crate struct RustdocVisitor<'a, 'tcx> {
   |                                     - help: you might be missing a type parameter: `, Symbol`
...
70 |     exact_paths: FxHashMap<DefId, Vec<Symbol>>,

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/visit_ast.rs:87:35
   |
   |
87 |     fn store_path(&mut self, did: DefId) {
   |                                   ^^^^^ not found in this scope
   |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
   |
87 |     fn store_path(&mut self, did: crate::clean::ItemId) {
   |                                   ~~~~~~~~~~~~~~~~~~~~
87 |     fn store_path(&mut self, did: crate::core::ImplTraitParam) {

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/visit_ast.rs:155:15
    |
---
    |
182 |         res: Res,
    |              ^^^ not found in this scope
    |
note: enum `crate::collect_intra_doc_links::Res` exists but is inaccessible
    |
62  | enum Res {
    | ^^^^^^^^ not accessible


error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/visit_ast.rs:183:25
    |
73  | impl<'a, 'tcx> RustdocVisitor<'a, 'tcx> {
    |              - help: you might be missing a type parameter: `, Symbol`
...
183 |         renamed: Option<Symbol>,

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/visit_ast.rs:269:25
    |
    |
73  | impl<'a, 'tcx> RustdocVisitor<'a, 'tcx> {
    |              - help: you might be missing a type parameter: `, Symbol`
...
269 |         renamed: Option<Symbol>,

error[E0412]: cannot find type `Symbol` in this scope
   --> src/librustdoc/visit_ast.rs:381:25
    |
    |
73  | impl<'a, 'tcx> RustdocVisitor<'a, 'tcx> {
    |              - help: you might be missing a type parameter: `, Symbol`
...
381 |         renamed: Option<Symbol>,

error[E0412]: cannot find type `TyCtxt` in this scope
  --> src/librustdoc/visit_lib.rs:12:10
   |
   |
12 |     tcx: TyCtxt<'tcx>,
   |          ^^^^^^ not found in this scope

error[E0412]: cannot find type `AccessLevels` in this scope
  --> src/librustdoc/visit_lib.rs:14:28
   |
14 |     access_levels: &'a mut AccessLevels<DefId>,

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/visit_lib.rs:14:41
   |
   |
14 |     access_levels: &'a mut AccessLevels<DefId>,
   |
   |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
   |
14 |     access_levels: &'a mut AccessLevels<crate::clean::ItemId>,
   |                                         ~~~~~~~~~~~~~~~~~~~~
14 |     access_levels: &'a mut AccessLevels<crate::core::ImplTraitParam>,
help: you might be missing a type parameter
   |
   |
11 | crate struct LibEmbargoVisitor<'a, 'tcx, DefId> {


error[E0412]: cannot find type `AccessLevel` in this scope
  --> src/librustdoc/visit_lib.rs:16:24
   |
11 | crate struct LibEmbargoVisitor<'a, 'tcx> {
   |                                        - help: you might be missing a type parameter: `, AccessLevel`
...
16 |     prev_level: Option<AccessLevel>,


error[E0412]: cannot find type `FxHashSet` in this scope
  --> src/librustdoc/visit_lib.rs:18:19
   |
18 |     visited_mods: FxHashSet<DefId>,

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/visit_lib.rs:18:29
   |
   |
18 |     visited_mods: FxHashSet<DefId>,
   |
   |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
   |
18 |     visited_mods: FxHashSet<crate::clean::ItemId>,
   |                             ~~~~~~~~~~~~~~~~~~~~
18 |     visited_mods: FxHashSet<crate::core::ImplTraitParam>,
help: you might be missing a type parameter
   |
   |
11 | crate struct LibEmbargoVisitor<'a, 'tcx, DefId> {

error[E0412]: cannot find type `CrateNum` in this scope
  --> src/librustdoc/visit_lib.rs:31:41
   |
   |
31 |     crate fn visit_lib(&mut self, cnum: CrateNum) {

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/visit_lib.rs:38:31
   |
   |
38 |     fn update(&mut self, did: DefId, level: Option<AccessLevel>) -> Option<AccessLevel> {
   |
   |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
   |
38 |     fn update(&mut self, did: crate::clean::ItemId, level: Option<AccessLevel>) -> Option<AccessLevel> {
   |                               ~~~~~~~~~~~~~~~~~~~~
38 |     fn update(&mut self, did: crate::core::ImplTraitParam, level: Option<AccessLevel>) -> Option<AccessLevel> {


error[E0412]: cannot find type `AccessLevel` in this scope
  --> src/librustdoc/visit_lib.rs:38:52
   |
21 | impl<'a, 'tcx> LibEmbargoVisitor<'a, 'tcx> {
   |              - help: you might be missing a type parameter: `, AccessLevel`
...
38 |     fn update(&mut self, did: DefId, level: Option<AccessLevel>) -> Option<AccessLevel> {


error[E0412]: cannot find type `AccessLevel` in this scope
  --> src/librustdoc/visit_lib.rs:38:76
   |
21 | impl<'a, 'tcx> LibEmbargoVisitor<'a, 'tcx> {
   |              - help: you might be missing a type parameter: `, AccessLevel`
...
38 |     fn update(&mut self, did: DefId, level: Option<AccessLevel>) -> Option<AccessLevel> {

error[E0412]: cannot find type `DefId` in this scope
  --> src/librustdoc/visit_lib.rs:51:43
   |
   |
51 |     crate fn visit_mod(&mut self, def_id: DefId) {
   |                                           ^^^^^ not found in this scope
   |
help: there is an enum variant `crate::clean::ItemId::DefId` and 1 other; try using the variant's enum
   |
51 |     crate fn visit_mod(&mut self, def_id: crate::clean::ItemId) {
   |                                           ~~~~~~~~~~~~~~~~~~~~
51 |     crate fn visit_mod(&mut self, def_id: crate::core::ImplTraitParam) {

error[E0412]: cannot find type `Res` in this scope
  --> src/librustdoc/visit_lib.rs:67:35
   |
   |
67 |     fn visit_item(&mut self, res: Res<!>) {
   |
   |
note: enum `crate::collect_intra_doc_links::Res` exists but is inaccessible
   |
62 | enum Res {
   | ^^^^^^^^ not accessible


error[E0412]: cannot find type `RustcOptGroup` in this scope
   --> src/librustdoc/lib.rs:239:18
    |
239 | fn opts() -> Vec<RustcOptGroup> {
    |        -         ^^^^^^^^^^^^^ not found in this scope
    |        |
    |        help: you might be missing a type parameter: `<RustcOptGroup>`
error[E0412]: cannot find type `ErrorGuaranteed` in this scope
   --> src/librustdoc/lib.rs:676:30
    |
676 | type MainResult = Result<(), ErrorGuaranteed>;
676 | type MainResult = Result<(), ErrorGuaranteed>;
    |                -             ^^^^^^^^^^^^^^^ not found in this scope
    |                |
    |                help: you might be missing a type parameter: `<ErrorGuaranteed>`
error[E0412]: cannot find type `TyCtxt` in this scope
   --> src/librustdoc/lib.rs:725:10
    |
725 |     tcx: TyCtxt<'tcx>,
725 |     tcx: TyCtxt<'tcx>,
    |          ^^^^^^ not found in this scope

thread 'rustc' panicked at 'internal error: entered unreachable code', compiler/rustc_resolve/src/lib.rs:1834:67
stack backtrace:
   0:     0x7f919bf11b72 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h40f8f3daa23f431e
   1:     0x7f919bf774a8 - core::fmt::write::h42d747535ccbaf5b
   2:     0x7f919bf02031 - std::io::Write::write_fmt::h495dbdc99589b77c
   3:     0x7f919bf14ea6 - std::panicking::default_hook::{{closure}}::h1f2cfb3a6707d9fe
   4:     0x7f919bf14aa5 - std::panicking::default_hook::h07bf01758e423d7c
   5:     0x7f919d0828fa - rustc_driver[9c3f2120bbf7371d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f918f29eb09 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hff3a3945c56c0b18
   7:     0x7f918f2ca3fb - proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::{{closure}}::hd8f4bb99054a635a
   8:     0x7f919bf157cd - std::panicking::rust_panic_with_hook::h85a296a572b16b0f
   9:     0x7f919bf15619 - std::panicking::begin_panic_handler::{{closure}}::h1b6f091ffeaf8dfa
  10:     0x7f919bf12114 - std::sys_common::backtrace::__rust_end_short_backtrace::h6654ca9f5c52ca6c
  11:     0x7f919bf15349 - rust_begin_unwind
  12:     0x7f919bec80e3 - core::panicking::panic_fmt::h7e15babac27fb630
  13:     0x7f919bec7fad - core::panicking::panic::h59ba1f41023369f0
  14:     0x7f91a144590b - <rustc_resolve[5e2eeef38d5b363f]::Resolver>::resolve_rustdoc_path
  15:     0x55d80d768ac0 - <rustdoc[234ebe4e4ad05f8c]::passes::collect_intra_doc_links::early::EarlyDocLinkResolver>::load_links_in_attrs
  16:     0x55d80d769144 - <rustdoc[234ebe4e4ad05f8c]::passes::collect_intra_doc_links::early::EarlyDocLinkResolver as rustc_ast[70b27e4311c19868]::visit::Visitor>::visit_item
  17:     0x55d80d7776fa - rustc_ast[70b27e4311c19868]::visit::walk_item::<rustdoc[234ebe4e4ad05f8c]::passes::collect_intra_doc_links::early::EarlyDocLinkResolver>
  18:     0x55d80d76918e - <rustdoc[234ebe4e4ad05f8c]::passes::collect_intra_doc_links::early::EarlyDocLinkResolver as rustc_ast[70b27e4311c19868]::visit::Visitor>::visit_item
  19:     0x55d80d7776fa - rustc_ast[70b27e4311c19868]::visit::walk_item::<rustdoc[234ebe4e4ad05f8c]::passes::collect_intra_doc_links::early::EarlyDocLinkResolver>
  20:     0x55d80d76918e - <rustdoc[234ebe4e4ad05f8c]::passes::collect_intra_doc_links::early::EarlyDocLinkResolver as rustc_ast[70b27e4311c19868]::visit::Visitor>::visit_item
  21:     0x55d80d70d72a - <rustc_interface[4431ea57bd42aced]::passes::boxed_resolver::BoxedResolver>::access::<rustdoc[234ebe4e4ad05f8c]::main_options::{closure#0}::{closure#0}::{closure#0}, rustdoc[234ebe4e4ad05f8c]::core::ResolverCaches>
  22:     0x55d80d5fc001 - <rustc_interface[4431ea57bd42aced]::interface::Compiler>::enter::<rustdoc[234ebe4e4ad05f8c]::main_options::{closure#0}::{closure#0}, core[734429170246c524]::result::Result<(), rustc_errors[65f611344d5dc886]::ErrorGuaranteed>>
  23:     0x55d80d412296 - rustc_span[c84b1e53812dd887]::with_source_map::<core[734429170246c524]::result::Result<(), rustc_errors[65f611344d5dc886]::ErrorGuaranteed>, rustc_interface[4431ea57bd42aced]::interface::create_compiler_and_run<core[734429170246c524]::result::Result<(), rustc_errors[65f611344d5dc886]::ErrorGuaranteed>, rustdoc[234ebe4e4ad05f8c]::main_options::{closure#0}>::{closure#1}>
  24:     0x55d80d5ff063 - rustc_interface[4431ea57bd42aced]::interface::create_compiler_and_run::<core[734429170246c524]::result::Result<(), rustc_errors[65f611344d5dc886]::ErrorGuaranteed>, rustdoc[234ebe4e4ad05f8c]::main_options::{closure#0}>
  25:     0x55d80d44b7da - rustdoc[234ebe4e4ad05f8c]::main_options
  26:     0x55d80d7af2db - <scoped_tls[9d249f0f2ce07b71]::ScopedKey<rustc_span[c84b1e53812dd887]::SessionGlobals>>::set::<rustdoc[234ebe4e4ad05f8c]::main_args::{closure#0}, core[734429170246c524]::result::Result<(), rustc_errors[65f611344d5dc886]::ErrorGuaranteed>>
  27:     0x55d80d64eb2f - std[801cb7f74ea44579]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4431ea57bd42aced]::util::run_in_thread_pool_with_globals<rustdoc[234ebe4e4ad05f8c]::main_args::{closure#0}, core[734429170246c524]::result::Result<(), rustc_errors[65f611344d5dc886]::ErrorGuaranteed>>::{closure#0}, core[734429170246c524]::result::Result<(), rustc_errors[65f611344d5dc886]::ErrorGuaranteed>>
  28:     0x55d80d6f7769 - <<std[801cb7f74ea44579]::thread::Builder>::spawn_unchecked_<rustc_interface[4431ea57bd42aced]::util::run_in_thread_pool_with_globals<rustdoc[234ebe4e4ad05f8c]::main_args::{closure#0}, core[734429170246c524]::result::Result<(), rustc_errors[65f611344d5dc886]::ErrorGuaranteed>>::{closure#0}, core[734429170246c524]::result::Result<(), rustc_errors[65f611344d5dc886]::ErrorGuaranteed>>::{closure#1} as core[734429170246c524]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  29:     0x7f919bf21f83 - std::sys::unix::thread::Thread::new::thread_start::hadd5bbf9df9317ed
  30:     0x7f919be3f609 - start_thread
  31:     0x7f919bbf8163 - clone
  32:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0 (23a96e02d 2022-04-13) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C symbol-mangling-version=v0 -Z unstable-options -Z unstable-options
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
Some errors have detailed explanations: E0405, E0412, E0425, E0432, E0433, E0464.
For more information about an error, try `rustc --explain E0405`.
error: could not document `rustdoc`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustdoc src/librustdoc/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=e171705f8e955b6d -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps --extern arrayvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libarrayvec-0a14e709d5c8eb6a.rmeta --extern askama=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaskama-b79ba7b89db3d38b.rmeta --extern atty=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libatty-beb994db994a7db7.rmeta --extern itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-78f9c70ffb52a7fe.rmeta --extern minifier=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libminifier-1872c70c516cdb05.rmeta --extern pulldown_cmark=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libpulldown_cmark-2cc3bf05442a0d37.rmeta --extern rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librayon-5fbe863dea921106.rmeta --extern regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-fd316442922c7c8e.rmeta --extern rustdoc_json_types=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustdoc_json_types-2bb3de22974b9852.rmeta --extern serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-51e1241cabb67336.rmeta --extern serde_json=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde_json-552643f7390a6095.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsmallvec-e973f72b40fb3ab3.rmeta --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtempfile-770a1467502b1170.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtracing-ef83ae9eae6c2630.rmeta --extern tracing_subscriber=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtracing_subscriber-9f558f6705d9d3da.rmeta --extern tracing_tree=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtracing_tree-d2a126c70d07f07a.rmeta -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=names()' '--check-cfg=values()' '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(release)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.62.0
  (23a96e02d
  2022-04-13)' --document-private-items --enable-index-page --show-type-layout --generate-link-to-definition -Zunstable-options` (exit status: 101)
