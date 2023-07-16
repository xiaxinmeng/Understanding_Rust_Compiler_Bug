
error: `impl` item signature doesn't match `trait` item signature
  --> src/main.rs:11:5
   |
5  |     fn bar(&self, vec: &Vec<u32>) -> Option<&u32>;
   |     ---------------------------------------------- expected `fn(&'1 Foo, &'2 Vec<u32>) -> Option<&'1 u32>`
...
11 |     fn bar<'a>(&self, vec: &'a Vec<u32>) -> Option<&'a u32> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ found `fn(&'1 Foo, &'2 Vec<u32>) -> Option<&'2 u32>`
   |
   = note: expected `fn(&'1 Foo, &'2 Vec<u32>) -> Option<&'1 u32>`
              found `fn(&'1 Foo, &'2 Vec<u32>) -> Option<&'2 u32>`
   = help: the lifetime requirements from the `impl` do not correspond to the requirements in the `trait`
   = help: verify the lifetime relationships in the `trait` and `impl` between the `self` argument, the other inputs and its output
