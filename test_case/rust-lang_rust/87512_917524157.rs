plain
   Compiling rustc_lint v0.0.0 (/checkout/compiler/rustc_lint)
error: redundant field names in struct initialization
   --> compiler/rustc_lint/src/lib.rs:114:40
    |
112 |  / macro_rules! pre_expansion_lint_passes {
113 |  |     ($macro:path, $args:tt) => {
114 |  |         $macro!($args, [KeywordIdents: KeywordIdents,]);
    |  |_________-------------------------------^----------------
    | ||         in this macro invocation (#2)
115 | ||     };
116 | || }
    | || -
    | || -
    | ||_|
    | |  in this expansion of `pre_expansion_lint_passes!` (#1)
...   |
...   |
142 | /  macro_rules! declare_combined_early_pass {
143 |        ([$name:ident], $passes:tt) => (
144 |            early_lint_methods!(declare_combined_early_lint_pass, [pub $name, $passes]);
145 |        )
146 | |  }
146 | |  }
    | |__- in this expansion of `declare_combined_early_pass!` (#2)
147 | 
148 | |  pre_expansion_lint_passes!(declare_combined_early_pass, [BuiltinCombinedPreExpansionLintPass]);
    | |  ----------------------------------------------------------------------------------------------- in this macro invocation (#1)
258 | |              $(
258 | |              $(
259 | |                  register_pass!($method, $passes, $constructor);
    | |______________________________^ help: replace it with: `KeywordIdents`
   ::: compiler/rustc_lint/src/passes.rs:157:1
    |
    |
157 | /  macro_rules! early_lint_methods {
158 | |      ($macro:path, $args:tt) => (
159 |            $macro!($args, [
    |  __________-
160 |                fn check_param(a: &ast::Param);
161 |                fn check_ident(a: Ident);
162 |                fn check_crate(a: &ast::Crate);
...
211 |                fn exit_lint_attrs(a: &[ast::Attribute]);
    | |____________- in this macro invocation (#4)
213 | |      )
214 | |  }
214 | |  }
    | |__- in this expansion of `early_lint_methods!` (#3)
...
249 | /  macro_rules! declare_combined_early_lint_pass {
250 | |      ([$v:vis $name:ident, [$($passes:ident: $constructor:expr,)*]], $methods:tt) => (
251 | |          #[allow(non_snake_case)]
252 | |          $v struct $name {
280 | |      )
281 | |  }
281 | |  }
    | |__- in this expansion of `declare_combined_early_lint_pass!` (#4)
    |
    = note: `-D redundant-field-initializers` implied by `-D warnings`
error: redundant field names in struct initialization
   --> compiler/rustc_lint/src/lib.rs:123:31
    |
    |
118 |   / macro_rules! early_lint_passes {
119 |   |     ($macro:path, $args:tt) => {
120 |   |         $macro!(
    |   |_________-
121 |  ||             $args,
122 |  ||             [
123 |  ||                 UnusedParens: UnusedParens,
    |  ||_______________________________^
124 | |||                 UnusedBraces: UnusedBraces,
125 | |||                 UnusedImportBraces: UnusedImportBraces,
126 | |||                 UnsafeCode: UnsafeCode,
...   |||
137 | |||             ]
138 | |||         );
    | |||__________- in this macro invocation (#2)
139 | | |     };
140 | | | }
    | | |_- in this expansion of `early_lint_passes!` (#1)
141 | |
142 | /   macro_rules! declare_combined_early_pass {
143 |         ([$name:ident], $passes:tt) => (
144 |             early_lint_methods!(declare_combined_early_lint_pass, [pub $name, $passes]);
145 |         )
146 | |   }
146 | |   }
    | |___- in this expansion of `declare_combined_early_pass!` (#2)
...
149 | |   early_lint_passes!(declare_combined_early_pass, [BuiltinCombinedEarlyLintPass]);
...   |
258 | |               $(
258 | |               $(
259 | |                   register_pass!($method, $passes, $constructor);
    | |_______________________________^ help: replace it with: `UnusedParens`
   ::: compiler/rustc_lint/src/passes.rs:157:1
    |
    |
157 | /   macro_rules! early_lint_methods {
158 | |       ($macro:path, $args:tt) => (
159 |             $macro!($args, [
    |  ___________-
160 |                 fn check_param(a: &ast::Param);
161 |                 fn check_ident(a: Ident);
162 |                 fn check_crate(a: &ast::Crate);
...
211 |                 fn exit_lint_attrs(a: &[ast::Attribute]);
    | |_____________- in this macro invocation (#4)
213 | |       )
214 | |   }
214 | |   }
    | |___- in this expansion of `early_lint_methods!` (#3)
...
249 | /   macro_rules! declare_combined_early_lint_pass {
250 | |       ([$v:vis $name:ident, [$($passes:ident: $constructor:expr,)*]], $methods:tt) => (
251 | |           #[allow(non_snake_case)]
252 | |           $v struct $name {
280 | |       )
281 | |   }
281 | |   }
    | |___- in this expansion of `declare_combined_early_lint_pass!` (#4)
error: redundant field names in struct initialization
   --> compiler/rustc_lint/src/lib.rs:124:31
    |
    |
118 |   / macro_rules! early_lint_passes {
119 |   |     ($macro:path, $args:tt) => {
120 |   |         $macro!(
    |   |_________-
121 |  ||             $args,
122 |  ||             [
123 |  ||                 UnusedParens: UnusedParens,
124 |  ||                 UnusedBraces: UnusedBraces,
    |  ||_______________________________^
125 | |||                 UnusedImportBraces: UnusedImportBraces,
126 | |||                 UnsafeCode: UnsafeCode,
127 | |||                 AnonymousParameters: AnonymousParameters,
...   |||
137 | |||             ]
138 | |||         );
    | |||__________- in this macro invocation (#2)
139 | | |     };
140 | | | }
    | | |_- in this expansion of `early_lint_passes!` (#1)
141 | |
142 | /   macro_rules! declare_combined_early_pass {
143 |         ([$name:ident], $passes:tt) => (
144 |             early_lint_methods!(declare_combined_early_lint_pass, [pub $name, $passes]);
145 |         )
146 | |   }
146 | |   }
    | |___- in this expansion of `declare_combined_early_pass!` (#2)
...
149 | |   early_lint_passes!(declare_combined_early_pass, [BuiltinCombinedEarlyLintPass]);
...   |
258 | |               $(
258 | |               $(
259 | |                   register_pass!($method, $passes, $constructor);
    | |_______________________________^ help: replace it with: `UnusedBraces`
   ::: compiler/rustc_lint/src/passes.rs:157:1
    |
    |
157 | /   macro_rules! early_lint_methods {
158 | |       ($macro:path, $args:tt) => (
159 |             $macro!($args, [
    |  ___________-
160 |                 fn check_param(a: &ast::Param);
161 |                 fn check_ident(a: Ident);
162 |                 fn check_crate(a: &ast::Crate);
...
211 |                 fn exit_lint_attrs(a: &[ast::Attribute]);
    | |_____________- in this macro invocation (#4)
213 | |       )
214 | |   }
214 | |   }
    | |___- in this expansion of `early_lint_methods!` (#3)
...
249 | /   macro_rules! declare_combined_early_lint_pass {
250 | |       ([$v:vis $name:ident, [$($passes:ident: $constructor:expr,)*]], $methods:tt) => (
251 | |           #[allow(non_snake_case)]
252 | |           $v struct $name {
280 | |       )
281 | |   }
281 | |   }
    | |___- in this expansion of `declare_combined_early_lint_pass!` (#4)
error: redundant field names in struct initialization
   --> compiler/rustc_lint/src/lib.rs:125:37
    |
    |
118 |   / macro_rules! early_lint_passes {
119 |   |     ($macro:path, $args:tt) => {
120 |   |         $macro!(
    |   |_________-
121 |  ||             $args,
122 |  ||             [
123 |  ||                 UnusedParens: UnusedParens,
124 |  ||                 UnusedBraces: UnusedBraces,
125 |  ||                 UnusedImportBraces: UnusedImportBraces,
    |  ||_____________________________________^
126 | |||                 UnsafeCode: UnsafeCode,
127 | |||                 AnonymousParameters: AnonymousParameters,
128 | |||                 EllipsisInclusiveRangePatterns: EllipsisInclusiveRangePatterns::default(),
...   |||
137 | |||             ]
138 | |||         );
    | |||__________- in this macro invocation (#2)
139 | | |     };
140 | | | }
    | | |_- in this expansion of `early_lint_passes!` (#1)
141 | |
142 | /   macro_rules! declare_combined_early_pass {
143 |         ([$name:ident], $passes:tt) => (
144 |             early_lint_methods!(declare_combined_early_lint_pass, [pub $name, $passes]);
145 |         )
146 | |   }
146 | |   }
    | |___- in this expansion of `declare_combined_early_pass!` (#2)
...
149 | |   early_lint_passes!(declare_combined_early_pass, [BuiltinCombinedEarlyLintPass]);
...   |
258 | |               $(
258 | |               $(
259 | |                   register_pass!($method, $passes, $constructor);
    | |_______________________________^ help: replace it with: `UnusedImportBraces`
   ::: compiler/rustc_lint/src/passes.rs:157:1
    |
    |
157 | /   macro_rules! early_lint_methods {
158 | |       ($macro:path, $args:tt) => (
159 |             $macro!($args, [
    |  ___________-
160 |                 fn check_param(a: &ast::Param);
161 |                 fn check_ident(a: Ident);
162 |                 fn check_crate(a: &ast::Crate);
...
211 |                 fn exit_lint_attrs(a: &[ast::Attribute]);
    | |_____________- in this macro invocation (#4)
213 | |       )
214 | |   }
214 | |   }
    | |___- in this expansion of `early_lint_methods!` (#3)
...
249 | /   macro_rules! declare_combined_early_lint_pass {
250 | |       ([$v:vis $name:ident, [$($passes:ident: $constructor:expr,)*]], $methods:tt) => (
251 | |           #[allow(non_snake_case)]
252 | |           $v struct $name {
280 | |       )
281 | |   }
281 | |   }
    | |___- in this expansion of `declare_combined_early_lint_pass!` (#4)
error: redundant field names in struct initialization
   --> compiler/rustc_lint/src/lib.rs:126:29
    |
    |
118 |   / macro_rules! early_lint_passes {
119 |   |     ($macro:path, $args:tt) => {
120 |   |         $macro!(
    |   |_________-
121 |  ||             $args,
122 |  ||             [
123 |  ||                 UnusedParens: UnusedParens,
...    ||
126 |  ||                 UnsafeCode: UnsafeCode,
    |  ||_____________________________^
127 | |||                 AnonymousParameters: AnonymousParameters,
128 | |||                 EllipsisInclusiveRangePatterns: EllipsisInclusiveRangePatterns::default(),
129 | |||                 NonCamelCaseTypes: NonCamelCaseTypes,
...   |||
137 | |||             ]
138 | |||         );
    | |||__________- in this macro invocation (#2)
139 | | |     };
140 | | | }
    | | |_- in this expansion of `early_lint_passes!` (#1)
141 | |
142 | /   macro_rules! declare_combined_early_pass {
143 |         ([$name:ident], $passes:tt) => (
144 |             early_lint_methods!(declare_combined_early_lint_pass, [pub $name, $passes]);
145 |         )
146 | |   }
146 | |   }
    | |___- in this expansion of `declare_combined_early_pass!` (#2)
...
149 | |   early_lint_passes!(declare_combined_early_pass, [BuiltinCombinedEarlyLintPass]);
...   |
258 | |               $(
258 | |               $(
259 | |                   register_pass!($method, $passes, $constructor);
    | |_______________________________^ help: replace it with: `UnsafeCode`
   ::: compiler/rustc_lint/src/passes.rs:157:1
    |
    |
157 | /   macro_rules! early_lint_methods {
158 | |       ($macro:path, $args:tt) => (
159 |             $macro!($args, [
    |  ___________-
160 |                 fn check_param(a: &ast::Param);
161 |                 fn check_ident(a: Ident);
162 |                 fn check_crate(a: &ast::Crate);
...
211 |                 fn exit_lint_attrs(a: &[ast::Attribute]);
    | |_____________- in this macro invocation (#4)
213 | |       )
214 | |   }
214 | |   }
    | |___- in this expansion of `early_lint_methods!` (#3)
...
249 | /   macro_rules! declare_combined_early_lint_pass {
250 | |       ([$v:vis $name:ident, [$($passes:ident: $constructor:expr,)*]], $methods:tt) => (
251 | |           #[allow(non_snake_case)]
252 | |           $v struct $name {
280 | |       )
281 | |   }
281 | |   }
    | |___- in this expansion of `declare_combined_early_lint_pass!` (#4)
error: redundant field names in struct initialization
   --> compiler/rustc_lint/src/lib.rs:127:38
    |
    |
118 |   / macro_rules! early_lint_passes {
119 |   |     ($macro:path, $args:tt) => {
120 |   |         $macro!(
    |   |_________-
121 |  ||             $args,
122 |  ||             [
123 |  ||                 UnusedParens: UnusedParens,
...    ||
127 |  ||                 AnonymousParameters: AnonymousParameters,
    |  ||______________________________________^
128 | |||                 EllipsisInclusiveRangePatterns: EllipsisInclusiveRangePatterns::default(),
129 | |||                 NonCamelCaseTypes: NonCamelCaseTypes,
130 | |||                 DeprecatedAttr: DeprecatedAttr::new(),
...   |||
137 | |||             ]
138 | |||         );
    | |||__________- in this macro invocation (#2)
139 | | |     };
140 | | | }
    | | |_- in this expansion of `early_lint_passes!` (#1)
141 | |
142 | /   macro_rules! declare_combined_early_pass {
143 |         ([$name:ident], $passes:tt) => (
144 |             early_lint_methods!(declare_combined_early_lint_pass, [pub $name, $passes]);
145 |         )
146 | |   }
146 | |   }
    | |___- in this expansion of `declare_combined_early_pass!` (#2)
...
149 | |   early_lint_passes!(declare_combined_early_pass, [BuiltinCombinedEarlyLintPass]);
...   |
258 | |               $(
258 | |               $(
259 | |                   register_pass!($method, $passes, $constructor);
    | |_______________________________^ help: replace it with: `AnonymousParameters`
   ::: compiler/rustc_lint/src/passes.rs:157:1
    |
    |
157 | /   macro_rules! early_lint_methods {
158 | |       ($macro:path, $args:tt) => (
159 |             $macro!($args, [
    |  ___________-
160 |                 fn check_param(a: &ast::Param);
161 |                 fn check_ident(a: Ident);
162 |                 fn check_crate(a: &ast::Crate);
...
211 |                 fn exit_lint_attrs(a: &[ast::Attribute]);
    | |_____________- in this macro invocation (#4)
213 | |       )
214 | |   }
214 | |   }
    | |___- in this expansion of `early_lint_methods!` (#3)
...
249 | /   macro_rules! declare_combined_early_lint_pass {
250 | |       ([$v:vis $name:ident, [$($passes:ident: $constructor:expr,)*]], $methods:tt) => (
251 | |           #[allow(non_snake_case)]
252 | |           $v struct $name {
280 | |       )
281 | |   }
281 | |   }
    | |___- in this expansion of `declare_combined_early_lint_pass!` (#4)
error: redundant field names in struct initialization
   --> compiler/rustc_lint/src/lib.rs:129:36
    |
    |
118 |   / macro_rules! early_lint_passes {
119 |   |     ($macro:path, $args:tt) => {
120 |   |         $macro!(
    |   |_________-
121 |  ||             $args,
122 |  ||             [
123 |  ||                 UnusedParens: UnusedParens,
...    ||
129 |  ||                 NonCamelCaseTypes: NonCamelCaseTypes,
    |  ||____________________________________^
130 | |||                 DeprecatedAttr: DeprecatedAttr::new(),
131 | |||                 WhileTrue: WhileTrue,
132 | |||                 NonAsciiIdents: NonAsciiIdents,
...   |||
137 | |||             ]
138 | |||         );
    | |||__________- in this macro invocation (#2)
139 | | |     };
140 | | | }
    | | |_- in this expansion of `early_lint_passes!` (#1)
141 | |
142 | /   macro_rules! declare_combined_early_pass {
143 |         ([$name:ident], $passes:tt) => (
144 |             early_lint_methods!(declare_combined_early_lint_pass, [pub $name, $passes]);
145 |         )
146 | |   }
146 | |   }
    | |___- in this expansion of `declare_combined_early_pass!` (#2)
...
149 | |   early_lint_passes!(declare_combined_early_pass, [BuiltinCombinedEarlyLintPass]);
...   |
258 | |               $(
258 | |               $(
259 | |                   register_pass!($method, $passes, $constructor);
    | |_______________________________^ help: replace it with: `NonCamelCaseTypes`
   ::: compiler/rustc_lint/src/passes.rs:157:1
    |
    |
157 | /   macro_rules! early_lint_methods {
158 | |       ($macro:path, $args:tt) => (
159 |             $macro!($args, [
    |  ___________-
160 |                 fn check_param(a: &ast::Param);
161 |                 fn check_ident(a: Ident);
162 |                 fn check_crate(a: &ast::Crate);
...
211 |                 fn exit_lint_attrs(a: &[ast::Attribute]);
    | |_____________- in this macro invocation (#4)
213 | |       )
214 | |   }
214 | |   }
    | |___- in this expansion of `early_lint_methods!` (#3)
...
249 | /   macro_rules! declare_combined_early_lint_pass {
250 | |       ([$v:vis $name:ident, [$($passes:ident: $constructor:expr,)*]], $methods:tt) => (
251 | |           #[allow(non_snake_case)]
252 | |           $v struct $name {
280 | |       )
281 | |   }
281 | |   }
    | |___- in this expansion of `declare_combined_early_lint_pass!` (#4)
error: redundant field names in struct initialization
   --> compiler/rustc_lint/src/lib.rs:131:28
    |
    |
118 |   / macro_rules! early_lint_passes {
119 |   |     ($macro:path, $args:tt) => {
120 |   |         $macro!(
    |   |_________-
121 |  ||             $args,
122 |  ||             [
123 |  ||                 UnusedParens: UnusedParens,
...    ||
131 |  ||                 WhileTrue: WhileTrue,
    |  ||____________________________^
132 | |||                 NonAsciiIdents: NonAsciiIdents,
133 | |||                 IncompleteFeatures: IncompleteFeatures,
134 | |||                 RedundantFieldInitializers: RedundantFieldInitializers,
...   |||
137 | |||             ]
138 | |||         );
    | |||__________- in this macro invocation (#2)
139 | | |     };
140 | | | }
    | | |_- in this expansion of `early_lint_passes!` (#1)
141 | |
142 | /   macro_rules! declare_combined_early_pass {
143 |         ([$name:ident], $passes:tt) => (
144 |             early_lint_methods!(declare_combined_early_lint_pass, [pub $name, $passes]);
145 |         )
146 | |   }
146 | |   }
    | |___- in this expansion of `declare_combined_early_pass!` (#2)
...
149 | |   early_lint_passes!(declare_combined_early_pass, [BuiltinCombinedEarlyLintPass]);
...   |
258 | |               $(
258 | |               $(
259 | |                   register_pass!($method, $passes, $constructor);
    | |_______________________________^ help: replace it with: `WhileTrue`
   ::: compiler/rustc_lint/src/passes.rs:157:1
    |
    |
157 | /   macro_rules! early_lint_methods {
158 | |       ($macro:path, $args:tt) => (
159 |             $macro!($args, [
    |  ___________-
160 |                 fn check_param(a: &ast::Param);
161 |                 fn check_ident(a: Ident);
162 |                 fn check_crate(a: &ast::Crate);
...
211 |                 fn exit_lint_attrs(a: &[ast::Attribute]);
    | |_____________- in this macro invocation (#4)
213 | |       )
214 | |   }
