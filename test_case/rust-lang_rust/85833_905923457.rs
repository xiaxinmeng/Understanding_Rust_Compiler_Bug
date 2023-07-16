plain
   |
26 | use crate::clean::types::Span;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`
error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> src/librustdoc/html/highlight.rs:580:26
    |
    |
580 |                         .href_from_span(*span)
    |                          |
    |                          expected 2 arguments
    |
note: associated function defined here
note: associated function defined here
   --> src/librustdoc/html/render/context.rs:307:14
    |
307 |     crate fn href_from_span(&self, span: clean::Span, link_lines: bool) -> Option<String> {

error[E0061]: this function takes 1 argument but 2 arguments were supplied
   --> src/librustdoc/html/render/mod.rs:177:25
    |
    |
177 |     if let Some(l) = cx.src_href(item.span(cx.tcx()), true) {
    |                         |
    |                         expected 1 argument
    |
note: associated function defined here
note: associated function defined here
   --> src/librustdoc/html/render/context.rs:303:19
    |
303 |     pub(super) fn src_href(&self, item: &clean::Item) -> Option<String> {

error[E0061]: this function takes 6 arguments but 3 arguments were supplied
    --> src/librustdoc/html/render/mod.rs:2446:9
     |
     |
2446 |         sources::print_src(w, &contents, edition);
     |         ^^^^^^^^^^^^^^^^^^ -  ---------  ------- supplied 3 arguments
     |         expected 6 arguments
     |
note: function defined here
    --> src/librustdoc/html/sources.rs:247:10
    --> src/librustdoc/html/sources.rs:247:10
     |
247  | crate fn print_src(
     |          ^^^^^^^^^
248  |     buf: &mut Buffer,
249  |     s: &str,
     |     -------
250  |     edition: Edition,
     |     ----------------
     |     ----------------
251  |     file_span: rustc_span::Span,
     |     ---------------------------
252  |     context: &Context<'_>,
253  |     root_path: &str,
     |     ---------------


error[E0624]: associated function `src_href` is private
   --> src/librustdoc/scrape_examples.rs:95:38
    |
95  |                         let url = cx.src_href(Span::from_rustc_span(span), false).unwrap();
    | 
   ::: src/librustdoc/html/render/context.rs:303:5
    |
    |
303 |     pub(super) fn src_href(&self, item: &clean::Item) -> Option<String> {


error[E0599]: no function or associated item named `from_rustc_span` found for struct `types::Span` in the current scope
    --> src/librustdoc/scrape_examples.rs:95:53
     |
95   |                         let url = cx.src_href(Span::from_rustc_span(span), false).unwrap();
     | 
    ::: src/librustdoc/clean/types.rs:1959:1
     |
     |
1959 | crate struct Span(rustc_span::Span);
     | ------------------------------------ function or associated item `from_rustc_span` not found for this
Some errors have detailed explanations: E0061, E0599, E0624.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `rustdoc` due to 6 previous errors
Build completed unsuccessfully in 0:02:37
