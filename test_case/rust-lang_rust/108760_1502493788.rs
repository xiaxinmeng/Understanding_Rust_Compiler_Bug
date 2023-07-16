plain
   Compiling rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
error: diagnostic with static strings only
   --> compiler/rustc_parse/src/parser/generics.rs:184:33
    |
184 | / ...                   this.struct_span_err(
185 | | ...                       where_predicate.span(),
186 | | ...                       "bounds on associated types do not belong here",
187 | | ...                   )
188 | | ...                   .span_label(where_predicate.span(), "belongs in `where` clause")
189 | | ...                   .emit();
    |
    |
    = note: `#[deny(rustc::untranslatable_diagnostic_trivial)]` on by default
error: diagnostic with static strings only
   --> compiler/rustc_parse/src/parser/generics.rs:204:33
    |
204 | / ...                   this.struct_span_err(
204 | / ...                   this.struct_span_err(
205 | | ...                       attrs[0].span,
206 | | ...                       "trailing attribute after generic parameter",
207 | | ...                   )
208 | | ...                   .span_label(attrs[0].span, "attributes must go before parameters")
209 | | ...                   .emit();

error: diagnostic with static strings only
   --> compiler/rustc_parse/src/parser/generics.rs:211:33
    |
    |
211 | / ...                   this.struct_span_err(
212 | | ...                       attrs[0].span,
213 | | ...                       "attribute without generic parameters",
...   |
218 | | ...                   )
219 | | ...                   .emit();
    | |______________________________^
    | |______________________________^

error: diagnostic with static strings only
   --> compiler/rustc_parse/src/parser/generics.rs:307:13
    |
307 | /             self.struct_span_err(
308 | |                 generics.span,
309 | |                 "generic parameters on `where` clauses are reserved for future use",
310 | |             )
311 | |             .span_label(generics.span, "currently unsupported")
312 | |             .emit();

error: diagnostic with static strings only
   --> compiler/rustc_parse/src/parser/path.rs:153:17
    |
---

error: diagnostic with static strings only
   --> compiler/rustc_parse/src/parser/path.rs:616:17
    |
616 | /                 self.struct_span_err(span, "associated lifetimes are not supported")
617 | |                     .span_label(lt.ident.span, "the lifetime is given here")
618 | |                     .help("if you meant to specify a trait object, write `dyn Trait + 'lifetime`")
619 | |                     .emit();

error: diagnostic with static strings only
   --> compiler/rustc_parse/src/parser/ty.rs:810:13
    |
    |
810 | /             self.struct_span_err(
811 | |                 span,
812 | |                 "`~const` may only modify trait bounds, not lifetime bounds",
814 | |             .emit();
    | |____________________^

error: diagnostic with static strings only
error: diagnostic with static strings only
   --> compiler/rustc_parse/src/parser/ty.rs:818:13
    |
818 | /             self.struct_span_err(span, "`?` may only modify trait bounds, not lifetime bounds")
819 | |                 .emit();

error: diagnostic with static strings only
   --> compiler/rustc_parse/src/lexer/mod.rs:210:29
    |
    |
210 | / ...                   self.sess
211 | | ...                       .span_diagnostic
212 | | ...                       .struct_span_err(
213 | | ...                           self.mk_sp(suffix_start, self.pos),
214 | | ...                           "underscore literal suffix is not allowed",
216 | | ...                       .emit();
    | |__________________________________^

   Compiling rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
