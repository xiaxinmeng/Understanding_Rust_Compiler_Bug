diff
diff --git a/src/librustc/hir/lowering.rs b/src/librustc/hir/lowering.rs
index 2238a56b29d..4310bafd781 100644
--- a/src/librustc/hir/lowering.rs
+++ b/src/librustc/hir/lowering.rs
@@ -988,12 +988,10 @@ impl<'a> LoweringContext<'a> {
         // lower attributes (we use the AST version) there is nowhere to keep
         // the `HirId`s. We don't actually need HIR version of attributes anyway.
         Attribute {
-            item: AttrItem {
-                path: attr.path.clone(),
-                tokens: self.lower_token_stream(attr.tokens.clone()),
-            },
             id: attr.id,
             style: attr.style,
+            path: attr.path.clone(),
+            tokens: self.lower_token_stream(attr.tokens.clone()),
             is_sugared_doc: attr.is_sugared_doc,
             span: attr.span,
         }
diff --git a/src/librustc/ich/impls_syntax.rs b/src/librustc/ich/impls_syntax.rs
index 23a2f115e05..bdcf9e42ac2 100644
--- a/src/librustc/ich/impls_syntax.rs
+++ b/src/librustc/ich/impls_syntax.rs
@@ -196,11 +196,6 @@ impl<'a> HashStable<StableHashingContext<'a>> for ast::Path {
     }
 }
 
-impl_stable_hash_for!(struct ::syntax::ast::AttrItem {
-    path,
-    tokens,
-});
-
 impl<'a> HashStable<StableHashingContext<'a>> for ast::Attribute {
     fn hash_stable(&self, hcx: &mut StableHashingContext<'a>, hasher: &mut StableHasher) {
         // Make sure that these have been filtered out.
@@ -208,15 +203,19 @@ impl<'a> HashStable<StableHashingContext<'a>> for ast::Attribute {
         debug_assert!(!self.is_sugared_doc);
 
         let ast::Attribute {
-            ref item,
             id: _,
             style,
+            ref path,
+            ref tokens,
             is_sugared_doc: _,
             span,
         } = *self;
 
-        item.hash_stable(hcx, hasher);
         style.hash_stable(hcx, hasher);
+        path.hash_stable(hcx, hasher);
+        for tt in tokens.trees() {
+            tt.hash_stable(hcx, hasher);
+        }
         span.hash_stable(hcx, hasher);
     }
 }
diff --git a/src/libsyntax/ast.rs b/src/libsyntax/ast.rs
index 023952042e6..7a5c92167bc 100644
--- a/src/libsyntax/ast.rs
+++ b/src/libsyntax/ast.rs
@@ -2139,29 +2139,18 @@ impl rustc_serialize::Decodable for AttrId {
     }
 }
 
-#[derive(Clone, RustcEncodable, RustcDecodable, Debug)]
-pub struct AttrItem {
-    pub path: Path,
-    pub tokens: TokenStream,
-}
-
 /// Metadata associated with an item.
 /// Doc-comments are promoted to attributes that have `is_sugared_doc = true`.
 #[derive(Clone, RustcEncodable, RustcDecodable, Debug)]
 pub struct Attribute {
-    pub item: AttrItem,
     pub id: AttrId,
     pub style: AttrStyle,
+    pub path: Path,
+    pub tokens: TokenStream,
     pub is_sugared_doc: bool,
     pub span: Span,
 }
 
-// Compatibility impl to avoid churn, consider removing.
-impl std::ops::Deref for Attribute {
-    type Target = AttrItem;
-    fn deref(&self) -> &Self::Target { &self.item }
-}
-
 /// `TraitRef`s appear in impls.
 ///
 /// Resolution maps each `TraitRef`'s `ref_id` to its defining trait; that's all
diff --git a/src/libsyntax/attr/mod.rs b/src/libsyntax/attr/mod.rs
index 7bef693a5be..122cb7fb12b 100644
--- a/src/libsyntax/attr/mod.rs
+++ b/src/libsyntax/attr/mod.rs
@@ -9,7 +9,7 @@ pub use StabilityLevel::*;
 pub use crate::ast::Attribute;
 
 use crate::ast;
-use crate::ast::{AttrItem, AttrId, AttrStyle, Name, Ident, Path, PathSegment};
+use crate::ast::{AttrId, AttrStyle, Name, Ident, Path, PathSegment};
 use crate::ast::{MetaItem, MetaItemKind, NestedMetaItem};
 use crate::ast::{Lit, LitKind, Expr, Item, Local, Stmt, StmtKind, GenericParam};
 use crate::mut_visit::visit_clobber;
@@ -255,8 +255,9 @@ impl MetaItem {
     }
 }
 
-impl AttrItem {
-    crate fn meta(&self, span: Span) -> Option<MetaItem> {
+impl Attribute {
+    /// Extracts the `MetaItem` from inside this `Attribute`.
+    pub fn meta(&self) -> Option<MetaItem> {
         let mut tokens = self.tokens.trees().peekable();
         Some(MetaItem {
             path: self.path.clone(),
@@ -268,16 +269,9 @@ impl AttrItem {
             } else {
                 return None;
             },
-            span,
+            span: self.span,
         })
     }
-}
-
-impl Attribute {
-    /// Extracts the MetaItem from inside this Attribute.
-    pub fn meta(&self) -> Option<MetaItem> {
-        self.item.meta(self.span)
-    }
 
     pub fn parse<'a, T, F>(&self, sess: &'a ParseSess, mut f: F) -> PResult<'a, T>
         where F: FnMut(&mut Parser<'a>) -> PResult<'a, T>,
@@ -339,9 +333,10 @@ impl Attribute {
                 DUMMY_SP,
             );
             f(&Attribute {
-                item: AttrItem { path: meta.path, tokens: meta.kind.tokens(meta.span) },
                 id: self.id,
                 style: self.style,
+                path: meta.path,
+                tokens: meta.kind.tokens(meta.span),
                 is_sugared_doc: true,
                 span: self.span,
             })
@@ -389,9 +384,10 @@ crate fn mk_attr_id() -> AttrId {
 
 pub fn mk_attr(style: AttrStyle, path: Path, tokens: TokenStream, span: Span) -> Attribute {
     Attribute {
-        item: AttrItem { path, tokens },
         id: mk_attr_id(),
         style,
+        path,
+        tokens,
         is_sugared_doc: false,
         span,
     }
@@ -412,12 +408,10 @@ pub fn mk_sugared_doc_attr(text: Symbol, span: Span) -> Attribute {
     let lit_kind = LitKind::Str(text, ast::StrStyle::Cooked);
     let lit = Lit::from_lit_kind(lit_kind, span);
     Attribute {
-        item: AttrItem {
-            path: Path::from_ident(Ident::with_dummy_span(sym::doc).with_span_pos(span)),
-            tokens: MetaItemKind::NameValue(lit).tokens(span),
-        },
         id: mk_attr_id(),
         style,
+        path: Path::from_ident(Ident::with_dummy_span(sym::doc).with_span_pos(span)),
+        tokens: MetaItemKind::NameValue(lit).tokens(span),
         is_sugared_doc: true,
         span,
     }
@@ -530,7 +524,7 @@ impl MetaItem {
             }
             Some(TokenTree::Token(Token { kind: token::Interpolated(nt), .. })) => match *nt {
                 token::Nonterminal::NtIdent(ident, _) => Path::from_ident(ident),
-                token::Nonterminal::NtMeta(ref item) => return item.meta(item.path.span),
+                token::Nonterminal::NtMeta(ref meta) => return Some(meta.clone()),
                 token::Nonterminal::NtPath(ref path) => path.clone(),
                 _ => return None,
             },
diff --git a/src/libsyntax/config.rs b/src/libsyntax/config.rs
index 2923cc86ba0..990358c674f 100644
--- a/src/libsyntax/config.rs
+++ b/src/libsyntax/config.rs
@@ -122,8 +122,8 @@ impl<'a> StripUnconfigured<'a> {
 
             while !parser.check(&token::CloseDelim(token::Paren)) {
                 let lo = parser.token.span.lo();
-                let item = parser.parse_attr_item()?;
-                expanded_attrs.push((item, parser.prev_span.with_lo(lo)));
+                let (path, tokens) = parser.parse_meta_item_unrestricted()?;
+                expanded_attrs.push((path, tokens, parser.prev_span.with_lo(lo)));
                 parser.expect_one_of(&[token::Comma], &[token::CloseDelim(token::Paren)])?;
             }
 
@@ -150,10 +150,11 @@ impl<'a> StripUnconfigured<'a> {
             // `cfg_attr` inside of another `cfg_attr`. E.g.
             //  `#[cfg_attr(false, cfg_attr(true, some_attr))]`.
             expanded_attrs.into_iter()
-            .flat_map(|(item, span)| self.process_cfg_attr(ast::Attribute {
-                item,
+            .flat_map(|(path, tokens, span)| self.process_cfg_attr(ast::Attribute {
                 id: attr::mk_attr_id(),
                 style: attr.style,
+                path,
+                tokens,
                 is_sugared_doc: false,
                 span,
             }))
diff --git a/src/libsyntax/ext/expand.rs b/src/libsyntax/ext/expand.rs
index bbd8da2acef..581ef5d4da9 100644
--- a/src/libsyntax/ext/expand.rs
+++ b/src/libsyntax/ext/expand.rs
@@ -1,4 +1,4 @@
-use crate::ast::{self, AttrItem, Block, Ident, LitKind, NodeId, PatKind, Path};
+use crate::ast::{self, Block, Ident, LitKind, NodeId, PatKind, Path};
 use crate::ast::{MacStmtStyle, StmtKind, ItemKind};
 use crate::attr::{self, HasAttrs};
 use crate::source_map::respan;
@@ -617,10 +617,9 @@ impl<'a, 'b> MacroExpander<'a, 'b> {
                         | Annotatable::Variant(..)
                             => panic!("unexpected annotatable"),
                     })), DUMMY_SP).into();
-                    let input = self.extract_proc_macro_attr_input(attr.item.tokens, span);
+                    let input = self.extract_proc_macro_attr_input(attr.tokens, span);
                     let tok_result = expander.expand(self.cx, span, input, item_tok);
-                    let res =
-                        self.parse_ast_fragment(tok_result, fragment_kind, &attr.item.path, span);
+                    let res = self.parse_ast_fragment(tok_result, fragment_kind, &attr.path, span);
                     self.gate_proc_macro_expansion(span, &res);
                     res
                 }
@@ -1523,10 +1522,11 @@ impl<'a, 'b> MutVisitor for InvocationCollector<'a, 'b> {
 
             let meta = attr::mk_list_item(Ident::with_dummy_span(sym::doc), items);
             *at = attr::Attribute {
-                item: AttrItem { path: meta.path, tokens: meta.kind.tokens(meta.span) },
                 span: at.span,
                 id: at.id,
                 style: at.style,
+                path: meta.path,
+                tokens: meta.kind.tokens(meta.span),
                 is_sugared_doc: false,
             };
         } else {
diff --git a/src/libsyntax/ext/mbe/macro_parser.rs b/src/libsyntax/ext/mbe/macro_parser.rs
index d1c50fd8594..8f49ba9572d 100644
--- a/src/libsyntax/ext/mbe/macro_parser.rs
+++ b/src/libsyntax/ext/mbe/macro_parser.rs
@@ -924,7 +924,7 @@ fn parse_nt(p: &mut Parser<'_>, sp: Span, name: Symbol) -> Nonterminal {
             FatalError.raise()
         }
         sym::path => token::NtPath(panictry!(p.parse_path(PathStyle::Type))),
-        sym::meta => token::NtMeta(panictry!(p.parse_attr_item())),
+        sym::meta => token::NtMeta(panictry!(p.parse_meta_item())),
         sym::vis => token::NtVis(panictry!(p.parse_visibility(true))),
         sym::lifetime => if p.check_lifetime() {
             token::NtLifetime(p.expect_lifetime().ident)
diff --git a/src/libsyntax/mut_visit.rs b/src/libsyntax/mut_visit.rs
index 3923b9f297b..80dfe9e5be0 100644
--- a/src/libsyntax/mut_visit.rs
+++ b/src/libsyntax/mut_visit.rs
@@ -550,8 +550,7 @@ pub fn noop_visit_local<T: MutVisitor>(local: &mut P<Local>, vis: &mut T) {
 }
 
 pub fn noop_visit_attribute<T: MutVisitor>(attr: &mut Attribute, vis: &mut T) {
-    let Attribute { item: AttrItem { path, tokens }, id: _, style: _, is_sugared_doc: _, span }
-        = attr;
+    let Attribute { id: _, style: _, path, tokens, is_sugared_doc: _, span } = attr;
     vis.visit_path(path);
     vis.visit_tts(tokens);
     vis.visit_span(span);
@@ -682,10 +681,7 @@ pub fn noop_visit_interpolated<T: MutVisitor>(nt: &mut token::Nonterminal, vis:
         token::NtIdent(ident, _is_raw) => vis.visit_ident(ident),
         token::NtLifetime(ident) => vis.visit_ident(ident),
         token::NtLiteral(expr) => vis.visit_expr(expr),
-        token::NtMeta(AttrItem { path, tokens }) => {
-            vis.visit_path(path);
-            vis.visit_tts(tokens);
-        }
+        token::NtMeta(meta) => vis.visit_meta_item(meta),
         token::NtPath(path) => vis.visit_path(path),
         token::NtTT(tt) => vis.visit_tt(tt),
         token::NtImplItem(item) =>
diff --git a/src/libsyntax/parse/attr.rs b/src/libsyntax/parse/attr.rs
index e74f3045db8..44688bd36b5 100644
--- a/src/libsyntax/parse/attr.rs
+++ b/src/libsyntax/parse/attr.rs
@@ -90,7 +90,7 @@ impl<'a> Parser<'a> {
         debug!("parse_attribute_with_inner_parse_policy: inner_parse_policy={:?} self.token={:?}",
                inner_parse_policy,
                self.token);
-        let (span, item, style) = match self.token.kind {
+        let (span, path, tokens, style) = match self.token.kind {
             token::Pound => {
                 let lo = self.token.span;
                 self.bump();
@@ -107,7 +107,7 @@ impl<'a> Parser<'a> {
                 };
 
                 self.expect(&token::OpenDelim(token::Bracket))?;
-                let item = self.parse_attr_item()?;
+                let (path, tokens) = self.parse_meta_item_unrestricted()?;
                 self.expect(&token::CloseDelim(token::Bracket))?;
                 let hi = self.prev_span;
 
@@ -142,7 +142,7 @@ impl<'a> Parser<'a> {
                     }
                 }
 
-                (attr_sp, item, style)
+                (attr_sp, path, tokens, style)
             }
             _ => {
                 let token_str = self.this_token_to_string();
@@ -151,9 +151,10 @@ impl<'a> Parser<'a> {
         };
 
         Ok(ast::Attribute {
-            item,
             id: attr::mk_attr_id(),
             style,
+            path,
+            tokens,
             is_sugared_doc: false,
             span,
         })
@@ -166,19 +167,19 @@ impl<'a> Parser<'a> {
     ///     PATH `[` TOKEN_STREAM `]`
     ///     PATH `{` TOKEN_STREAM `}`
     ///     PATH
-    ///     PATH `=` UNSUFFIXED_LIT
+    ///     PATH `=` TOKEN_TREE
     /// The delimiters or `=` are still put into the resulting token stream.
-    pub fn parse_attr_item(&mut self) -> PResult<'a, ast::AttrItem> {
-        let item = match self.token.kind {
+    pub fn parse_meta_item_unrestricted(&mut self) -> PResult<'a, (ast::Path, TokenStream)> {
+        let meta = match self.token.kind {
             token::Interpolated(ref nt) => match **nt {
-                Nonterminal::NtMeta(ref item) => Some(item.clone()),
+                Nonterminal::NtMeta(ref meta) => Some(meta.clone()),
                 _ => None,
             },
             _ => None,
         };
-        Ok(if let Some(item) = item {
+        Ok(if let Some(meta) = meta {
             self.bump();
-            item
+            (meta.path, meta.kind.tokens(meta.span))
         } else {
             let path = self.parse_path(PathStyle::Mod)?;
             let tokens = if self.check(&token::OpenDelim(DelimToken::Paren)) ||
@@ -205,7 +206,7 @@ impl<'a> Parser<'a> {
             } else {
                 TokenStream::empty()
             };
-            ast::AttrItem { path, tokens }
+            (path, tokens)
         })
     }
 
@@ -262,7 +263,7 @@ impl<'a> Parser<'a> {
 
     /// Matches the following grammar (per RFC 1559).
     ///
-    ///     meta_item : PATH ( '=' UNSUFFIXED_LIT | '(' meta_item_inner? ')' )? ;
+    ///     meta_item : IDENT ( '=' UNSUFFIXED_LIT | '(' meta_item_inner? ')' )? ;
     ///     meta_item_inner : (meta_item | UNSUFFIXED_LIT) (',' meta_item_inner)? ;
     pub fn parse_meta_item(&mut self) -> PResult<'a, ast::MetaItem> {
         let nt_meta = match self.token.kind {
@@ -273,14 +274,9 @@ impl<'a> Parser<'a> {
             _ => None,
         };
 
-        if let Some(item) = nt_meta {
-            return match item.meta(item.path.span) {
-                Some(meta) => {
-                    self.bump();
-                    Ok(meta)
-                }
-                None => self.unexpected(),
-            }
+        if let Some(meta) = nt_meta {
+            self.bump();
+            return Ok(meta);
         }
 
         let lo = self.token.span;
diff --git a/src/libsyntax/parse/parser/path.rs b/src/libsyntax/parse/parser/path.rs
index ca823991a2e..463ae9124ca 100644
--- a/src/libsyntax/parse/parser/path.rs
+++ b/src/libsyntax/parse/parser/path.rs
@@ -114,9 +114,9 @@ impl<'a> Parser<'a> {
     pub fn parse_path_allowing_meta(&mut self, style: PathStyle) -> PResult<'a, Path> {
         let meta_ident = match self.token.kind {
             token::Interpolated(ref nt) => match **nt {
-                token::NtMeta(ref item) => match item.tokens.is_empty() {
-                    true => Some(item.path.clone()),
-                    false => None,
+                token::NtMeta(ref meta) => match meta.kind {
+                    ast::MetaItemKind::Word => Some(meta.path.clone()),
+                    _ => None,
                 },
                 _ => None,
             },
diff --git a/src/libsyntax/parse/token.rs b/src/libsyntax/parse/token.rs
index fd78a2bd534..fe3b51aa246 100644
--- a/src/libsyntax/parse/token.rs
+++ b/src/libsyntax/parse/token.rs
@@ -687,7 +687,7 @@ pub enum Nonterminal {
     NtLifetime(ast::Ident),
     NtLiteral(P<ast::Expr>),
     /// Stuff inside brackets for attributes
-    NtMeta(ast::AttrItem),
+    NtMeta(ast::MetaItem),
     NtPath(ast::Path),
     NtVis(ast::Visibility),
     NtTT(TokenTree),
diff --git a/src/libsyntax/print/pprust.rs b/src/libsyntax/print/pprust.rs
index 7d4ffe493d7..4b9c2d13f26 100644
--- a/src/libsyntax/print/pprust.rs
+++ b/src/libsyntax/print/pprust.rs
@@ -324,7 +324,7 @@ fn token_to_string_ext(token: &Token, convert_dollar_crate: bool) -> String {
 crate fn nonterminal_to_string(nt: &Nonterminal) -> String {
     match *nt {
         token::NtExpr(ref e)        => expr_to_string(e),
-        token::NtMeta(ref e)        => attr_item_to_string(e),
+        token::NtMeta(ref e)        => meta_item_to_string(e),
         token::NtTy(ref e)          => ty_to_string(e),
         token::NtPath(ref e)        => path_to_string(e),
         token::NtItem(ref e)        => item_to_string(e),
@@ -412,8 +412,8 @@ pub fn meta_list_item_to_string(li: &ast::NestedMetaItem) -> String {
     to_string(|s| s.print_meta_list_item(li))
 }
 
-fn attr_item_to_string(ai: &ast::AttrItem) -> String {
-    to_string(|s| s.print_attr_item(ai, ai.path.span))
+pub fn meta_item_to_string(mi: &ast::MetaItem) -> String {
+    to_string(|s| s.print_meta_item(mi))
 }
 
 pub fn attribute_to_string(attr: &ast::Attribute) -> String {
@@ -629,28 +629,24 @@ pub trait PrintState<'a>: std::ops::Deref<Target = pp::Printer> + std::ops::Dere
                 ast::AttrStyle::Inner => self.word("#!["),
                 ast::AttrStyle::Outer => self.word("#["),
             }
-            self.print_attr_item(&attr.item, attr.span);
-            self.word("]");
-        }
-    }
-
-    fn print_attr_item(&mut self, item: &ast::AttrItem, span: Span) {
-        self.ibox(0);
-        match item.tokens.trees().next() {
-            Some(TokenTree::Delimited(_, delim, tts)) => {
-                self.print_mac_common(
-                    Some(MacHeader::Path(&item.path)), false, None, delim, tts, true, span
-                );
-            }
-            tree => {
-                self.print_path(&item.path, false, 0);
-                if tree.is_some() {
-                    self.space();
-                    self.print_tts(item.tokens.clone(), true);
+            self.ibox(0);
+            match attr.tokens.trees().next() {
+                Some(TokenTree::Delimited(_, delim, tts)) => {
+                    self.print_mac_common(
+                        Some(MacHeader::Path(&attr.path)), false, None, delim, tts, true, attr.span
+                    );
+                }
+                tree => {
+                    self.print_path(&attr.path, false, 0);
+                    if tree.is_some() {
+                        self.space();
+                        self.print_tts(attr.tokens.clone(), true);
+                    }
                 }
             }
+            self.end();
+            self.word("]");
         }
-        self.end();
     }
 
     fn print_meta_list_item(&mut self, item: &ast::NestedMetaItem) {
diff --git a/src/libsyntax_ext/cmdline_attrs.rs b/src/libsyntax_ext/cmdline_attrs.rs
index 203c4a83489..bb8e3df3db9 100644
--- a/src/libsyntax_ext/cmdline_attrs.rs
+++ b/src/libsyntax_ext/cmdline_attrs.rs
@@ -1,6 +1,6 @@
 //! Attributes injected into the crate root from command line using `-Z crate-attr`.
 
-use syntax::ast::{self, AttrItem, AttrStyle};
+use syntax::ast::{self, AttrStyle};
 use syntax::attr::mk_attr;
 use syntax::panictry;
 use syntax::parse::{self, token, ParseSess};
@@ -15,7 +15,7 @@ pub fn inject(mut krate: ast::Crate, parse_sess: &ParseSess, attrs: &[String]) -
         );
 
         let start_span = parser.token.span;
-        let AttrItem { path, tokens } = panictry!(parser.parse_attr_item());
+        let (path, tokens) = panictry!(parser.parse_meta_item_unrestricted());
         let end_span = parser.token.span;
         if parser.token != token::Eof {
             parse_sess.span_diagnostic
diff --git a/src/test/ui/cfg/cfg_stmt_expr.rs b/src/test/ui/cfg/cfg_stmt_expr.rs
index 6381bb2d588..e466ad69f72 100644
--- a/src/test/ui/cfg/cfg_stmt_expr.rs
+++ b/src/test/ui/cfg/cfg_stmt_expr.rs
@@ -57,7 +57,7 @@ fn main() {
     // check that macro expanded code works
 
     macro_rules! if_cfg {
-        ($cfg:meta? $ib:block else $eb:block) => {
+        ($cfg:meta $ib:block else $eb:block) => {
             {
                 let r;
                 #[cfg($cfg)]
@@ -69,7 +69,7 @@ fn main() {
         }
     }
 
-    let n = if_cfg!(unset? {
+    let n = if_cfg!(unset {
         413
     } else {
         612
diff --git a/src/test/ui/macros/macro-first-set.rs b/src/test/ui/macros/macro-first-set.rs
index eb2504d4bfd..34529cdaa64 100644
--- a/src/test/ui/macros/macro-first-set.rs
+++ b/src/test/ui/macros/macro-first-set.rs
@@ -252,6 +252,12 @@ test_path!(::std);
 test_path!(std::u8,);
 test_path!(any, super, super::super::self::path, X<Y>::Z<'a, T=U>);
 
+macro_rules! test_meta_block {
+    ($($m:meta)* $b:block) => {};
+}
+
+test_meta_block!(windows {});
+
 macro_rules! test_lifetime {
     (1. $($l:lifetime)* $($b:block)*) => {};
     (2. $($b:block)* $($l:lifetime)*) => {};
diff --git a/src/test/ui/macros/macro-meta-items-modern.rs b/src/test/ui/macros/macro-meta-items-modern.rs
deleted file mode 100644
index bc6938d4a6c..00000000000
--- a/src/test/ui/macros/macro-meta-items-modern.rs
+++ /dev/null
@@ -1,11 +0,0 @@
-// check-pass
-
-macro_rules! check { ($meta:meta) => () }
-
-check!(meta(a b c d));
-check!(meta[a b c d]);
-check!(meta { a b c d });
-check!(meta);
-check!(meta = 0);
-
-fn main() {}
