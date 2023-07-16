plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:fbd127f5fb2c74988e07e7abbc13faa754861242)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
   Compiling rustc_baked_icu_data v0.0.0 (/checkout/compiler/rustc_baked_icu_data)
   Compiling rustc_macros v0.1.0 (/checkout/compiler/rustc_macros)
   Compiling rustc_index v0.0.0 (/checkout/compiler/rustc_index)
   Compiling rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
error[E0505]: cannot move out of `self.owner` because it is borrowed
    |
    |
395 |     pub fn try_map<F, U: ?Sized, E>(self, f: F) -> Result<OwningRef<O, U>, E>
    |                                     ---- binding `self` declared here
...
400 |         Ok(OwningRef { reference: f(&self)?, owner: self.owner })
    |                                     -----           ^^^^^^^^^^ move out of `self.owner` occurs here
    |                                     |
    |                                     borrow of `self` occurs here
    |     - borrow later used here


error[E0505]: cannot move out of `self.owner` because it is borrowed
    |
    |
608 |     pub fn try_map<F, U: ?Sized, E>(mut self, f: F) -> Result<OwningRef<O, U>, E>
    |                                     -------- binding `self` declared here
...
613 |         Ok(OwningRef { reference: f(&mut self)?, owner: self.owner })
    |                                     ---------           ^^^^^^^^^^ move out of `self.owner` occurs here
    |                                     |
    |                                     borrow of `self` occurs here
    |     - borrow later used here


error[E0505]: cannot move out of `self.owner` because it is borrowed
    |
    |
637 |     pub fn try_map_mut<F, U: ?Sized, E>(mut self, f: F) -> Result<OwningRefMut<O, U>, E>
    |                                         -------- binding `self` declared here
...
642 |         Ok(OwningRefMut { reference: f(&mut self)?, owner: self.owner })
    |                                        ---------           ^^^^^^^^^^ move out of `self.owner` occurs here
    |                                        |
    |                                        borrow of `self` occurs here
    |     - borrow later used here


error[E0597]: `string_cache` does not live long enough
    |
632 |         let mut string_cache = self.string_cache.write();
632 |         let mut string_cache = self.string_cache.write();
    |             ---------------- binding `string_cache` declared here
...
635 |         match string_cache.entry(s.into()) {
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ borrowed value does not live long enough
642 |     }
    |     -
    |     |
    |     |
    |     `string_cache` dropped here while still borrowed

Some errors have detailed explanations: E0505, E0597.
For more information about an error, try `rustc --explain E0505`.
error: could not compile `rustc_data_structures` due to 4 previous errors
