plain
    Checking askama_shared v0.12.0
   Compiling askama_derive v0.11.0
    Checking askama v0.11.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0599]: no method named `contains_key` found for struct `Rc<RefCell<HashMap<rustc_span::def_id::DefId, types::TraitWithExtraInfo, BuildHasherDefault<FxHasher>>>>` in the current scope
    |
    |
182 |             if self.traits.contains_key(&trait_did) {
    |                            ^^^^^^^^^^^^ method not found in `Rc<RefCell<HashMap<rustc_span::def_id::DefId, types::TraitWithExtraInfo, BuildHasherDefault<FxHasher>>>>`

error[E0599]: no method named `entry` found for struct `Rc<RefCell<HashMap<rustc_span::def_id::DefId, types::TraitWithExtraInfo, BuildHasherDefault<FxHasher>>>>` in the current scope
    |
    |
225 |             self.cache.traits.entry(item.def_id.expect_def_id()).or_insert_with(|| {
    |                               ^^^^^ method not found in `Rc<RefCell<HashMap<rustc_span::def_id::DefId, types::TraitWithExtraInfo, BuildHasherDefault<FxHasher>>>>`

error[E0599]: no method named `contains_key` found for struct `Rc<RefCell<HashMap<rustc_span::def_id::DefId, types::TraitWithExtraInfo, BuildHasherDefault<FxHasher>>>>` in the current scope
    |
    |
470 |             if impl_item.trait_did().map_or(true, |d| self.cache.traits.contains_key(&d)) {
    |                                                                         ^^^^^^^^^^^^ method not found in `Rc<RefCell<HashMap<rustc_span::def_id::DefId, types::TraitWithExtraInfo, BuildHasherDefault<FxHasher>>>>`

error[E0599]: no method named `get` found for struct `Rc<RefCell<HashMap<rustc_span::def_id::DefId, types::TraitWithExtraInfo, BuildHasherDefault<FxHasher>>>>` in the current scope
    --> src/librustdoc/html/render/mod.rs:1273:42
     |
1273 |                     if cx.cache().traits.get(&trait_did).map_or(false, |t| t.is_notable) {
     |                                          ^^^ method not found in `Rc<RefCell<HashMap<rustc_span::def_id::DefId, types::TraitWithExtraInfo, BuildHasherDefault<FxHasher>>>>`

error[E0608]: cannot index into a value of type `&Rc<RefCell<HashMap<rustc_span::def_id::DefId, types::TraitWithExtraInfo, BuildHasherDefault<FxHasher>>>>`
    --> src/librustdoc/html/render/mod.rs:1349:43
     |
1349 |     let trait_ = i.trait_did().map(|did| &traits[&did]);


error[E0599]: no method named `iter` found for struct `Rc<RefCell<HashMap<rustc_span::def_id::DefId, types::TraitWithExtraInfo, BuildHasherDefault<FxHasher>>>>` in the current scope
   --> src/librustdoc/json/mod.rs:102:14
102 |             .iter()
102 |             .iter()
    |              ^^^^ method not found in `Rc<RefCell<HashMap<rustc_span::def_id::DefId, types::TraitWithExtraInfo, BuildHasherDefault<FxHasher>>>>`
Some errors have detailed explanations: E0599, E0608.
For more information about an error, try `rustc --explain E0599`.
error: could not compile `rustdoc` due to 6 previous errors
Build completed unsuccessfully in 0:02:31
