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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_parse/src/parser/attr_wrapper.rs at line 1:
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_parse/src/parser/attr_wrapper.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
-use super::{Parser, TokenCursor, TrailingToken, ForceCollect};
 use super::attr;
-use rustc_ast::{self as ast};
+use super::{ForceCollect, Parser, TokenCursor, TrailingToken};
 use rustc_ast::token::{self, Token, TokenKind};
-use rustc_ast::tokenstream::{DelimSpan, Spacing, LazyTokenStream};
 use rustc_ast::tokenstream::{CreateTokenStream, TokenStream, TokenTree, TreeAndSpacing};
+use rustc_ast::tokenstream::{DelimSpan, LazyTokenStream, Spacing};
 use rustc_ast::HasTokens;
+use rustc_ast::{self as ast};
 use rustc_errors::PResult;
 use rustc_span::{Span, DUMMY_SP};
 
Diff in /checkout/compiler/rustc_parse/src/parser/attr_wrapper.rs at line 138:
         ret.finalize_tokens(LazyTokenStream::new(lazy_impl));
         Ok(ret)
-
 }
 
 
 /// Converts a flattened iterator of tokens (including open and close delimiter tokens)
Build completed unsuccessfully in 0:00:21
