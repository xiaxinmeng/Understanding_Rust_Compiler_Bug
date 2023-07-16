
 Documenting diesel v1.4.8 (/home/rylevick/code/rustc-perf/collector/benchmarks/diesel-1.4.8)
error[E0275]: overflow evaluating the requirement `<_ as query_source::Column>::Table`
    |
    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`diesel`)
note: required for `&expression::operators::Eq<_, _>` to implement `insertable::Insertable<_>`
   --> src/expression/operators.rs:376:21
    |
376 | impl<'a, T, Tab, U> Insertable<Tab> for &'a Eq<T, U>
