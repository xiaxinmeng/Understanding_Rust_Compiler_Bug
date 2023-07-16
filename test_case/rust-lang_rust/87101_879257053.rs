diff
diff --git a/compiler/rustc_expand/src/expand.rs b/compiler/rustc_expand/src/expand.rs
index f8a12ef8a20..76832c03c09 100644
--- a/compiler/rustc_expand/src/expand.rs
+++ b/compiler/rustc_expand/src/expand.rs
@@ -22,7 +22,9 @@
 use rustc_data_structures::sync::Lrc;
 use rustc_errors::{Applicability, FatalError, PResult};
 use rustc_feature::Features;
-use rustc_parse::parser::{AttemptLocalParseRecovery, ForceCollect, Parser, RecoverComma};
+use rustc_parse::parser::{
+    AttemptLocalParseRecovery, ForceCollect, Parser, RecoverColon, RecoverComma,
+};
 use rustc_parse::validate_attr;
 use rustc_session::lint::builtin::UNUSED_DOC_COMMENTS;
 use rustc_session::lint::BuiltinLintDiagnostics;
@@ -930,9 +932,11 @@ pub fn parse_ast_fragment<'a>(
             }
         }
         AstFragmentKind::Ty => AstFragment::Ty(this.parse_ty()?),
-        AstFragmentKind::Pat => {
-            AstFragment::Pat(this.parse_pat_allow_top_alt(None, RecoverComma::No)?)
-        }
+        AstFragmentKind::Pat => AstFragment::Pat(this.parse_pat_allow_top_alt(
+            None,
+            RecoverComma::No,
+            RecoverColon::Yes,
+        )?),
         AstFragmentKind::Arms
         | AstFragmentKind::Fields
         | AstFragmentKind::FieldPats
diff --git a/compiler/rustc_parse/src/parser/expr.rs b/compiler/rustc_parse/src/parser/expr.rs
index 9dff40ff1ed..47fdd852d90 100644
--- a/compiler/rustc_parse/src/parser/expr.rs
+++ b/compiler/rustc_parse/src/parser/expr.rs
@@ -1,4 +1,4 @@
-use super::pat::{RecoverComma, PARAM_EXPECTED};
+use super::pat::{RecoverColon, RecoverComma, PARAM_EXPECTED};
 use super::ty::{AllowPlus, RecoverQPath, RecoverReturnSign};
 use super::{AttrWrapper, BlockMode, ForceCollect, Parser, PathStyle, Restrictions, TokenType};
 use super::{SemiColonMode, SeqSep, TokenExpectType, TrailingToken};
@@ -1813,7 +1813,7 @@ fn parse_cond_expr(&mut self) -> PResult<'a, P<Expr>> {
     /// The `let` token has already been eaten.
     fn parse_let_expr(&mut self, attrs: AttrVec) -> PResult<'a, P<Expr>> {
         let lo = self.prev_token.span;
-        let pat = self.parse_pat_allow_top_alt(None, RecoverComma::Yes)?;
+        let pat = self.parse_pat_allow_top_alt(None, RecoverComma::Yes, RecoverColon::Yes)?;
         self.expect(&token::Eq)?;
         let expr = self.with_res(self.restrictions | Restrictions::NO_STRUCT_LITERAL, |this| {
             this.parse_assoc_expr_with(1 + prec_let_scrutinee_needs_par(), None.into())
@@ -1876,7 +1876,7 @@ fn parse_for_expr(
             _ => None,
         };
 
-        let pat = self.parse_pat_allow_top_alt(None, RecoverComma::Yes)?;
+        let pat = self.parse_pat_allow_top_alt(None, RecoverComma::Yes, RecoverColon::Yes)?;
         if !self.eat_keyword(kw::In) {
             self.error_missing_in_for_loop();
         }
@@ -2083,7 +2083,7 @@ pub(super) fn parse_arm(&mut self) -> PResult<'a, Arm> {
         let attrs = self.parse_outer_attributes()?;
         self.collect_tokens_trailing_token(attrs, ForceCollect::No, |this, attrs| {
             let lo = this.token.span;
-            let pat = this.parse_pat_allow_top_alt(None, RecoverComma::Yes)?;
+            let pat = this.parse_pat_allow_top_alt(None, RecoverComma::Yes, RecoverColon::Yes)?;
             let guard = if this.eat_keyword(kw::If) {
                 let if_span = this.prev_token.span;
                 let cond = this.parse_expr()?;
diff --git a/compiler/rustc_parse/src/parser/mod.rs b/compiler/rustc_parse/src/parser/mod.rs
index cd9f84db5e5..51d4e007b59 100644
--- a/compiler/rustc_parse/src/parser/mod.rs
+++ b/compiler/rustc_parse/src/parser/mod.rs
@@ -14,7 +14,7 @@
 pub use attr_wrapper::AttrWrapper;
 pub use diagnostics::AttemptLocalParseRecovery;
 use diagnostics::Error;
-pub use pat::RecoverComma;
+pub use pat::{RecoverColon, RecoverComma};
 pub use path::PathStyle;
 
 use rustc_ast::ptr::P;
diff --git a/compiler/rustc_parse/src/parser/nonterminal.rs b/compiler/rustc_parse/src/parser/nonterminal.rs
index 30a6b61407f..313d9db58fc 100644
--- a/compiler/rustc_parse/src/parser/nonterminal.rs
+++ b/compiler/rustc_parse/src/parser/nonterminal.rs
@@ -5,7 +5,7 @@
 use rustc_errors::PResult;
 use rustc_span::symbol::{kw, Ident};
 
-use crate::parser::pat::RecoverComma;
+use crate::parser::pat::{RecoverColon, RecoverComma};
 use crate::parser::{FollowedByType, ForceCollect, Parser, PathStyle};
 
 impl<'a> Parser<'a> {
@@ -125,7 +125,7 @@ pub fn parse_nonterminal(&mut self, kind: NonterminalKind) -> PResult<'a, Nonter
                 token::NtPat(self.collect_tokens_no_attrs(|this| match kind {
                     NonterminalKind::PatParam { .. } => this.parse_pat_no_top_alt(None),
                     NonterminalKind::PatWithOr { .. } => {
-                        this.parse_pat_allow_top_alt(None, RecoverComma::No)
+                        this.parse_pat_allow_top_alt(None, RecoverComma::No, RecoverColon::No)
                     }
                     _ => unreachable!(),
                 })?)
diff --git a/compiler/rustc_parse/src/parser/pat.rs b/compiler/rustc_parse/src/parser/pat.rs
index 566677d032a..7da9648354c 100644
--- a/compiler/rustc_parse/src/parser/pat.rs
+++ b/compiler/rustc_parse/src/parser/pat.rs
@@ -24,6 +24,13 @@ pub enum RecoverComma {
     No,
 }
 
+/// Whether or not to recover a `:` when parsing patterns that were meant to be paths.
+#[derive(PartialEq, Copy, Clone)]
+pub enum RecoverColon {
+    Yes,
+    No,
+}
+
 /// The result of `eat_or_separator`. We want to distinguish which case we are in to avoid
 /// emitting duplicate diagnostics.
 #[derive(Debug, Clone, Copy)]
@@ -58,8 +65,9 @@ pub fn parse_pat_allow_top_alt(
         &mut self,
         expected: Expected,
         rc: RecoverComma,
+        ra: RecoverColon,
     ) -> PResult<'a, P<Pat>> {
-        self.parse_pat_allow_top_alt_inner(expected, rc).map(|(pat, _)| pat)
+        self.parse_pat_allow_top_alt_inner(expected, rc, ra).map(|(pat, _)| pat)
     }
 
     /// Returns the pattern and a bool indicating whether we recovered from a trailing vert (true =
@@ -68,6 +76,7 @@ fn parse_pat_allow_top_alt_inner(
         &mut self,
         expected: Expected,
         rc: RecoverComma,
+        ra: RecoverColon,
     ) -> PResult<'a, (P<Pat>, bool)> {
         // Keep track of whether we recovered from a trailing vert so that we can avoid duplicated
         // suggestions (which bothers rustfix).
@@ -89,6 +98,51 @@ fn parse_pat_allow_top_alt_inner(
             // If we parsed a leading `|` which should be gated,
             // then we should really gate the leading `|`.
             // This complicated procedure is done purely for diagnostics UX.
+            let mut first_pat = first_pat;
+
+            if let (RecoverColon::Yes, token::Colon) = (ra, &self.token.kind) {
+                if self.look_ahead(1, |token| token.is_ident() && !token.is_reserved_ident()) {
+                    // The pattern looks like it might be a path with a `::` -> `:` typo:
+                    // `match foo { bar:baz => {} }`
+                    let span = self.token.span;
+                    // We only emit "unexpected `:`" error here if we can successfully parse the
+                    // whole pattern correctly in that case.
+                    let snapshot = self.clone();
+
+                    // Create error for "unexpected `:`".
+                    match self.expected_one_of_not_found(&[], &[]) {
+                        Err(mut err) => {
+                            self.bump(); // Skip the `:`.
+                            match self.parse_pat_no_top_alt(expected) {
+                                Err(mut inner_err) => {
+                                    // Carry on as if we had not done anything, callers will emit a
+                                    // reasonable error.
+                                    inner_err.cancel();
+                                    err.cancel();
+                                    *self = snapshot;
+                                }
+                                Ok(pat) => {
+                                    // We've parsed the rest of the pattern.
+                                    err.span_suggestion(
+                                        span,
+                                        "maybe write a path separator here",
+                                        "::".to_string(),
+                                        Applicability::MachineApplicable,
+                                    );
+                                    err.emit();
+                                    first_pat =
+                                        self.mk_pat(first_pat.span.to(pat.span), PatKind::Wild);
+                                }
+                            }
+                        }
+                        _ => {
+                            // Carry on as if we had not done anything. This should be unreachable.
+                            *self = snapshot;
+                        }
+                    };
+                }
+            }
+
             if let Some(leading_vert_span) = leading_vert_span {
                 // If there was a leading vert, treat this as an or-pattern. This improves
                 // diagnostics.
@@ -140,7 +194,8 @@ pub(super) fn parse_pat_before_ty(
         // We use `parse_pat_allow_top_alt` regardless of whether we actually want top-level
         // or-patterns so that we can detect when a user tries to use it. This allows us to print a
         // better error message.
-        let (pat, trailing_vert) = self.parse_pat_allow_top_alt_inner(expected, rc)?;
+        let (pat, trailing_vert) =
+            self.parse_pat_allow_top_alt_inner(expected, rc, RecoverColon::No)?;
         let colon = self.eat(&token::Colon);
 
         if let PatKind::Or(pats) = &pat.kind {
@@ -350,7 +405,7 @@ fn parse_pat_with_range_pat(
         } else if self.check(&token::OpenDelim(token::Bracket)) {
             // Parse `[pat, pat,...]` as a slice pattern.
             let (pats, _) = self.parse_delim_comma_seq(token::Bracket, |p| {
-                p.parse_pat_allow_top_alt(None, RecoverComma::No)
+                p.parse_pat_allow_top_alt(None, RecoverComma::No, RecoverColon::No)
             })?;
             PatKind::Slice(pats)
         } else if self.check(&token::DotDot) && !self.is_pat_range_end_start(1) {
@@ -563,8 +618,9 @@ fn recover_lifetime_in_deref_pat(&mut self) {
 
     /// Parse a tuple or parenthesis pattern.
     fn parse_pat_tuple_or_parens(&mut self) -> PResult<'a, PatKind> {
-        let (fields, trailing_comma) =
-            self.parse_paren_comma_seq(|p| p.parse_pat_allow_top_alt(None, RecoverComma::No))?;
+        let (fields, trailing_comma) = self.parse_paren_comma_seq(|p| {
+            p.parse_pat_allow_top_alt(None, RecoverComma::No, RecoverColon::No)
+        })?;
 
         // Here, `(pat,)` is a tuple pattern.
         // For backward compatibility, `(..)` is a tuple pattern as well.
@@ -873,8 +929,9 @@ fn parse_pat_struct(&mut self, qself: Option<QSelf>, path: Path) -> PResult<'a,
 
     /// Parse tuple struct or tuple variant pattern (e.g. `Foo(...)` or `Foo::Bar(...)`).
     fn parse_pat_tuple_struct(&mut self, qself: Option<QSelf>, path: Path) -> PResult<'a, PatKind> {
-        let (fields, _) =
-            self.parse_paren_comma_seq(|p| p.parse_pat_allow_top_alt(None, RecoverComma::No))?;
+        let (fields, _) = self.parse_paren_comma_seq(|p| {
+            p.parse_pat_allow_top_alt(None, RecoverComma::No, RecoverColon::No)
+        })?;
         if qself.is_some() {
             self.sess.gated_spans.gate(sym::more_qualified_paths, path.span);
         }
@@ -1033,7 +1090,7 @@ fn parse_pat_field(&mut self, lo: Span, attrs: Vec<Attribute>) -> PResult<'a, Pa
             // Parsing a pattern of the form `fieldname: pat`.
             let fieldname = self.parse_field_name()?;
             self.bump();
-            let pat = self.parse_pat_allow_top_alt(None, RecoverComma::No)?;
+            let pat = self.parse_pat_allow_top_alt(None, RecoverComma::No, RecoverColon::No)?;
             hi = pat.span;
             (pat, fieldname, false)
         } else {
