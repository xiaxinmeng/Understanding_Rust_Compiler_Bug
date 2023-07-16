sh
~/tmp/issue-98932$ cargo +nightly-2022-04-15-x86_64-unknown-linux-gnu check
    Checking issue-98932 v0.1.0 (~/tmp/issue-98932)
error[E0599]: no method named `level_mask` found for reference `&HashesEntry<'a>` in the current scope
   --> src/main.rs:108:54
    |
108 |         let offset = Self::HASHES_OFFSET + 32 * self.level_mask().calc_hash_index(n as usize);
    |                                                      ^^^^^^^^^^ method not found in `&HashesEntry<'a>`

error[E0599]: no method named `level_mask` found for reference `&HashesEntry<'a>` in the current scope
   --> src/main.rs:113:53
    |
113 |         let offset = Self::DEPTHS_OFFSET + 2 * self.level_mask().calc_hash_index(n as usize);
    |                                                     ^^^^^^^^^^ method not found in `&HashesEntry<'a>`

error[E0599]: no method named `level_mask` found for reference `&HashesEntry<'a>` in the current scope
   --> src/main.rs:121:31
    |
121 |         let level_mask = self.level_mask();
    |                               ^^^^^^^^^^ method not found in `&HashesEntry<'a>`

error[E0599]: no method named `level_mask` found for reference `&HashesEntry<'a>` in the current scope
   --> src/main.rs:135:31
    |
135 |         let level_mask = self.level_mask();
    |                               ^^^^^^^^^^ method not found in `&HashesEntry<'a>`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `issue-98932` due to 4 previous errors
