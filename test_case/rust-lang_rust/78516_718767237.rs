
---- [ui] ui/associated-types/associated-types-incomplete-object.rs stdout ----
 stderr:
------------------------------------------
thread 'rustc' panicked at 'assertion failed: eps.array_windows().all(|[a, b]| a.stable_cmp(self, b) != Ordering::Greater)', compiler/rustc_middle/src/ty/context.rs:2419:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

 query stack during panic:
#0 [object_safety_violations] determine object safety of trait `Foo`
#1 [typeck] type-checking `main`
end of query stack
