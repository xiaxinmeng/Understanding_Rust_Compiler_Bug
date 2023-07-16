

rustc 1.18.0-nightly (128aa262e 2017-04-28)
warning: enum variant is more than three times larger (28 bytes) than the next largest
  --> <anon>:96:9
   |
53 | |                  $variant($ty),
   | |______________________________^
...
89 | |/ error_enum! {
90 | ||     /// Foo.
91 | ||     pub enum Foo {
92 | ||         /// A.
...  ||
96 | /|         B(B),
97 |  |     }
98 |  | }
   |  |_- in this macro invocation
   |
note: lint level defined here
  --> <anon>:1:9
   |
1  | #![warn(variant_size_differences)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^

