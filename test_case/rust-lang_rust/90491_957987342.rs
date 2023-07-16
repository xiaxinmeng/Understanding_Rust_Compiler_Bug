plain
   Compiling rustc_macros v0.1.0 (/checkout/compiler/rustc_macros)
   Compiling chalk-derive v0.55.0
   Compiling chalk-ir v0.55.0
   Compiling tracing v0.1.28
error[E0716]: temporary value dropped while borrowed
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/serde_derive-1.0.125/src/internals/attr.rs:318:19
318 |             match &meta_item {
318 |             match &meta_item {
    |                   ^^^^^^^^^^ creates a temporary which is freed while still in use
586 |         }
    |         - temporary value is freed at the end of this statement


error[E0597]: `meta_item` does not live long enough
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/serde_derive-1.0.125/src/internals/attr.rs:318:19
318 |             match &meta_item {
318 |             match &meta_item {
    |                   ^^^^^^^^^^ borrowed value does not live long enough
586 |         }
586 |         }
    |         - `meta_item` dropped here while still borrowed

error[E0716]: temporary value dropped while borrowed
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/serde_derive-1.0.125/src/internals/attr.rs:880:19
880  |             match &meta_item {
880  |             match &meta_item {
     |                   ^^^^^^^^^^ creates a temporary which is freed while still in use
1037 |         }
     |         - temporary value is freed at the end of this statement


error[E0597]: `meta_item` does not live long enough
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/serde_derive-1.0.125/src/internals/attr.rs:880:19
880  |             match &meta_item {
880  |             match &meta_item {
     |                   ^^^^^^^^^^ borrowed value does not live long enough
1037 |         }
1037 |         }
     |         - `meta_item` dropped here while still borrowed
   Compiling rustc_index v0.0.0 (/checkout/compiler/rustc_index)
   Compiling rustc_index v0.0.0 (/checkout/compiler/rustc_index)
error[E0716]: temporary value dropped while borrowed
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/serde_derive-1.0.125/src/internals/attr.rs:1182:19
1182 |             match &meta_item {
1182 |             match &meta_item {
     |                   ^^^^^^^^^^ creates a temporary which is freed while still in use
1348 |         }
     |         - temporary value is freed at the end of this statement


error[E0597]: `meta_item` does not live long enough
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/serde_derive-1.0.125/src/internals/attr.rs:1182:19
1182 |             match &meta_item {
1182 |             match &meta_item {
     |                   ^^^^^^^^^^ borrowed value does not live long enough
1348 |         }
1348 |         }
     |         - `meta_item` dropped here while still borrowed
   Compiling rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
Some errors have detailed explanations: E0597, E0716.
For more information about an error, try `rustc --explain E0597`.
error: could not compile `serde_derive` due to 6 previous errors
