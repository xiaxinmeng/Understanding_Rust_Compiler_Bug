plain
    Checking chalk-solve v0.75.0
    Checking rustc_arena v0.0.0 (/checkout/compiler/rustc_arena)
    Checking rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
    Checking rustc_span v0.0.0 (/checkout/compiler/rustc_span)
error[E0599]: no method named `push` found for struct `HashMap` in the current scope
    |
    |
168 |             let expn_id = data.local_expn_data.push(None);
    |                                                ^^^^ method not found in `HashMap<LocalExpnId, Option<ExpnData>, BuildHasherDefault<FxHasher>>`

error[E0599]: no method named `push` found for struct `HashMap` in the current scope
    |
    |
169 |             let _eid = data.local_expn_hashes.push(ExpnHash(Fingerprint::ZERO));
    |                                               ^^^^ method not found in `HashMap<LocalExpnId, ExpnHash, BuildHasherDefault<FxHasher>>`

error[E0599]: no method named `push` found for struct `HashMap` in the current scope
    |
    |
179 |             let expn_id = data.local_expn_data.push(Some(expn_data));
    |                                                ^^^^ method not found in `HashMap<LocalExpnId, Option<ExpnData>, BuildHasherDefault<FxHasher>>`

error[E0599]: no method named `push` found for struct `HashMap` in the current scope
    |
    |
180 |             let _eid = data.local_expn_hashes.push(expn_hash);
    |                                               ^^^^ method not found in `HashMap<LocalExpnId, ExpnHash, BuildHasherDefault<FxHasher>>`
error[E0308]: mismatched types
   --> compiler/rustc_span/src/hygiene.rs:208:59
    |
    |
208 |             let old_expn_data = &mut data.local_expn_data[self];
    |                                                           |
    |                                                           |
    |                                                           expected `&LocalExpnId`, found struct `LocalExpnId`
    |                                                           help: consider borrowing here: `&self`
error[E0308]: mismatched types
   --> compiler/rustc_span/src/hygiene.rs:211:53
    |
    |
211 |             debug_assert_eq!(data.local_expn_hashes[self].0, Fingerprint::ZERO);
    |                                                     |
    |                                                     |
    |                                                     expected `&LocalExpnId`, found struct `LocalExpnId`
    |                                                     help: consider borrowing here: `&self`
error[E0308]: mismatched types
   --> compiler/rustc_span/src/hygiene.rs:212:36
    |
    |
212 |             data.local_expn_hashes[self] = expn_hash;
    |                                    |
    |                                    |
    |                                    expected `&LocalExpnId`, found struct `LocalExpnId`
    |                                    help: consider borrowing here: `&self`
error[E0308]: mismatched types
   --> compiler/rustc_span/src/hygiene.rs:349:30
    |
    |
349 |             local_expn_data: IndexVec::from_elem_n(Some(root_data), 1),
    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `HashMap`, found struct `IndexVec`
    |
    = note: expected struct `HashMap<LocalExpnId, Option<ExpnData>, BuildHasherDefault<FxHasher>>`
               found struct `IndexVec<_, Option<ExpnData>>`
error[E0308]: mismatched types
   --> compiler/rustc_span/src/hygiene.rs:350:32
    |
    |
350 |             local_expn_hashes: IndexVec::from_elem_n(ExpnHash(Fingerprint::ZERO), 1),
    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `HashMap`, found struct `IndexVec`
    |
    = note: expected struct `HashMap<LocalExpnId, ExpnHash, BuildHasherDefault<FxHasher>>`
               found struct `IndexVec<_, ExpnHash>`
error[E0308]: mismatched types
   --> compiler/rustc_span/src/hygiene.rs:374:32
    |
    |
374 |         self.local_expn_hashes[expn_id]
    |                                |
    |                                |
    |                                expected `&LocalExpnId`, found struct `LocalExpnId`
    |                                help: consider borrowing here: `&expn_id`
error[E0308]: mismatched types
   --> compiler/rustc_span/src/hygiene.rs:380:53
    |
    |
380 |             Some(expn_id) => self.local_expn_hashes[expn_id],
    |                                                     |
    |                                                     |
    |                                                     expected `&LocalExpnId`, found struct `LocalExpnId`
    |                                                     help: consider borrowing here: `&expn_id`
error[E0308]: mismatched types
   --> compiler/rustc_span/src/hygiene.rs:386:30
    |
    |
386 |         self.local_expn_data[expn_id].as_ref().expect("no expansion data for an expansion ID")
    |                              |
    |                              |
    |                              expected `&LocalExpnId`, found struct `LocalExpnId`
    |                              help: consider borrowing here: `&expn_id`
error[E0308]: mismatched types
   --> compiler/rustc_span/src/hygiene.rs:391:34
    |
    |
391 |             self.local_expn_data[expn_id].as_ref().expect("no expansion data for an expansion ID")
    |                                  |
    |                                  |
    |                                  expected `&LocalExpnId`, found struct `LocalExpnId`
    |                                  help: consider borrowing here: `&expn_id`

error[E0599]: no method named `iter_enumerated` found for struct `HashMap` in the current scope
    |
    |
628 |             data.local_expn_data.iter_enumerated().for_each(|(id, expn_data)| {
    |                                  ^^^^^^^^^^^^^^^ method not found in `HashMap<LocalExpnId, Option<ExpnData>, BuildHasherDefault<FxHasher>>`

error[E0599]: no method named `next_index` found for struct `HashMap` in the current scope
     |
     |
1234 |         let expn_id = hygiene_data.local_expn_data.next_index();
     |                                                    ^^^^^^^^^^ method not found in `HashMap<LocalExpnId, Option<ExpnData>, BuildHasherDefault<FxHasher>>`

error[E0599]: no method named `push` found for struct `HashMap` in the current scope
     |
     |
1235 |         hygiene_data.local_expn_data.push(Some(data));
     |                                      ^^^^ method not found in `HashMap<LocalExpnId, Option<ExpnData>, BuildHasherDefault<FxHasher>>`

error[E0599]: no method named `push` found for struct `HashMap` in the current scope
     |
     |
1236 |         let _eid = hygiene_data.local_expn_hashes.push(hash);
     |                                                   ^^^^ method not found in `HashMap<LocalExpnId, ExpnHash, BuildHasherDefault<FxHasher>>`

error[E0277]: `LocalExpnId` doesn't implement `Debug`
    |
316 |   #[derive(Debug)]
    |            ----- in this derive macro expansion
...
...
321 |       local_expn_data: FxHashMap<LocalExpnId, Option<ExpnData>>,
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `LocalExpnId` cannot be formatted using `{:?}`
   ::: /checkout/library/core/src/fmt/mod.rs:649:5
    |
    |
649 | /     pub macro Debug($item:item) {
651 | |     }
    | |_____- in this expansion of `#[derive(Debug)]`
    |
    = help: the trait `Debug` is not implemented for `LocalExpnId`
    = help: the trait `Debug` is not implemented for `LocalExpnId`
    = note: add `#[derive(Debug)]` to `LocalExpnId` or manually `impl Debug for LocalExpnId`

error[E0277]: `LocalExpnId` doesn't implement `Debug`
    |
316 |   #[derive(Debug)]
    |            ----- in this derive macro expansion
...
...
322 |       local_expn_hashes: FxHashMap<LocalExpnId, ExpnHash>,
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `LocalExpnId` cannot be formatted using `{:?}`
   ::: /checkout/library/core/src/fmt/mod.rs:649:5
    |
    |
649 | /     pub macro Debug($item:item) {
651 | |     }
    | |_____- in this expansion of `#[derive(Debug)]`
    |
    = help: the trait `Debug` is not implemented for `LocalExpnId`
    = help: the trait `Debug` is not implemented for `LocalExpnId`
    = note: add `#[derive(Debug)]` to `LocalExpnId` or manually `impl Debug for LocalExpnId`
Some errors have detailed explanations: E0277, E0308, E0599.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `rustc_span` due to 19 previous errors
warning: build failed, waiting for other jobs to finish...
