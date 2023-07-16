
Copying stage1 test from stage1 (x86_64-pc-windows-gnu -> x86_64-pc-windows-gnu / x86_64-pc-windows-gnu)
Building stage1 compiler artifacts (x86_64-pc-windows-gnu -> x86_64-pc-windows-gnu)
/* ... */
   Compiling syntax_pos v0.0.0 
   Compiling rustc_errors v0.0.0
   Compiling syntax_ext v0.0.0
error: internal compiler error: src\librustc\ty\relate.rs:773: impossible case reached: can't relate: '_#0r with Type(RustaceansAreAwesome)
thread 'rustc' panicked at 'Box<Any>', src\librustc_errors\lib.rs:638:9
stack backtrace:
   0: _report_error
/* debug info was not enabled */
  99: _report_error
query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `&Self: std::ops::DispatchFromDyn<&RustaceansAreAwesome>`
#1 [is_object_safe] determine object safety of trait `lint::LateLintPass`
#2 [check_item_well_formed] processing `lint::context::LintStore`
#3 [analysis] running analysis passes on this crate
end of query stack
