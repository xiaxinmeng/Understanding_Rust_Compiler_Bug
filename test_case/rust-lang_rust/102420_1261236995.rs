plain
    Checking rustc_lint v0.0.0 (/checkout/compiler/rustc_lint)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error: unused imports: `LateContext`, `LintContext`
  |
  |
1 | use crate::{LateContext, LateLintPass, LintContext};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`

error: unused imports: `Applicability`, `fluent`
  |
  |
2 | use rustc_errors::{fluent, Applicability};

error: unused import: `rustc_hir as hir`
 --> compiler/rustc_lint/src/array_into_iter.rs:3:5
  |
---

error: unused imports: `Adjust`, `Adjustment`
 --> compiler/rustc_lint/src/array_into_iter.rs:5:36
  |
5 | use rustc_middle::ty::adjustment::{Adjust, Adjustment};

error: unused import: `rustc_span::symbol::sym`
 --> compiler/rustc_lint/src/array_into_iter.rs:8:5
  |
  |
8 | use rustc_span::symbol::sym;
  |     ^^^^^^^^^^^^^^^^^^^^^^^

error: unused import: `errors::BuiltinEllpisisInclusiveRangePatterns`
   |
   |
24 |     errors::BuiltinEllpisisInclusiveRangePatterns,

error: unused import: `rustc_ast::attr`
  --> compiler/rustc_lint/src/builtin.rs:28:5
   |
   |
28 | use rustc_ast::attr;
   |     ^^^^^^^^^^^^^^^

error: unused imports: `FnCtxt`, `FnKind`
   |
   |
30 | use rustc_ast::visit::{FnCtxt, FnKind};


error: unused imports: `expr_to_string`, `self`
   |
   |
32 | use rustc_ast_pretty::pprust::{self, expr_to_string};


error: unused imports: `DiagnosticStyledString`, `MultiSpan`
   |
   |
36 |     fluent, Applicability, Diagnostic, DiagnosticMessage, DiagnosticStyledString,
37 |     LintDiagnosticBuilder, MultiSpan,
   |                            ^^^^^^^^^


error: unused imports: `AttributeGate`, `GateIssue`, `Stability`
   |
   |
39 | use rustc_feature::{deprecated_attributes, AttributeGate, BuiltinAttribute, GateIssue, Stability};


error: unused imports: `ForeignItemKind`, `GenericParamKind`, `PatKind`, `PredicateOrigin`
   |
   |
43 | use rustc_hir::{ForeignItemKind, GenericParamKind, HirId, PatKind, PredicateOrigin};

error: unused import: `rustc_middle::ty::print::with_no_trimmed_paths`
  --> compiler/rustc_lint/src/builtin.rs:47:5
   |
   |
47 | use rustc_middle::ty::print::with_no_trimmed_paths;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: unused import: `VariantDef`
  --> compiler/rustc_lint/src/builtin.rs:49:52
   |
49 | use rustc_middle::ty::{self, Instance, Ty, TyCtxt, VariantDef};

error: unused import: `BuiltinLintDiagnostics`
  --> compiler/rustc_lint/src/builtin.rs:50:27
   |
   |
50 | use rustc_session::lint::{BuiltinLintDiagnostics, FutureIncompatibilityReason};

error: unused import: `rustc_span::source_map::Spanned`
  --> compiler/rustc_lint/src/builtin.rs:52:5
   |
   |
52 | use rustc_span::source_map::Spanned;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: unused imports: `BytePos`, `InnerSpan`
   |
   |
54 | use rustc_span::{BytePos, InnerSpan, Span};


error: unused imports: `misc::can_type_implement_copy`, `self`
   |
   |
56 | use rustc_trait_selection::traits::{self, misc::can_type_implement_copy};


error: unused imports: `MethodLateContext`, `method_context`
   |
   |
58 | use crate::nonstandard_style::{method_context, MethodLateContext};

error: unused import: `std::fmt::Write`
  --> compiler/rustc_lint/src/builtin.rs:60:5
   |
   |
60 | use std::fmt::Write;
   |     ^^^^^^^^^^^^^^^

error: unused macro definition: `expand_early_lint_pass_impl_methods`
   --> compiler/rustc_lint/src/early.rs:300:14
    |
300 | macro_rules! expand_early_lint_pass_impl_methods {
    |
    |
    = note: `-D unused-macros` implied by `-D warnings`
error: unused import: `symbol::sym`
 --> compiler/rustc_lint/src/enum_intrinsics_non_enums.rs:5:18
  |
  |
5 | use rustc_span::{symbol::sym, Span};

error: unused import: `contains_text_flow_control_chars`
 --> compiler/rustc_lint/src/hidden_unicode_codepoints.rs:2:26
  |
  |
2 | use ast::util::unicode::{contains_text_flow_control_chars, TEXT_FLOW_CONTROL_CHARS};

error: unused import: `EarlyContext`
 --> compiler/rustc_lint/src/internal.rs:4:13
  |
  |
4 | use crate::{EarlyContext, EarlyLintPass, LateContext, LateLintPass, LintContext};

error: unused import: `Applicability`
 --> compiler/rustc_lint/src/internal.rs:6:28
  |
  |
6 | use rustc_errors::{fluent, Applicability};


error: unused imports: `PatKind`, `QPath`
  |
  |
8 | use rustc_hir::{def_id::DefId, Expr, ExprKind, GenericArg, PatKind, Path, PathSegment, QPath};


error: unused imports: `Pat`, `TyKind`, `Ty`
  |
  |
9 | use rustc_hir::{HirId, Impl, Item, ItemKind, Node, Pat, Ty, TyKind};
  |                                                    ^^^  ^^  ^^^^^^

error: unused imports: `ExpnKind`, `MacroKind`
   |
   |
12 | use rustc_span::hygiene::{ExpnKind, MacroKind};


error: unused imports: `LateContext`, `LintContext`
  |
  |
1 | use crate::{LateContext, LateLintPass, LintContext};

error: unused import: `MultiSpan`
 --> compiler/rustc_lint/src/let_underscore.rs:2:58
  |
  |
2 | use rustc_errors::{Applicability, LintDiagnosticBuilder, MultiSpan};

error: unused import: `rustc_middle::ty`
 --> compiler/rustc_lint/src/let_underscore.rs:4:5
  |
  |
4 | use rustc_middle::ty;
  |     ^^^^^^^^^^^^^^^^

error: unused imports: `EarlyContext`, `LintContext`
  |
  |
1 | use crate::{EarlyContext, EarlyLintPass, LintContext};

error: unused import: `rustc_ast as ast`
 --> compiler/rustc_lint/src/non_ascii_idents.rs:2:5
  |
---
  |
3 | use rustc_attr as attr;
  |     ^^^^^^^^^^^^^^^^^^

error: unused imports: `DefKind`, `Res`
  |
  |
6 | use rustc_hir::def::{DefKind, Res};

error: unused import: `rustc_hir::intravisit::FnKind`
 --> compiler/rustc_lint/src/nonstandard_style.rs:7:5
  |
  |
7 | use rustc_hir::intravisit::FnKind;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: unused imports: `GenericParamKind`, `PatKind`
  |
  |
8 | use rustc_hir::{GenericParamKind, PatKind};

error: unused import: `rustc_span::symbol::sym`
  --> compiler/rustc_lint/src/nonstandard_style.rs:10:5
   |
   |
10 | use rustc_span::symbol::sym;
   |     ^^^^^^^^^^^^^^^^^^^^^^^

error: unused imports: `BytePos`, `Span`
  --> compiler/rustc_lint/src/nonstandard_style.rs:11:33
   |
11 | use rustc_span::{symbol::Ident, BytePos, Span};

error: unused import: `rustc_target::spec::abi::Abi`
  --> compiler/rustc_lint/src/nonstandard_style.rs:12:5
   |
---

error: unused imports: `ExprKind`, `Expr`
 --> compiler/rustc_lint/src/noop_method_call.rs:7:17
  |
7 | use rustc_hir::{Expr, ExprKind};

error: unused import: `rustc_middle::ty`
 --> compiler/rustc_lint/src/noop_method_call.rs:8:5
  |
---
  |
9 | use rustc_span::symbol::sym;
  |     ^^^^^^^^^^^^^^^^^^^^^^^

error: unused imports: `Applicability`, `fluent`
  |
  |
2 | use rustc_errors::{fluent, Applicability};


error: unused imports: `Block`, `StmtKind`
  |
  |
2 | use rustc_ast::{Block, StmtKind};

error: unused import: `crate::LateContext`
 --> compiler/rustc_lint/src/traits.rs:1:5
  |
---
   |
12 | use rustc_span::source_map;
   |     ^^^^^^^^^^^^^^^^^^^^^^

error: unused imports: `TagEncoding`, `Variants`
   |
   |
16 | use rustc_target::abi::{Integer, TagEncoding, Variants};

error: unused import: `std::cmp`
  --> compiler/rustc_lint/src/types.rs:19:5
   |
---

error: unused import: `LintContext`
 --> compiler/rustc_lint/src/pass_by_value.rs:1:40
  |
1 | use crate::{LateContext, LateLintPass, LintContext};


error: field `KeywordIdents` is never read
    |
123 | / macro_rules! pre_expansion_lint_passes {
123 | / macro_rules! pre_expansion_lint_passes {
124 | |     ($macro:path, $args:tt) => {
125 | |         $macro!($args, [KeywordIdents: KeywordIdents,]);
126 | |     };
127 | | }
127 | | }
    | |_- in this expansion of `pre_expansion_lint_passes!` (#1)
...
155 | / macro_rules! declare_combined_early_pass {
156 | |     ([$name:ident], $passes:tt) => (
157 |           early_lint_methods!(declare_combined_early_lint_pass, [pub $name, $passes]);
158 |       )
159 | | }
    | | -
    | |_|
    | |_|
    |   in this expansion of `declare_combined_early_pass!` (#2)
160 | |
161 | | pre_expansion_lint_passes!(declare_combined_early_pass, [BuiltinCombinedPreExpansionLintPass]);
    | | |                                                        |
    | | |                                                        field in this struct
    | | in this macro invocation (#1)
...   |
...   |
173 | |                 // Builds a global list of all impls of `Debug`.
174 | |                 // FIXME: Turn the computation of types which implement Debug into a query
    |
   ::: compiler/rustc_lint/src/passes.rs:211:1
    |
211 | / macro_rules! declare_combined_early_lint_pass {
211 | / macro_rules! declare_combined_early_lint_pass {
212 | |     ([$v:vis $name:ident, [$($passes:ident: $constructor:expr,)*]], $methods:tt) => (
213 | |         #[allow(non_snake_case)]
214 | |         $v struct $name {
242 | |     )
243 | | }
243 | | }
    | |_- in this expansion of `declare_combined_early_lint_pass!` (#3)
    |
    = note: `-D dead-code` implied by `-D warnings`
error: multiple fields are never read
   --> compiler/rustc_lint/src/lib.rs:134:17
    |
129 |                   macro_rules! early_lint_passes {
---
    |                 |_|
    |                 |_|
    |                 |_|
    |                 |
130 |                 |     ($macro:path, $args:tt) => {
131 |                 |         $macro!(
    | |_______________|________________|
    | |_______________|________________|
    | |_______________|________________|
    | |_______________|________________|
---
    | |_______________|________________|
    | |               |
132 | |               |             $args,
133 | |               |             [
134 | |               |                 UnusedParens: UnusedParens,
    | |               |                 ^^^^^^^^^^^^
135 | |               |                 UnusedBraces: UnusedBraces,
    | |               |                 ^^^^^^^^^^^^
136 | |               |                 UnusedImportBraces: UnusedImportBraces,
    | |               |                 ^^^^^^^^^^^^^^^^^^
137 | |               |                 UnsafeCode: UnsafeCode,
    | |               |                 ^^^^^^^^^^
138 | |               |                 SpecialModuleName: SpecialModuleName,
    | |               |                 ^^^^^^^^^^^^^^^^^
139 | |               |                 AnonymousParameters: AnonymousParameters,
    | |               |                 ^^^^^^^^^^^^^^^^^^^
140 | |               |                 EllipsisInclusiveRangePatterns: EllipsisInclusiveRangePatterns::default(),
    | |               |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
141 | |               |                 NonCamelCaseTypes: NonCamelCaseTypes,
142 | |               |                 DeprecatedAttr: DeprecatedAttr::new(),
    | |               |                 ^^^^^^^^^^^^^^
    | |               |                 ^^^^^^^^^^^^^^
143 | |               |                 WhileTrue: WhileTrue,
    | |               |                 ^^^^^^^^^
144 | |               |                 NonAsciiIdents: NonAsciiIdents,
    | |               |                 ^^^^^^^^^^^^^^
145 | |               |                 HiddenUnicodeCodepoints: HiddenUnicodeCodepoints,
146 | |               |                 IncompleteFeatures: IncompleteFeatures,
    | |               |                 ^^^^^^^^^^^^^^^^^^
    | |               |                 ^^^^^^^^^^^^^^^^^^
147 | |               |                 RedundantSemicolons: RedundantSemicolons,
    | |               |                 ^^^^^^^^^^^^^^^^^^^
148 | |               |                 UnusedDocComment: UnusedDocComment,
    | |               |                 ^^^^^^^^^^^^^^^^
149 | |               |                 UnexpectedCfgs: UnexpectedCfgs,
150 | |               |             ]
151 | |               |         );
    | |               |         -
    | |_______________|_________|
---
    |                 |         in this macro invocation (#2)
152 | |               |     };
153 | |               | }
    | |               | -
    | |               |_|
    | |               |_in this expansion of `early_lint_passes!` (#1)
    | |               |_in this expansion of `early_lint_passes!` (#1)
    | |               |_in this expansion of `early_lint_passes!` (#1)
    | |               |_in this expansion of `early_lint_passes!` (#1)
    | |               |_in this expansion of `early_lint_passes!` (#1)
    | |               |_in this expansion of `early_lint_passes!` (#1)
    | |               |_in this expansion of `early_lint_passes!` (#1)
    | |               |_in this expansion of `early_lint_passes!` (#1)
    | |               |_in this expansion of `early_lint_passes!` (#1)
    | |               |_in this expansion of `early_lint_passes!` (#1)
    | |               |_in this expansion of `early_lint_passes!` (#1)
    | |               |_in this expansion of `early_lint_passes!` (#1)
    | |               |_in this expansion of `early_lint_passes!` (#1)
    | |               |_in this expansion of `early_lint_passes!` (#1)
    | |               |_in this expansion of `early_lint_passes!` (#1)
    | |                 in this expansion of `early_lint_passes!` (#1)
154 | |
155 |                 | macro_rules! declare_combined_early_pass {
    |  _______________|_-
    | |_______________|_|
    | |_______________|_|
    | |_______________|_|
    | |_______________|_|
    | |_______________|_|
    | |_______________|_|
    | |_______________|_|
    | |_______________|_|
    | |_______________|_|
    | |_______________|_|
    | |_______________|_|
    | |_______________|_|
    | |_______________|_|
    | |_______________|_|
    | |_______________|_|
    | |               |
156 | |               |     ([$name:ident], $passes:tt) => (
157 |                 |         early_lint_methods!(declare_combined_early_lint_pass, [pub $name, $passes]);
    | |_______________|_____________________________|
    | |_______________|_____________________________|
    | |_______________|_____________________________|
    | |_______________|_____________________________|
---
    | |               |
158 | |               |     )
159 | |               | }
    | |               | -
    | |_______________|_|
    | |_______________|_in this expansion of `declare_combined_early_pass!` (#2)
    | |_______________|_in this expansion of `declare_combined_early_pass!` (#2)
    | |_______________|_in this expansion of `declare_combined_early_pass!` (#2)
    | |_______________|_in this expansion of `declare_combined_early_pass!` (#2)
    | |_______________|_in this expansion of `declare_combined_early_pass!` (#2)
    | |_______________|_in this expansion of `declare_combined_early_pass!` (#2)
    | |_______________|_in this expansion of `declare_combined_early_pass!` (#2)
    | |_______________|_in this expansion of `declare_combined_early_pass!` (#2)
    | |_______________|_in this expansion of `declare_combined_early_pass!` (#2)
    | |_______________|_in this expansion of `declare_combined_early_pass!` (#2)
    | |_______________|_in this expansion of `declare_combined_early_pass!` (#2)
    | |_______________|_in this expansion of `declare_combined_early_pass!` (#2)
    | |_______________|_in this expansion of `declare_combined_early_pass!` (#2)
    | |_______________|_in this expansion of `declare_combined_early_pass!` (#2)
    | |_______________|_in this expansion of `declare_combined_early_pass!` (#2)
    |                 | in this expansion of `declare_combined_early_pass!` (#2)
160 | |               |
161 | |               | pre_expansion_lint_passes!(declare_combined_early_pass, [BuiltinCombinedPreExpansionLintPass]);
162 | |               | early_lint_passes!(declare_combined_early_pass, [BuiltinCombinedEarlyLintPass]);
    | |               | |                                                |
    | |               | |                                                fields in this struct
    | |               | in this macro invocation (#1)
    | |               | in this macro invocation (#1)
---
    | |               | in this macro invocation (#1)
    | |               | in this macro invocation (#1)
    | |               | in this macro invocation (#1)
...   |               |
173 | |               |                 // Builds a global list of all impls of `Debug`.
174 | |               |                 // FIXME: Turn the computation of types which implement Debug into a query
    | |_______________|__________|
    | |_______________|__________in this macro invocation (#3)
    | |_______________|__________in this macro invocation (#3)
    | |_______________|__________in this macro invocation (#3)
---
    | |_________________|
    | |_________________|
    | |_________________|
    | |
212 | |                     ([$v:vis $name:ident, [$($passes:ident: $constructor:expr,)*]], $methods:tt) => (
213 | |                         #[allow(non_snake_case)]
214 | |                         $v struct $name {
242 | |                     )
243 | |                 }
    | |                 -
    | |_________________|
    | |_________________|
    | |_________________in this expansion of `declare_combined_early_lint_pass!` (#3)
    | |_________________in this expansion of `declare_combined_early_lint_pass!` (#3)
    | |_________________in this expansion of `declare_combined_early_lint_pass!` (#3)
    | |_________________in this expansion of `declare_combined_early_lint_pass!` (#3)
    | |_________________in this expansion of `declare_combined_early_lint_pass!` (#3)
    | |_________________in this expansion of `declare_combined_early_lint_pass!` (#3)
    | |_________________in this expansion of `declare_combined_early_lint_pass!` (#3)
    | |_________________in this expansion of `declare_combined_early_lint_pass!` (#3)
    | |_________________in this expansion of `declare_combined_early_lint_pass!` (#3)
    | |_________________in this expansion of `declare_combined_early_lint_pass!` (#3)
    | |_________________in this expansion of `declare_combined_early_lint_pass!` (#3)
    | |_________________in this expansion of `declare_combined_early_lint_pass!` (#3)
    | |_________________in this expansion of `declare_combined_early_lint_pass!` (#3)
    | |_________________in this expansion of `declare_combined_early_lint_pass!` (#3)
    | |_________________in this expansion of `declare_combined_early_lint_pass!` (#3)
    |                   in this expansion of `declare_combined_early_lint_pass!` (#3)

error: fields `UnnameableTestItems`, `MissingDoc`, `MissingDebugImplementations` and `ClashingExternDeclarations` are never read
    |
    |
56  | | mod internal;
    | |__________|
    | |__________in this macro invocation (#3)
    | |__________in this macro invocation (#3)
    | |__________in this macro invocation (#3)
---
    | |_|
    | |_|
    | |_|
    | |
165 | |     ($macro:path, $args:tt) => {
166 |           $macro!(
    | |________________|
    | |________________|
    | |________________|
    | |
    | |
167 | |             $args,
168 | |             [
169 | |                 // Tracks state across modules
170 | |                 UnnameableTestItems: UnnameableTestItems::new(),
171 | |                 // Tracks attributes of parents
171 | |                 // Tracks attributes of parents
172 | |                 MissingDoc: MissingDoc::new(),
...   |
...   |
176 | |                 MissingDebugImplementations: MissingDebugImplementations::default(),
    | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^
177 | |                 // Keeps a global list of foreign declarations.
178 | |                 ClashingExternDeclarations: ClashingExternDeclarations::new(),
179 | |             ]
180 | |         );
    | |         -
    | |_________|
---
181 | |     };
182 | | }
    | | -
    | |_|
    | |_in this expansion of `late_lint_passes!` (#1)
    | |_in this expansion of `late_lint_passes!` (#1)
    | |_in this expansion of `late_lint_passes!` (#1)
    |   in this expansion of `late_lint_passes!` (#1)
231 |   macro_rules! declare_combined_late_pass {
    |  _-
    | |_|
    | |_|
    | |_|
    | |_|
    | |
232 | |     ([$v:vis $name:ident], $passes:tt) => (
233 |           late_lint_methods!(declare_combined_late_lint_pass, [$v $name, $passes], ['tcx]);
    | |____________________________|
    | |____________________________|
    | |____________________________|
    | |
    | |
234 | |     )
235 | | }
    | | -
    | |_|
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    |   in this expansion of `declare_combined_late_pass!` (#2)
...
238 | | late_lint_passes!(declare_combined_late_pass, [pub BuiltinCombinedLateLintPass]);
    | | |                                                  |
    | | |                                                  fields in this struct
    | | in this macro invocation (#1)
    | | in this macro invocation (#1)
---
    | |_|
    | |_|
    | |_|
    | |
105 | |     ([$v:vis $name:ident, [$($passes:ident: $constructor:expr,)*]], [$hir:tt], $methods:tt) => (
106 | |         #[allow(non_snake_case)]
107 | |         $v struct $name {
135 | |     )
136 | | }
    | | -
    | |_|
    | |_|
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    |   in this expansion of `declare_combined_late_lint_pass!` (#3)
error: multiple fields are never read
   --> compiler/rustc_lint/src/lib.rs:189:17
    |
    |
56  | | mod internal;
    | |__________|
    | |__________in this macro invocation (#3)
    | |__________in this macro invocation (#3)
    | |__________in this macro invocation (#3)
---
    | |_|
    | |_|
    | |_|
    | |
185 | |     ($macro:path, $args:tt) => {
186 |           $macro!(
    | |________________|
    | |________________|
    | |________________|
    | |________________|
---
    | |________________|
    | |
187 | |             $args,
188 | |             [
189 | |                 HardwiredLints: HardwiredLints,
    | |                 ^^^^^^^^^^^^^^
190 | |                 ImproperCTypesDeclarations: ImproperCTypesDeclarations,
    | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
191 | |                 ImproperCTypesDefinitions: ImproperCTypesDefinitions,
192 | |                 VariantSizeDifferences: VariantSizeDifferences,
    | |                 ^^^^^^^^^^^^^^^^^^^^^^
    | |                 ^^^^^^^^^^^^^^^^^^^^^^
193 | |                 BoxPointers: BoxPointers,
194 | |                 PathStatements: PathStatements,
    | |                 ^^^^^^^^^^^^^^
    | |                 ^^^^^^^^^^^^^^
195 | |                 LetUnderscore: LetUnderscore,
196 | |                 // Depends on referenced function signatures in expressions
197 | |                 UnusedResults: UnusedResults,
    | |                 ^^^^^^^^^^^^^
    | |                 ^^^^^^^^^^^^^
198 | |                 NonUpperCaseGlobals: NonUpperCaseGlobals,
    | |                 ^^^^^^^^^^^^^^^^^^^
199 | |                 NonShorthandFieldPatterns: NonShorthandFieldPatterns,
200 | |                 UnusedAllocation: UnusedAllocation,
    | |                 ^^^^^^^^^^^^^^^^
201 | |                 // Depends on types used in type definitions
201 | |                 // Depends on types used in type definitions
202 | |                 MissingCopyImplementations: MissingCopyImplementations,
203 | |                 // Depends on referenced function signatures in expressions
204 | |                 MutableTransmutes: MutableTransmutes,
    | |                 ^^^^^^^^^^^^^^^^^
    | |                 ^^^^^^^^^^^^^^^^^
205 | |                 TypeAliasBounds: TypeAliasBounds,
206 | |                 TrivialConstraints: TrivialConstraints,
    | |                 ^^^^^^^^^^^^^^^^^^
207 | |                 TypeLimits: TypeLimits::new(),
    | |                 ^^^^^^^^^^
    | |                 ^^^^^^^^^^
208 | |                 NonSnakeCase: NonSnakeCase,
    | |                 ^^^^^^^^^^^^
209 | |                 InvalidNoMangleItems: InvalidNoMangleItems,
210 | |                 // Depends on access levels
210 | |                 // Depends on access levels
211 | |                 UnreachablePub: UnreachablePub,
    | |                 ^^^^^^^^^^^^^^
212 | |                 ExplicitOutlivesRequirements: ExplicitOutlivesRequirements,
213 | |                 InvalidValue: InvalidValue,
    | |                 ^^^^^^^^^^^^
    | |                 ^^^^^^^^^^^^
214 | |                 DerefNullPtr: DerefNullPtr,
    | |                 ^^^^^^^^^^^^
215 | |                 // May Depend on constants elsewhere
216 | |                 UnusedBrokenConst: UnusedBrokenConst,
217 | |                 UnstableFeatures: UnstableFeatures,
    | |                 ^^^^^^^^^^^^^^^^
218 | |                 ArrayIntoIter: ArrayIntoIter::default(),
    | |                 ^^^^^^^^^^^^^
    | |                 ^^^^^^^^^^^^^
219 | |                 DropTraitConstraints: DropTraitConstraints,
    | |                 ^^^^^^^^^^^^^^^^^^^^
220 | |                 TemporaryCStringAsPtr: TemporaryCStringAsPtr,
    | |                 ^^^^^^^^^^^^^^^^^^^^^
221 | |                 NonPanicFmt: NonPanicFmt,
    | |                 ^^^^^^^^^^^
222 | |                 NoopMethodCall: NoopMethodCall,
    | |                 ^^^^^^^^^^^^^^
223 | |                 EnumIntrinsicsNonEnums: EnumIntrinsicsNonEnums,
224 | |                 InvalidAtomicOrdering: InvalidAtomicOrdering,
    | |                 ^^^^^^^^^^^^^^^^^^^^^
    | |                 ^^^^^^^^^^^^^^^^^^^^^
225 | |                 NamedAsmLabels: NamedAsmLabels,
226 | |             ]
227 | |         );
    | |         -
    | |_________|
---
228 | |     };
229 | | }
    | | -
    | |_|
    | |_in this expansion of `late_lint_mod_passes!` (#1)
    | |_in this expansion of `late_lint_mod_passes!` (#1)
    | |_in this expansion of `late_lint_mod_passes!` (#1)
    | |_in this expansion of `late_lint_mod_passes!` (#1)
    | |_in this expansion of `late_lint_mod_passes!` (#1)
    | |_in this expansion of `late_lint_mod_passes!` (#1)
    | |_in this expansion of `late_lint_mod_passes!` (#1)
    | |_in this expansion of `late_lint_mod_passes!` (#1)
    | |_in this expansion of `late_lint_mod_passes!` (#1)
    | |_in this expansion of `late_lint_mod_passes!` (#1)
    | |_in this expansion of `late_lint_mod_passes!` (#1)
    | |_in this expansion of `late_lint_mod_passes!` (#1)
    | |_in this expansion of `late_lint_mod_passes!` (#1)
    | |_in this expansion of `late_lint_mod_passes!` (#1)
    | |_in this expansion of `late_lint_mod_passes!` (#1)
    | |_in this expansion of `late_lint_mod_passes!` (#1)
    | |_in this expansion of `late_lint_mod_passes!` (#1)
    | |_in this expansion of `late_lint_mod_passes!` (#1)
    | |_in this expansion of `late_lint_mod_passes!` (#1)
    | |_in this expansion of `late_lint_mod_passes!` (#1)
    | |_in this expansion of `late_lint_mod_passes!` (#1)
    | |_in this expansion of `late_lint_mod_passes!` (#1)
    | |_in this expansion of `late_lint_mod_passes!` (#1)
    | |_in this expansion of `late_lint_mod_passes!` (#1)
    | |_in this expansion of `late_lint_mod_passes!` (#1)
    | |_in this expansion of `late_lint_mod_passes!` (#1)
    | |_in this expansion of `late_lint_mod_passes!` (#1)
    | |_in this expansion of `late_lint_mod_passes!` (#1)
    | |_in this expansion of `late_lint_mod_passes!` (#1)
    | |_in this expansion of `late_lint_mod_passes!` (#1)
    | |_in this expansion of `late_lint_mod_passes!` (#1)
    |   in this expansion of `late_lint_mod_passes!` (#1)
231 |   macro_rules! declare_combined_late_pass {
    |  _-
    | |_|
    | |_|
---
    | |_|
    | |_|
    | |_|
    | |
232 | |     ([$v:vis $name:ident], $passes:tt) => (
233 |           late_lint_methods!(declare_combined_late_lint_pass, [$v $name, $passes], ['tcx]);
    | |____________________________|
    | |____________________________|
    | |____________________________|
    | |____________________________|
---
234 | |     )
235 | | }
    | | -
    | |_|
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    | |_in this expansion of `declare_combined_late_pass!` (#2)
    |   in this expansion of `declare_combined_late_pass!` (#2)
...
240 | | late_lint_mod_passes!(declare_combined_late_pass, [BuiltinCombinedModuleLateLintPass]);
    | | |                                                  |
    | | |                                                  fields in this struct
    | | in this macro invocation (#1)
    | | in this macro invocation (#1)
---
    | |_|
    | |_|
    | |_|
    | |
105 | |     ([$v:vis $name:ident, [$($passes:ident: $constructor:expr,)*]], [$hir:tt], $methods:tt) => (
106 | |         #[allow(non_snake_case)]
107 | |         $v struct $name {
135 | |     )
136 | | }
    | | -
    | |_|
    | |_|
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    | |_in this expansion of `declare_combined_late_lint_pass!` (#3)
    |   in this expansion of `declare_combined_late_lint_pass!` (#3)
error: field `for_expr_span` is never read
  --> compiler/rustc_lint/src/array_into_iter.rs:42:5
   |
41 | pub struct ArrayIntoIter {
41 | pub struct ArrayIntoIter {
   |            ------------- field in this struct
42 |     for_expr_span: Span,
   |     ^^^^^^^^^^^^^
   |
   = note: `ArrayIntoIter` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis

error: function `pierce_parens` is never used
   |
   |
91 | fn pierce_parens(mut expr: &ast::Expr) -> &ast::Expr {

error: field `doc_hidden_stack` is never read
   --> compiler/rustc_lint/src/builtin.rs:503:5
    |
    |
501 | pub struct MissingDoc {
    |            ---------- field in this struct
502 |     /// Stack of whether `#[doc(hidden)]` is set at each level which has lint attributes.
503 |     doc_hidden_stack: Vec<bool>,


error: function `has_doc` is never used
    |
    |
508 | fn has_doc(attr: &ast::Attribute) -> bool {

error: field `impling_types` is never read
   --> compiler/rustc_lint/src/builtin.rs:804:5
    |
    |
803 | pub struct MissingDebugImplementations {
    |            --------------------------- field in this struct
804 |     impling_types: Option<LocalDefIdSet>,

error: field `depr_attrs` is never read
   --> compiler/rustc_lint/src/builtin.rs:941:5
    |
    |
938 | pub struct DeprecatedAttr {
    |            -------------- field in this struct
...
941 |     depr_attrs: Vec<&'static BuiltinAttribute>,
    |
    = note: `DeprecatedAttr` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis


error: function `warn_if_doc` is never used
    |
    |
999 | fn warn_if_doc(cx: &EarlyContext<'_>, node_span: Span, node_kind: &str, attrs: &[ast::Attribute]) {

error: field `node_id` is never read
    --> compiler/rustc_lint/src/builtin.rs:1710:5
     |
     |
1707 | pub struct EllipsisInclusiveRangePatterns {
...
...
1710 |     node_id: Option<ast::NodeId>,


error: fields `boundary` and `items_nameable` are never read
     |
     |
1842 | pub struct UnnameableTestItems {
     |            ------------------- fields in this struct
1843 |     boundary: Option<hir::OwnerId>, // Id of the item under which things are not nameable
1844 |     items_nameable: bool,
     |     ^^^^^^^^^^^^^^


error: struct `UnderMacro` is never constructed
     |
     |
1931 | struct UnderMacro(bool);

error: constant `HAS_MIN_FEATURES` is never used
    --> compiler/rustc_lint/src/builtin.rs:2323:7
     |
     |
2323 | const HAS_MIN_FEATURES: &[Symbol] = &[sym::specialization];


error: field `seen_decls` is never read
     |
2700 | pub struct ClashingExternDeclarations {
     |            -------------------------- field in this struct
...
...
2706 |     seen_decls: FxHashMap<Symbol, HirId>,

error: enum `SymbolName` is never used
    --> compiler/rustc_lint/src/builtin.rs:2713:6
     |
     |
2713 | enum SymbolName {
     |      ^^^^^^^^^^

error: associated function `check_heap_type` is never used
    |
    |
155 |     fn check_heap_type(&self, cx: &LateContext<'_>, span: Span, ty: Ty<'_>) {


error: associated function `report_unsafe` is never used
    |
308 |     fn report_unsafe(
    |        ^^^^^^^^^^^^^

---
    |
333 |     fn report_overridden_symbol_section(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: associated function `doc_hidden` is never used
    |
    |
537 |     fn doc_hidden(&self) -> bool {

error: associated function `check_missing_docs_attrs` is never used
   --> compiler/rustc_lint/src/builtin.rs:541:8
    |
---

error: associated function `suggest_changing_assoc_types` is never used
    --> compiler/rustc_lint/src/builtin.rs:1455:8
     |
1455 |     fn suggest_changing_assoc_types(ty: &hir::Ty<'_>, err: &mut Diagnostic) {


error: associated function `check_tokens` is never used
     |
     |
1934 |     fn check_tokens(&mut self, cx: &EarlyContext<'_>, tokens: TokenStream) {

error: associated function `check_ident_token` is never used
    --> compiler/rustc_lint/src/builtin.rs:1948:8
     |
     |
1948 |     fn check_ident_token(
     |        ^^^^^^^^^^^^^^^^^

error: associated function `lifetimes_outliving_lifetime` is never used
     |
     |
2016 |     fn lifetimes_outliving_lifetime<'tcx>(


error: associated function `lifetimes_outliving_type` is never used
     |
     |
2032 |     fn lifetimes_outliving_type<'tcx>(

error: associated function `collect_outlives_bound_spans` is never used
    --> compiler/rustc_lint/src/builtin.rs:2047:8
     |
---

error: associated function `insert` is never used
    --> compiler/rustc_lint/src/builtin.rs:2734:8
     |
2734 |     fn insert(&mut self, tcx: TyCtxt<'_>, fi: &hir::ForeignItem<'_>) -> Option<HirId> {


error: associated function `name_of_extern_decl` is never used
     |
     |
2751 |     fn name_of_extern_decl(tcx: TyCtxt<'_>, fi: &hir::ForeignItem<'_>) -> SymbolName {

error: associated function `structurally_same_type` is never used
    --> compiler/rustc_lint/src/builtin.rs:2773:8
     |
     |
2773 |     fn structurally_same_type<'tcx>(
     |        ^^^^^^^^^^^^^^^^^^^^^^

error: field `lints` is never read
   --> compiler/rustc_lint/src/early.rs:290:5
    |
289 | struct EarlyLintPassObjects<'a> {
    |        -------------------- field in this struct
290 |     lints: &'a mut [EarlyLintPassObject],

error: function `is_non_enum` is never used
  --> compiler/rustc_lint/src/enum_intrinsics_non_enums.rs:41:4
   |
---

error: function `enforce_mem_variant_count` is never used
  --> compiler/rustc_lint/src/enum_intrinsics_non_enums.rs:63:4
   |
63 | fn enforce_mem_variant_count(cx: &LateContext<'_>, func_expr: &hir::Expr<'_>, span: Span) {

error: associated function `lint_text_direction_codepoint` is never used
  --> compiler/rustc_lint/src/hidden_unicode_codepoints.rs:42:8
   |
   |
42 |     fn lint_text_direction_codepoint(
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: function `lint_ty_kind_usage` is never used
    |
    |
231 | fn lint_ty_kind_usage(cx: &LateContext<'_>, res: &Res) -> bool {


error: function `is_ty_or_ty_ctxt` is never used
    |
    |
239 | fn is_ty_or_ty_ctxt(cx: &LateContext<'_>, path: &Path<'_>) -> Option<String> {

error: function `gen_args` is never used
   --> compiler/rustc_lint/src/internal.rs:267:4
    |
    |
267 | fn gen_args(segment: &PathSegment<'_>) -> String {

error: function `is_doc_keyword` is never used
   --> compiler/rustc_lint/src/internal.rs:333:4
    |
    |
333 | fn is_doc_keyword(s: Symbol) -> bool {

error: constant `SYNC_GUARD_SYMBOLS` is never used
  --> compiler/rustc_lint/src/let_underscore.rs:96:7
   |
   |
96 | const SYNC_GUARD_SYMBOLS: [Symbol; 3] = [

error: function `build_and_emit_lint` is never used
   --> compiler/rustc_lint/src/let_underscore.rs:153:4
    |
    |
153 | fn build_and_emit_lint(
    |    ^^^^^^^^^^^^^^^^^^^

error: function `in_macro` is never used
  --> compiler/rustc_lint/src/methods.rs:37:4
   |
37 | fn in_macro(span: Span) -> bool {

error: function `first_method_call` is never used
  --> compiler/rustc_lint/src/methods.rs:45:4
   |
---

error: function `check_panic` is never used
  --> compiler/rustc_lint/src/non_fmt_panic.rs:88:4
   |
88 | fn check_panic<'tcx>(cx: &LateContext<'tcx>, f: &'tcx hir::Expr<'tcx>, arg: &'tcx hir::Expr<'tcx>) {

error: function `check_panic_str` is never used
   --> compiler/rustc_lint/src/non_fmt_panic.rs:221:4
    |
    |
221 | fn check_panic_str<'tcx>(
    |    ^^^^^^^^^^^^^^^

error: function `find_delimiters` is never used
   --> compiler/rustc_lint/src/non_fmt_panic.rs:309:4
    |
309 | fn find_delimiters<'tcx>(cx: &LateContext<'tcx>, span: Span) -> Option<(Span, Span, char)> {

error: function `panic_call` is never used
   --> compiler/rustc_lint/src/non_fmt_panic.rs:320:4
    |
    |
320 | fn panic_call<'tcx>(cx: &LateContext<'tcx>, f: &'tcx hir::Expr<'tcx>) -> (Span, Symbol, Symbol) {

error: function `is_arg_inside_call` is never used
   --> compiler/rustc_lint/src/non_fmt_panic.rs:351:4
    |
    |
351 | fn is_arg_inside_call(arg: Span, call: Span) -> bool {


error: variants `TraitAutoImpl`, `TraitImpl` and `PlainImpl` are never constructed
   |
   |
15 | pub enum MethodLateContext {
   |          ----------------- variants in this enum
16 |     TraitAutoImpl,
17 |     TraitImpl,
   |     ^^^^^^^^^
18 |     PlainImpl,
   |     ^^^^^^^^^
   |     ^^^^^^^^^

error: function `method_context` is never used
   |
   |
21 | pub fn method_context(cx: &LateContext<'_>, id: hir::HirId) -> MethodLateContext {


error: function `char_has_case` is never used
   |
   |
62 | fn char_has_case(c: char) -> bool {

error: function `is_camel_case` is never used
  --> compiler/rustc_lint/src/nonstandard_style.rs:74:4
   |
   |
74 | fn is_camel_case(name: &str) -> bool {


error: function `to_camel_case` is never used
   |
   |
90 | fn to_camel_case(s: &str) -> String {

error: associated function `check_case` is never used
   --> compiler/rustc_lint/src/nonstandard_style.rs:135:8
    |
    |
135 |     fn check_case(&self, cx: &EarlyContext<'_>, sort: &str, ident: &Ident) {

error: associated function `to_snake_case` is never used
   --> compiler/rustc_lint/src/nonstandard_style.rs:227:8
    |
    |
227 |     fn to_snake_case(mut str: &str) -> String {

error: associated function `check_snake_case` is never used
   --> compiler/rustc_lint/src/nonstandard_style.rs:258:8
    |
    |
258 |     fn check_snake_case(&self, cx: &LateContext<'_>, sort: &str, ident: &Ident) {

error: associated function `check_upper_case` is never used
   --> compiler/rustc_lint/src/nonstandard_style.rs:478:8
    |
    |
478 |     fn check_upper_case(cx: &LateContext<'_>, sort: &str, ident: &Ident) {


error: function `path_for_pass_by_value` is never used
   |
   |
51 | fn path_for_pass_by_value(cx: &LateContext<'_>, ty: &hir::Ty<'_>) -> Option<String> {

error: function `gen_args` is never used
  --> compiler/rustc_lint/src/pass_by_value.rs:73:4
   |
   |
73 | fn gen_args(cx: &LateContext<'_>, segment: &PathSegment<'_>) -> String {


error: function `maybe_lint_redundant_semis` is never used
   |
   |
43 | fn maybe_lint_redundant_semis(cx: &EarlyContext<'_>, seq: &mut Option<(Span, bool)>) {

error: field `negated_expr_id` is never read
   --> compiler/rustc_lint/src/types.rs:108:5
    |
    |
106 | pub struct TypeLimits {
    |            ---------- field in this struct
107 |     /// Id of the last visited negated expression
108 |     negated_expr_id: Option<hir::HirId>,
    |
    = note: `TypeLimits` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis

error: function `lint_overflowing_range_endpoint` is never used
---

error: function `int_ty_range` is never used
   --> compiler/rustc_lint/src/types.rs:178:4
    |
178 | fn int_ty_range(int_ty: ty::IntTy) -> (i128, i128) {

error: function `uint_ty_range` is never used
   --> compiler/rustc_lint/src/types.rs:189:4
    |
    |
189 | fn uint_ty_range(uint_ty: ty::UintTy) -> (u128, u128) {


error: function `get_bin_hex_repr` is never used
    |
    |
201 | fn get_bin_hex_repr(cx: &LateContext<'_>, lit: &hir::Lit) -> Option<String> {


error: function `report_bin_hex_error` is never used
    |
215 | fn report_bin_hex_error(
    |    ^^^^^^^^^^^^^^^^^^^^


error: function `get_type_suggestion` is never used
   --> compiler/rustc_lint/src/types.rs:279:4
    |
279 | fn get_type_suggestion(t: Ty<'_>, val: u128, negative: bool) -> Option<&'static str> {

error: function `lint_int_literal` is never used
   --> compiler/rustc_lint/src/types.rs:319:4
    |
    |
319 | fn lint_int_literal<'tcx>(
    |    ^^^^^^^^^^^^^^^^

error: function `lint_uint_literal` is never used
    |
376 | fn lint_uint_literal<'tcx>(
    |    ^^^^^^^^^^^^^^^^^


error: function `lint_literal` is never used
   --> compiler/rustc_lint/src/types.rs:445:4
    |
445 | fn lint_literal<'tcx>(
    |    ^^^^^^^^^^^^

error: variants `Declaration` and `Definition` are never constructed
    |
    |
650 | pub(crate) enum CItemKind {
    |                 --------- variants in this enum
    |     ^^^^^^^^^^^
652 |     Definition,
    |     ^^^^^^^^^^
    |
    |
    = note: `CItemKind` has a derived impl for the trait `Clone`, but this is intentionally ignored during dead code analysis

error: struct `ImproperCTypesVisitor` is never constructed
    |
    |
655 | struct ImproperCTypesVisitor<'a, 'tcx> {


error: enum `FfiResult` is never used
    |
    |
660 | enum FfiResult<'tcx> {


error: function `nonnull_optimization_guaranteed` is never used
    |
    |
666 | pub(crate) fn nonnull_optimization_guaranteed<'tcx>(

error: function `transparent_newtype_field` is never used
   --> compiler/rustc_lint/src/types.rs:675:8
    |
    |
675 | pub fn transparent_newtype_field<'a, 'tcx>(


error: function `ty_is_known_nonnull` is never used
    |
    |
688 | fn ty_is_known_nonnull<'tcx>(cx: &LateContext<'tcx>, ty: Ty<'tcx>, mode: CItemKind) -> bool {

error: function `get_nullable_type` is never used
   --> compiler/rustc_lint/src/types.rs:717:4
    |
    |
717 | fn get_nullable_type<'tcx>(cx: &LateContext<'tcx>, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {

error: function `repr_nullable_ptr` is never used
   --> compiler/rustc_lint/src/types.rs:767:15
    |
    |
767 | pub(crate) fn repr_nullable_ptr<'tcx>(

error: associated function `check_for_array_ty` is never used
   --> compiler/rustc_lint/src/types.rs:817:8
    |
    |
817 |     fn check_for_array_ty(&mut self, sp: Span, ty: Ty<'tcx>) -> bool {


error: associated function `check_field_type_for_ffi` is never used
    |
832 |     fn check_field_type_for_ffi(
    |        ^^^^^^^^^^^^^^^^^^^^^^^^


error: associated function `check_variant_for_ffi` is never used
    |
848 |     fn check_variant_for_ffi(
    |        ^^^^^^^^^^^^^^^^^^^^^


error: associated function `check_type_for_ffi` is never used
    |
    |
895 |     fn check_type_for_ffi(&self, cache: &mut FxHashSet<Ty<'tcx>>, ty: Ty<'tcx>) -> FfiResult<'tcx> {


error: associated function `emit_ffi_unsafe_type_lint` is never used
     |
     |
1141 |     fn emit_ffi_unsafe_type_lint(

error: associated function `check_for_opaque_ty` is never used
    --> compiler/rustc_lint/src/types.rs:1175:8
     |
     |
1175 |     fn check_for_opaque_ty(&mut self, sp: Span, ty: Ty<'tcx>) -> bool {


error: associated function `check_type_for_ffi_and_report_errors` is never used
     |
     |
1212 |     fn check_type_for_ffi_and_report_errors(

error: associated function `check_foreign_fn` is never used
    --> compiler/rustc_lint/src/types.rs:1263:8
     |
     |
1263 |     fn check_foreign_fn(&mut self, id: hir::HirId, decl: &hir::FnDecl<'_>) {

error: associated function `check_foreign_static` is never used
    --> compiler/rustc_lint/src/types.rs:1278:8
     |
     |
1278 |     fn check_foreign_static(&mut self, id: hir::HirId, span: Span) {

error: associated function `is_internal_abi` is never used
    --> compiler/rustc_lint/src/types.rs:1284:8
     |
     |
1284 |     fn is_internal_abi(&self, abi: SpecAbi) -> bool {


error: associated function `inherent_atomic_method_call` is never used
     |
     |
1440 |     fn inherent_atomic_method_call<'hir>(

error: associated function `match_ordering` is never used
    --> compiler/rustc_lint/src/types.rs:1477:8
     |
     |
1477 |     fn match_ordering(cx: &LateContext<'_>, ord_arg: &Expr<'_>) -> Option<Symbol> {


error: associated function `check_atomic_load_store` is never used
     |
     |
1494 |     fn check_atomic_load_store(cx: &LateContext<'_>, expr: &Expr<'_>) {


error: associated function `check_memory_fence` is never used
     |
     |
1519 |     fn check_memory_fence(cx: &LateContext<'_>, expr: &Expr<'_>) {

error: associated function `check_atomic_compare_exchange` is never used
    --> compiler/rustc_lint/src/types.rs:1534:8
     |
     |
1534 |     fn check_atomic_compare_exchange(cx: &LateContext<'_>, expr: &Expr<'_>) {

error: multiple variants are never constructed
   --> compiler/rustc_lint/src/unused.rs:386:5
    |
    |
385 | enum UnusedDelimsCtx {
    |      --------------- variants in this enum
386 |     FunctionArg,
387 |     MethodArg,
    |     ^^^^^^^^^
388 |     AssignedValue,
    |     ^^^^^^^^^^^^^
    |     ^^^^^^^^^^^^^
389 |     AssignedValueLetElse,
390 |     IfCond,
391 |     WhileCond,
    |     ^^^^^^^^^
392 |     ForIterExpr,
    |     ^^^^^^^^^^^
    |     ^^^^^^^^^^^
393 |     MatchScrutineeExpr,
    |     ^^^^^^^^^^^^^^^^^^
394 |     ReturnValue,
    |     ^^^^^^^^^^^
395 |     BlockRetValue,
    |     ^^^^^^^^^^^^^
396 |     LetScrutineeExpr,
397 |     ArrayLenExpr,
398 |     AnonConst,
399 |     MatchArmExpr,
    |     ^^^^^^^^^^^^
    |
    |
    = note: `UnusedDelimsCtx` has derived impls for the traits `Debug` and `Clone`, but these are intentionally ignored during dead code analysis

error: associated function `check_unused_parens_pat` is never used
    |
747 |     fn check_unused_parens_pat(
    |        ^^^^^^^^^^^^^^^^^^^^^^^


error: associated function `check_use_tree` is never used
    --> compiler/rustc_lint/src/unused.rs:1106:8
     |
1106 |     fn check_use_tree(&self, cx: &EarlyContext<'_>, use_tree: &ast::UseTree, item: &ast::Item) {

error: could not compile `rustc_lint` due to 170 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_lint` due to 167 previous errors
