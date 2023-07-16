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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_parse/src/parser/item.rs at line 832:
     }
 
     fn parse_rename(&mut self) -> PResult<'a, Option<Ident>> {
-        if self.eat_keyword(kw::As) {
-            self.parse_ident_or_underscore().map(Some)
-            Ok(None)
-        }
-        }
+        if self.eat_keyword(kw::As) { self.parse_ident_or_underscore().map(Some) } else { Ok(None) }
 
 
     fn parse_ident_or_underscore(&mut self) -> PResult<'a, Ident> {
Diff in /checkout/compiler/rustc_parse/src/parser/item.rs at line 1070:
         &mut self,
         m: Option<Mutability>,
     ) -> PResult<'a, (Ident, P<Ty>, Option<P<ast::Expr>>)> {
-        let id =
-            if m.is_none() { self.parse_ident_or_underscore() } else { self.parse_ident() }?;
+        let id = if m.is_none() { self.parse_ident_or_underscore() } else { self.parse_ident() }?;
 
         // Parse the type of a `const` or `static mut?` item.
         // That is, the `":" $ty` fragment.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_parse/src/parser/item.rs" "/checkout/compiler/rustc_parse/src/parser/diagnostics.rs" "/checkout/compiler/rustc_parse/src/parser/ty.rs" "/checkout/compiler/rustc_parse/src/parser/stmt.rs" "/checkout/compiler/rustc_parse/src/parser/path.rs" "/checkout/compiler/rustc_parse/src/lexer/unicode_chars.rs" "/checkout/compiler/rustc_parse/src/lexer/unescape_error_reporting.rs" "/checkout/compiler/rustc_parse/src/parser/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:13
