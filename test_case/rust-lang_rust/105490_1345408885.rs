plain
   Compiling rustc_apfloat v0.0.0 (/checkout/compiler/rustc_apfloat)
   Compiling tinyvec_macros v0.1.0
   Compiling rustc_parse_format v0.0.0 (/checkout/compiler/rustc_parse_format)
   Compiling tinyvec v1.6.0
error[E0283]: type annotations needed: cannot satisfy `for<'a, 'b> F: FnMut<(&'a mut K, &'b mut V)>`
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/indexmap-1.9.1/src/mutable_keys.rs:67:12
   |
67 |         F: FnMut(&mut K, &mut V) -> bool,
   |
   |
   = note: cannot satisfy `for<'a, 'b> F: FnMut<(&'a mut K, &'b mut V)>`
note: the requirement `for<'a, 'b> F: FnMut<(&'a mut K, &'b mut V)>` appears on the `impl`'s method `retain2` but not on the corresponding trait's method
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/indexmap-1.9.1/src/mutable_keys.rs:38:8
19 | pub trait MutableKeys {
   |           ----------- in this trait
...
...
38 |     fn retain2<F>(&mut self, keep: F)
   |        ^^^^^^^ this trait's method doesn't have the requirement `for<'a, 'b> F: FnMut<(&'a mut K, &'b mut V)>`
   Compiling regex-syntax v0.6.26
   Compiling block-buffer v0.10.2
   Compiling crypto-common v0.1.2
   Compiling polonius-engine v0.13.0
