
error: The raw string needs 2 trailing `#`, but found 3
  --> /home/op/me/rust2/src/test/ui/parser/raw/raw-literal-too-many.rs:2:26
   |
LL |     let _foo = r##"bar"###;
   |                          ^ remove this `#`

error: expected one of `.`, `;`, `?`, or an operator, found `#`
  --> /home/op/me/rust2/src/test/ui/parser/raw/raw-literal-too-many.rs:2:26
   |
LL |     let _foo = r##"bar"###;
   |                          ^ expected one of `.`, `;`, `?`, or an operator here

error: aborting due to 2 previous errors
