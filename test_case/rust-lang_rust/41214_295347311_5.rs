rust
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter 'a in generic type due to conflicting requirements
  --> <anon>:11:5
   |
11 |     fn bar<'a>(&self, vec: &'a Vec<u32>) -> Option<&'a u32> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: first, the lifetime cannot outlive the anonymous lifetime #2 defined on the body at 11:60...
  --> <anon>:11:61
   |
11 |       fn bar<'a>(&self, vec: &'a Vec<u32>) -> Option<&'a u32> {
   |  _____________________________________________________________^
12 | |         vec.get(0)
13 | |     }
   | |_____^
note: ...so that method type is compatible with trait (expected fn(&Foo, &std::vec::Vec<u32>) -> std::option::Option<&u32>, found fn(&Foo, &std::vec::Vec<u32>) -> std::option::Option<&u32>)
  --> <anon>:11:5
   |
11 |     fn bar<'a>(&self, vec: &'a Vec<u32>) -> Option<&'a u32> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: but, the lifetime must be valid for the anonymous lifetime #1 defined on the body at 11:60...
  --> <anon>:11:61
   |
11 |       fn bar<'a>(&self, vec: &'a Vec<u32>) -> Option<&'a u32> {
   |  _____________________________________________________________^
12 | |         vec.get(0)
13 | |     }
   | |_____^
note: ...so that method type is compatible with trait (expected fn(&Foo, &std::vec::Vec<u32>) -> std::option::Option<&u32>, found fn(&Foo, &std::vec::Vec<u32>) -> std::option::Option<&u32>)
  --> <anon>:11:5
   |
11 |     fn bar<'a>(&self, vec: &'a Vec<u32>) -> Option<&'a u32> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error
