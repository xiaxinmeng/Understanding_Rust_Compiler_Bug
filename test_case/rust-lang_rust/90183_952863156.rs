plain
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking tera v1.10.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0599]: no method named `def_id_full` found for reference `&types::Type` in the current scope
    --> src/librustdoc/html/render/mod.rs:1091:45
     |
1091 |                 if let Some(def_id) = type_.def_id_full(cx.cache()) {
     |                                             ^^^^^^^^^^^ method not found in `&types::Type`

error[E0599]: no method named `def_id_full` found for reference `&types::Type` in the current scope
    --> src/librustdoc/html/render/mod.rs:2144:67
     |
2144 |                 let id = if let Some(target_def_id) = real_target.def_id_full(c) {
     |                                                                   ^^^^^^^^^^^ method not found in `&types::Type`
error[E0061]: this function takes 1 argument but 0 arguments were supplied
    --> src/librustdoc/html/render/mod.rs:2168:42
     |
     |
2168 |         if let Some(target_did) = target.def_id() {
     |                                          |
     |                                          expected 1 argument
     |
note: associated function defined here
note: associated function defined here
    --> src/librustdoc/clean/types.rs:1540:14
     |
1540 |     crate fn def_id(&self, cache: &Cache) -> Option<DefId> {

error[E0061]: this function takes 1 argument but 0 arguments were supplied
    --> src/librustdoc/passes/collect_trait_impls.rs:100:45
     |
     |
100  |                 if let Some(for_did) = for_.def_id() {
     |                                             |
     |                                             expected 1 argument
     |
note: associated function defined here
note: associated function defined here
    --> src/librustdoc/clean/types.rs:1540:14
     |
1540 |     crate fn def_id(&self, cache: &Cache) -> Option<DefId> {

error[E0061]: this function takes 1 argument but 0 arguments were supplied
    --> src/librustdoc/passes/collect_trait_impls.rs:69:53
     |
     |
69   |             } else if let Some(target_did) = target.def_id() {
     |                                                     |
     |                                                     expected 1 argument
     |
note: associated function defined here
note: associated function defined here
    --> src/librustdoc/clean/types.rs:1540:14
     |
1540 |     crate fn def_id(&self, cache: &Cache) -> Option<DefId> {

Some errors have detailed explanations: E0061, E0599.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `rustdoc` due to 5 previous errors
