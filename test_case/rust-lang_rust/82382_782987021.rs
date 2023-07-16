plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0061]: this function takes 0 arguments but 1 argument was supplied
   --> src/librustdoc/core.rs:141:66
    |
141 |         let def_index = if let Some(def_idx) = self.fake_def_ids.get_mut(&crate_num) {
    |                                                                  |
    |                                                                  expected 0 arguments

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> src/librustdoc/core.rs:141:32
    |
141 |         let def_index = if let Some(def_idx) = self.fake_def_ids.get_mut(&crate_num) {
    |                                ^^^^^^^^^^^^^   ------------------------------------- this expression has type `&mut HashMap<CrateNum, DefIndex, BuildHasherDefault<FxHasher>>`
    |                                |
    |                                expected struct `HashMap`, found enum `std::option::Option`
    |
    = note: expected struct `HashMap<CrateNum, DefIndex, BuildHasherDefault<FxHasher>>`
                 found enum `std::option::Option<_>`

error[E0599]: no method named `insert` found for struct `RefCell<HashMap<CrateNum, DefIndex, BuildHasherDefault<FxHasher>>>` in the current scope
    |
    |
157 |             self.fake_def_ids.insert(crate_num, num_def_idx);
    |                               ^^^^^^ method not found in `RefCell<HashMap<CrateNum, DefIndex, BuildHasherDefault<FxHasher>>>`
error[E0061]: this function takes 0 arguments but 1 argument was supplied
   --> src/librustdoc/core.rs:159:31
    |
    |
159 |             self.fake_def_ids.get_mut(&crate_num).unwrap()
    |                               |
    |                               expected 0 arguments


error[E0599]: no method named `unwrap` found for mutable reference `&mut HashMap<CrateNum, DefIndex, BuildHasherDefault<FxHasher>>` in the current scope
    |
    |
159 |             self.fake_def_ids.get_mut(&crate_num).unwrap()
    |                                                   ^^^^^^ method not found in `&mut HashMap<CrateNum, DefIndex, BuildHasherDefault<FxHasher>>`
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0061, E0308, E0599.
For more information about an error, try `rustc --explain E0061`.
