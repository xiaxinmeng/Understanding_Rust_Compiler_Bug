
error[E0495]: cannot infer an appropriate lifetime for borrow expression due to conflicting requirements
  --> src/main.rs:16:19
   |
16 |         let a = f(&self[0]);
   |                   ^^^^^^^^
   |
note: first, the lifetime cannot outlive the anonymous lifetime #1 defined on the method body at 10:5...
  --> src/main.rs:10:5
   |
10 | /     fn my_sort_by_key<'a, B, F>(&mut self, f: F)
11 | |     where
12 | |         B: 'a + Ord,
13 | |         T: 'a,
...  |
21 | |         }
22 | |     }
   | |_____^
note: ...so that reference does not outlive borrowed content
  --> src/main.rs:16:19
   |
16 |         let a = f(&self[0]);
   |                   ^^^^^^^^
note: but, the lifetime must be valid for the lifetime 'a as defined on the method body at 10:5...
  --> src/main.rs:10:5
   |
10 | /     fn my_sort_by_key<'a, B, F>(&mut self, f: F)
11 | |     where
12 | |         B: 'a + Ord,
13 | |         T: 'a,
...  |
21 | |         }
22 | |     }
   | |_____^
note: ...so that reference does not outlive borrowed content
  --> src/main.rs:16:19
   |
16 |         let a = f(&self[0]);
   |                   ^^^^^^^^
