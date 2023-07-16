rust
diff --git a/compiler/rustc_expand/src/mbe/quoted.rs b/compiler/rustc_expand/src/mbe/quoted.rs
index d4b8563a036..d7049eb28a5 100644
--- a/compiler/rustc_expand/src/mbe/quoted.rs
+++ b/compiler/rustc_expand/src/mbe/quoted.rs
@@ -9,7 +9,7 @@
 use rustc_span::symbol::{kw, sym, Ident};
 
 use rustc_span::edition::Edition;
-use rustc_span::{Span, SyntaxContext};
+use rustc_span::{ExpnId, Span, SyntaxContext};
 
 const VALID_FRAGMENT_NAMES_MSG: &str = "valid fragment specifiers are \
                                         `ident`, `block`, `stmt`, `expr`, `pat`, `ty`, `lifetime`, \
@@ -221,6 +221,12 @@ fn parse_tree(
                     let (ident, is_raw) = token.ident().unwrap();
                     let span = ident.span.with_lo(span.lo());
                     if ident.name == kw::Crate && !is_raw {
+                        // If we use `ident.span` here, $crate will refer to the crate where
+                        // the crate token appears; we want it to refer to the crate which
+                        // is actually defining this macro (the current local crate). This is
+                        // most easily accomplished with a macro expanded macro_rules! which
+                        // uses `$$ crate` to get a `$crate` in the expanded macro definition.
+                        let span = span.with_call_site_ctxt(ExpnId::root());
                         TokenTree::token(token::Ident(kw::DollarCrate, is_raw), span)
                     } else {
                         TokenTree::MetaVar(span, ident)
