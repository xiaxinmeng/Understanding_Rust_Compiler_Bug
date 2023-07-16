text
error[E0308]: mismatched types
  --> /opt/rustwide/cargo-home/registry/src/github.com-1ecc6299db9ec823/lazy_static-1.1.0/build.rs:13:19
   |
13 |         .filter(|&&f| f)
   |                   ^^
   |                   |
   |                   expected bool, found reference
   |                   help: you can probably remove the explicit borrow: `f`
   |
   = note: expected type `bool`
              found type `&_`

error: aborting due to previous error
