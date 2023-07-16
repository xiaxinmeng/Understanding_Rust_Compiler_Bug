
error[E0502]: cannot borrow `*self` as immutable because it is also borrowed as mutable
   --> src/librustdoc/core.rs:150:25
    |
144 |         let def_index = match self.fake_def_ids.entry(crate_num) {
    |                               ----------------- mutable borrow occurs here
...
150 |                         self.enter_resolver(|r| r.cstore().num_def_ids(crate_num))
    |                         ^^^^ immutable borrow occurs here
...
159 |                 e.insert(num_def_idx)
    |                 - mutable borrow later used here
