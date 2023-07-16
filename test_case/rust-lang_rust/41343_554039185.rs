
error: `impl` item signature doesn't match `trait` item signature
  --> src/main.rs:11:5
   |
5  |     fn bar(&self, vec: &Vec<u32>) -> Option<&u32>;
   |     ---------------------------------------------- expected fn(&Foo, &std::vec::Vec<u32>) -> std::option::Option<&u32>
...
11 |     fn bar<'a>(&self, vec: &'a Vec<u32>) -> Option<&'a u32> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ found fn(&Foo, &std::vec::Vec<u32>) -> std::option::Option<&u32>
   |
   = note: expected `fn(&Foo, &std::vec::Vec<u32>) -> std::option::Option<&u32>`
              found `fn(&Foo, &std::vec::Vec<u32>) -> std::option::Option<&u32>`
