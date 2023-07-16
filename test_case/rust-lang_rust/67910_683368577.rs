
error[E0759]: `x` has an anonymous lifetime `'_` but it needs to satisfy a `'static` lifetime requirement
  --> $DIR/must_outlive_least_region_or_bound.rs:7:46
   |
LL | fn elided2(x: &i32) -> impl Copy + 'static { x }
   |               ----                           ^ ...is captured here...
   |               |
   |               this data with an anonymous lifetime `'_`...
   |
note: ...and is required to live as long as `'static` here
  --> $DIR/must_outlive_least_region_or_bound.rs:7:24
   |
LL | fn elided2(x: &i32) -> impl Copy + 'static { x }
   |                        ^^^^^^^^^^^^^^^^^^^
help: consider changing the `impl Trait`'s explicit `'static` bound to the lifetime of argument `x`
   |
LL | fn elided2(x: &i32) -> impl Copy + '_ { x }
   |                                    ^^
help: alternatively, add an explicit `'static` bound to this reference
   |
LL | fn elided2(x: &'static i32) -> impl Copy + 'static { x }
   |               ^^^^^^^^^^^^
