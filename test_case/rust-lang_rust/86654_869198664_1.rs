
error[E0308]: mismatched types
  --> src/lib.rs:10:9
   |
10 |     bar(&good);
   |         ^^^^^ expected fn pointer, found fn item
   |
   = note: expected reference `&fn()`
              found reference `&fn() {good}`
