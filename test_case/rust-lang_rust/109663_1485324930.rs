plain
    Checking rustc_macros v0.1.0 (/checkout/compiler/rustc_macros)
error[E0432]: unresolved import `syn::NestedMeta`
  --> compiler/rustc_macros/src/diagnostics/diagnostic_builder.rs:15:78
   |
15 |     parse_quote, spanned::Spanned, Attribute, Meta, MetaList, MetaNameValue, NestedMeta, Path, Type,
   |                                                                              ^^^^^^^^^^ no `NestedMeta` in the root
error[E0432]: unresolved import `syn::NestedMeta`
 --> compiler/rustc_macros/src/diagnostics/error.rs:4:65
  |
  |
4 | use syn::{spanned::Spanned, Attribute, Error as SynError, Meta, NestedMeta};
  |                                                                 ^^^^^^^^^^ no `NestedMeta` in the root
error[E0432]: unresolved import `syn::NestedMeta`
  --> compiler/rustc_macros/src/diagnostics/subdiagnostic.rs:16:56
   |
   |
16 | use syn::{spanned::Spanned, Attribute, Meta, MetaList, NestedMeta, Path};
   |                                                        ^^^^^^^^^^ no `NestedMeta` in the root
error[E0433]: failed to resolve: could not find `NestedMeta` in `syn`
   --> compiler/rustc_macros/src/diagnostics/error.rs:108:14
    |
    |
108 |         syn::NestedMeta::Meta(meta) => meta,
    |              ^^^^^^^^^^ could not find `NestedMeta` in `syn`
error[E0433]: failed to resolve: could not find `NestedMeta` in `syn`
   --> compiler/rustc_macros/src/diagnostics/error.rs:109:14
    |
    |
109 |         syn::NestedMeta::Lit(_) => {
    |              ^^^^^^^^^^ could not find `NestedMeta` in `syn`
error[E0433]: failed to resolve: use of undeclared type `NestedMeta`
   --> compiler/rustc_macros/src/diagnostics/utils.rs:695:32
    |
    |
695 |         let slug = if let Some(NestedMeta::Meta(Meta::Path(path))) = nested_iter.peek() {
    |                                ^^^^^^^^^^ use of undeclared type `NestedMeta`
error[E0433]: failed to resolve: use of undeclared type `NestedMeta`
   --> compiler/rustc_macros/src/diagnostics/utils.rs:706:17
    |
    |
706 |                 NestedMeta::Meta(ref meta) => meta,
    |                 ^^^^^^^^^^ use of undeclared type `NestedMeta`
error[E0433]: failed to resolve: use of undeclared type `NestedMeta`
   --> compiler/rustc_macros/src/diagnostics/utils.rs:707:17
    |
    |
707 |                 NestedMeta::Lit(_) => {
    |                 ^^^^^^^^^^ use of undeclared type `NestedMeta`

error[E0599]: no method named `parse_meta` found for reference `&syn::Attribute` in the current scope
    |
    |
163 |             let meta = attr.parse_meta()?;


error[E0599]: no method named `parse_meta` found for reference `&syn::Attribute` in the current scope
    |
    |
196 |         let meta = attr.parse_meta()?;


error[E0026]: struct `MetaList` does not have a field named `nested`
    |
    |
199 |             let Meta::List(MetaList { ref nested, .. }) = meta else {
    |                                           ^^^^^^ struct `MetaList` does not have this field

error[E0026]: struct `syn::MetaNameValue` does not have a field named `lit`
    |
    |
225 |                         lit: syn::Lit::Str(value),
    |                         ^^^ struct `syn::MetaNameValue` does not have this field

error[E0615]: attempted to take value of method `path` on type `&syn::Attribute`
    |
    |
312 |                 let name = attr.path.segments.last().unwrap().ident.to_string();
    |
help: use parentheses to call the method
    |
    |
312 |                 let name = attr.path().segments.last().unwrap().ident.to_string();


error[E0599]: no method named `parse_meta` found for reference `&syn::Attribute` in the current scope
    |
    |
346 |         let meta = attr.parse_meta()?;


error[E0615]: attempted to take value of method `path` on type `&syn::Attribute`
    |
    |
348 |         let ident = &attr.path.segments.last().unwrap().ident;
    |
help: use parentheses to call the method
    |
    |
348 |         let ident = &attr.path().segments.last().unwrap().ident;


error[E0026]: struct `MetaList` does not have a field named `nested`
    |
    |
381 |             (Meta::List(MetaList { ref nested, .. }), "subdiagnostic") => {
    |                                        ^^^^^^ struct `MetaList` does not have this field

error[E0615]: attempted to take value of method `path` on type `&syn::Attribute`
    |
    |
103 |     let name = attr.path.segments.last().unwrap().ident.to_string();
    |
help: use parentheses to call the method
    |
    |
103 |     let name = attr.path().segments.last().unwrap().ident.to_string();


error[E0599]: no method named `parse_meta` found for reference `&syn::Attribute` in the current scope
    |
    |
270 |         let meta = attr.parse_meta()?;


error[E0615]: attempted to take value of method `path` on type `&syn::Attribute`
    |
    |
608 |         let name = attr.path.segments.last().unwrap().ident.to_string();
    |
help: use parentheses to call the method
    |
    |
608 |         let name = attr.path().segments.last().unwrap().ident.to_string();


error[E0599]: no method named `parse_meta` found for reference `&syn::Attribute` in the current scope
    |
    |
611 |         let meta = attr.parse_meta()?;


error[E0026]: struct `MetaList` does not have a field named `nested`
    |
    |
660 |             Meta::List(MetaList { ref nested, .. }) => {
    |                                       ^^^^^^ struct `MetaList` does not have this field

error[E0026]: struct `syn::MetaNameValue` does not have a field named `lit`
    |
    |
718 |                 Meta::NameValue(MetaNameValue { lit: syn::Lit::Str(value), .. }) => Some(value),
    |                                                 ^^^ struct `syn::MetaNameValue` does not have this field
error[E0308]: mismatched types
   --> compiler/rustc_macros/src/diagnostics/utils.rs:729:25
    |
728 |                     let code_init = build_suggestion_code(
728 |                     let code_init = build_suggestion_code(
    |                                     --------------------- arguments to this function are incorrect
729 |                         code_field,
    |                         ^^^^^^^^^^
    |                         |
    |                         expected `&Ident`, found `Ident`
    |
note: function defined here
   --> compiler/rustc_macros/src/diagnostics/utils.rs:424:15
    |
    |
424 | pub(super) fn build_suggestion_code(
    |               ^^^^^^^^^^^^^^^^^^^^^
425 |     code_field: &Ident,


error[E0615]: attempted to take value of method `path` on type `&syn::Attribute`
    |
    |
852 |     attr.path.segments.last().unwrap().ident == "doc"
    |
help: use parentheses to call the method
    |
    |
852 |     attr.path().segments.last().unwrap().ident == "doc"

Some errors have detailed explanations: E0026, E0308, E0432, E0433, E0599, E0615.
For more information about an error, try `rustc --explain E0026`.
error: could not compile `rustc_macros` due to 24 previous errors
