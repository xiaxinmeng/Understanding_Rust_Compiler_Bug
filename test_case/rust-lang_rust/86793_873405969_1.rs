
error[E0308]: mismatched types
  --> src/main.rs:13:12
   |
6  | fn foo() -> impl Trait { S }
   |             ---------- the found opaque type
7  | 
8  | type Concrete = impl Trait;
   |                 ---------- the expected opaque type
...
13 |     v.push(foo());
   |            ^^^^^ expected opaque type, found a different opaque type
   |
   = note: expected opaque type `impl Trait` (opaque type at <src/main.rs:8:17>)
              found opaque type `impl Trait` (opaque type at <src/main.rs:6:13>)
   = note: distinct uses of `impl Trait` result in different opaque types

error: aborting due to previous error
