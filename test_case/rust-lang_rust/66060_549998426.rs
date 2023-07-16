rust
// revisions: cfail1 cfail2
// error-pattern: delayed span bug triggered by #[rustc_error(delay_span_bug_from_inside_query)]
// failure-status: 101

#![feature(rustc_attrs)]

#[rustc_error(delay_span_bug_from_inside_query)]
fn main() {}
