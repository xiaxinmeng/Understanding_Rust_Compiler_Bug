plain
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking tera v1.10.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0277]: can't compare `&types::DocFragment` with `std::option::Option<&types::DocFragment>`
    --> src/librustdoc/clean/types.rs:1092:25
     |
1092 |             if new_frag == self.doc_strings.last() {
     |                         ^^ no implementation for `&types::DocFragment == std::option::Option<&types::DocFragment>`
     |
     = help: the trait `std::cmp::PartialEq<std::option::Option<&types::DocFragment>>` is not implemented for `&types::DocFragment`
error[E0308]: mismatched types
    --> src/librustdoc/doctest.rs:1128:16
     |
     |
1128 |         if let Some(doc) = attrs.collapsed_doc_value() {
     |                ^^^^^^^^^   --------------------------- this expression has type `std::string::String`
     |                expected struct `std::string::String`, found enum `std::option::Option`
     |
     = note: expected struct `std::string::String`
                  found enum `std::option::Option<_>`
                  found enum `std::option::Option<_>`

error[E0277]: the size for values of type `str` cannot be known at compilation time
    --> src/librustdoc/doctest.rs:1128:21
     |
1128 |         if let Some(doc) = attrs.collapsed_doc_value() {
     |
     = help: the trait `std::marker::Sized` is not implemented for `str`
     = note: all local variables must have a statically known size
     = help: unsized locals are gated as an unstable feature
     = help: unsized locals are gated as an unstable feature

error[E0277]: the size for values of type `str` cannot be known at compilation time
    --> src/librustdoc/doctest.rs:1128:16
     |
1128 |         if let Some(doc) = attrs.collapsed_doc_value() {
     |
     = help: the trait `std::marker::Sized` is not implemented for `str`
     = help: the trait `std::marker::Sized` is not implemented for `str`
note: required by a bound in `doctest::_::_serde::__private::Some`
     |
514  | pub enum Option<T> {
514  | pub enum Option<T> {
     |                 ^ required by this bound in `doctest::_::_serde::__private::Some`
error[E0308]: mismatched types
   --> src/librustdoc/html/render/mod.rs:570:12
    |
    |
570 |     if let Some(s) = item.collapsed_doc_value() {
    |            ^^^^^^^   -------------------------- this expression has type `std::string::String`
    |            expected struct `std::string::String`, found enum `std::option::Option`
    |
    = note: expected struct `std::string::String`
                 found enum `std::option::Option<_>`
                 found enum `std::option::Option<_>`

error[E0277]: the size for values of type `str` cannot be known at compilation time
   --> src/librustdoc/html/render/mod.rs:570:17
    |
570 |     if let Some(s) = item.collapsed_doc_value() {
    |
    = help: the trait `std::marker::Sized` is not implemented for `str`
    = note: all local variables must have a statically known size
    = help: unsized locals are gated as an unstable feature
    = help: unsized locals are gated as an unstable feature

error[E0277]: the size for values of type `str` cannot be known at compilation time
   --> src/librustdoc/html/render/mod.rs:570:12
    |
570 |     if let Some(s) = item.collapsed_doc_value() {
    |
    = help: the trait `std::marker::Sized` is not implemented for `str`
    = help: the trait `std::marker::Sized` is not implemented for `str`
note: required by a bound in `doctest::_::_serde::__private::Some`
    |
514 | pub enum Option<T> {
514 | pub enum Option<T> {
    |                 ^ required by this bound in `doctest::_::_serde::__private::Some`
error[E0277]: the size for values of type `str` cannot be known at compilation time
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/tracing-0.1.28/src/macros.rs:670:25
     |
586  | / macro_rules! event {
586  | / macro_rules! event {
587  | |     (target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* } )=> (
588  | |         $crate::__tracing_log!(
589  | |             target: $target,
...    |
670  | |             { message = format_args!($($arg)+), $($fields)* }
     | |                         |
     | |                         doesn't have a size known at compile-time
     | |                         in this macro invocation (#3)
...    |
...    |
791  | |     );
792  | | }
     | |_- in this expansion of `$crate::event!` (#2)
1017 | / macro_rules! debug {
1017 | / macro_rules! debug {
1018 | |     (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
1019 | |         $crate::event!(target: $target, parent: $parent, $crate::Level::DEBUG, { $($field)* }, $($arg)*)
...    |
1186 | /         $crate::event!(
1186 | /         $crate::event!(
1187 |               target: module_path!(),
1188 |               $crate::Level::DEBUG,
1189 |               {},
1191 | |         )
     | |_________- in this macro invocation (#2)
1192 | |     );
1193 | | }
1193 | | }
     | |_- in this expansion of `debug!` (#1)
    ::: src/librustdoc/html/render/mod.rs:571:9
     |
     |
571  |           debug!("Doc block: =====\n{}\n=====", s);
     |
    ::: /checkout/library/core/src/macros/mod.rs:839:5
     |
839  | /     macro_rules! format_args {
839  | /     macro_rules! format_args {
840  | |         ($fmt:expr) => {{ /* compiler built-in */ }};
841  | |         ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
     | |_____- in this expansion of `format_args!` (#3)
     |
     = help: the trait `std::marker::Sized` is not implemented for `str`
     = help: the trait `std::marker::Sized` is not implemented for `str`
note: required by a bound in `ArgumentV1::<'a>::new`
     |
     |
314  |     pub fn new<'b, T>(x: &'b T, f: fn(&T, &mut Formatter<'_>) -> Result) -> ArgumentV1<'b> {
     |                    ^ required by this bound in `ArgumentV1::<'a>::new`
error[E0308]: mismatched types
    --> src/librustdoc/html/render/mod.rs:1615:16
     |
     |
1615 |         if let Some(ref dox) = i.impl_item.collapsed_doc_value() {
     |                ^^^^^^^^^^^^^   --------------------------------- this expression has type `std::string::String`
     |                expected struct `std::string::String`, found enum `std::option::Option`
     |
     = note: expected struct `std::string::String`
                  found enum `std::option::Option<_>`
                  found enum `std::option::Option<_>`

error[E0277]: the size for values of type `str` cannot be known at compilation time
    --> src/librustdoc/html/render/mod.rs:1615:16
     |
1615 |         if let Some(ref dox) = i.impl_item.collapsed_doc_value() {
     |
     = help: the trait `std::marker::Sized` is not implemented for `str`
     = help: the trait `std::marker::Sized` is not implemented for `str`
note: required by a bound in `doctest::_::_serde::__private::Some`
     |
514  | pub enum Option<T> {
514  | pub enum Option<T> {
     |                 ^ required by this bound in `doctest::_::_serde::__private::Some`
error[E0308]: mismatched types
  --> src/librustdoc/json/conversions.rs:54:13
   |
54 |             docs,
54 |             docs,
   |             ^^^^ expected enum `std::option::Option`, found struct `std::string::String`
   |
   = note: expected enum `std::option::Option<std::string::String>`
            found struct `std::string::String`
help: try wrapping the expression in `doctest::_::_serde::__private::Some`
   |
54 |             doctest::_::_serde::__private::Some(docs),
   |             ++++++++++++++++++++++++++++++++++++    +
error[E0599]: no method named `unwrap_or_default` found for struct `std::string::String` in the current scope
  --> src/librustdoc/passes/bare_urls.rs:71:52
   |
   |
71 |         let dox = item.attrs.collapsed_doc_value().unwrap_or_default();

error[E0599]: no method named `unwrap_or_else` found for struct `std::string::String` in the current scope
  --> src/librustdoc/passes/check_doc_test_visibility.rs:38:52
   |
   |
38 |         let dox = item.attrs.collapsed_doc_value().unwrap_or_else(String::new);

error[E0308]: mismatched types
   --> src/librustdoc/passes/check_code_block_syntax.rs:148:16
    |
    |
148 |         if let Some(dox) = &item.attrs.collapsed_doc_value() {
    |                ^^^^^^^^^   --------------------------------- this expression has type `&std::string::String`
    |                expected struct `std::string::String`, found enum `std::option::Option`
    |
    = note: expected struct `std::string::String`
                 found enum `std::option::Option<_>`
                 found enum `std::option::Option<_>`

error[E0277]: the size for values of type `str` cannot be known at compilation time
   --> src/librustdoc/passes/check_code_block_syntax.rs:148:16
    |
148 |         if let Some(dox) = &item.attrs.collapsed_doc_value() {
    |
    = help: the trait `std::marker::Sized` is not implemented for `str`
    = help: the trait `std::marker::Sized` is not implemented for `str`
note: required by a bound in `doctest::_::_serde::__private::Some`
    |
514 | pub enum Option<T> {
514 | pub enum Option<T> {
    |                 ^ required by this bound in `doctest::_::_serde::__private::Some`
error[E0599]: no method named `unwrap_or_default` found for struct `std::string::String` in the current scope
   --> src/librustdoc/passes/calculate_doc_coverage.rs:211:52
    |
    |
211 |                     &i.attrs.collapsed_doc_value().unwrap_or_default(),

error[E0599]: no method named `unwrap_or_default` found for struct `std::string::String` in the current scope
   --> src/librustdoc/passes/html_tags.rs:179:52
    |
    |
179 |         let dox = item.attrs.collapsed_doc_value().unwrap_or_default();

Some errors have detailed explanations: E0277, E0308, E0599.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `rustdoc` due to 17 previous errors
