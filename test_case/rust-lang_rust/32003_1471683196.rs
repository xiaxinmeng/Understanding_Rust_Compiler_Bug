
error[[E0308]](https://doc.rust-lang.org/stable/error_codes/E0308.html): mismatched types
  --> src/main.rs:12:31
   |
12 |     let fails: fn(MyStruct) = MyStruct::func; // ERROR: mismatched types
   |                               ^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected fn pointer `for<'a> fn(MyStruct<'a>)`
                 found fn item `fn(MyStruct<'_>) {MyStruct::<'_>::func}`
