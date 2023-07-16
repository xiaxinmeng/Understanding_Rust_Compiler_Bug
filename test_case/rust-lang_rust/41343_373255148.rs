
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter 'a in generic type due to conflicting requirements
  --> src/main.rs:11:5
   |
11 |     fn bar<'a>(&self, vec: &'a Vec<u32>) -> Option<&'a u32> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
note: first, the lifetime cannot outlive the anonymous lifetime #2 defined on the method body at 11:5...
  --> src/main.rs:11:5
   |
11 | /     fn bar<'a>(&self, vec: &'a Vec<u32>) -> Option<&'a u32> {
12 | |         vec.get(0)
13 | |     }
   | |_____^
note: ...but the lifetime must also be valid for the anonymous lifetime #1 defined on the method body at 11:5...
  --> src/main.rs:11:5
   |
11 | /     fn bar<'a>(&self, vec: &'a Vec<u32>) -> Option<&'a u32> {
12 | |         vec.get(0)
13 | |     }
   | |_____^
   = note: ...so that the method type is compatible with trait:
           expected fn(&Foo, &std::vec::Vec<u32>) -> std::option::Option<&u32>
              found fn(&Foo, &std::vec::Vec<u32>) -> std::option::Option<&u32>
