
/Users/tyoverby/workspace/rust/asset_store/src/lib.rs:62:19: 62:23 error: cannot borrow `*self` as mutable more than once at a time
/Users/tyoverby/workspace/rust/asset_store/src/lib.rs:62             match self.fetch(path) {
                                                                           ^~~~
/Users/tyoverby/workspace/rust/asset_store/src/lib.rs:62:19: 62:23 note: previous borrow of `*self` occurs here; the mutable borrow prevents subsequent moves, borrows, or modification of `*self` until the borrow ends
/Users/tyoverby/workspace/rust/asset_store/src/lib.rs:62             match self.fetch(path) {
                                                                           ^~~~
/Users/tyoverby/workspace/rust/asset_store/src/lib.rs:68:6: 68:6 note: previous borrow ends here
/Users/tyoverby/workspace/rust/asset_store/src/lib.rs:60     fn fetch_block<'a>(&'a mut self, path: &str) -> Result<&'a Vec<u8>, E> {
...
/Users/tyoverby/workspace/rust/asset_store/src/lib.rs:68     }
                                                             ^
error: aborting due to previous error
