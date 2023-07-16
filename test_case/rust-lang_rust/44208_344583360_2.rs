
error[E0599]: no method named `cloned` found for type `std::option::Option<()>` in the current scope
  --> src/main.rs:19:43
   |
19 |     let x: Option<char> = TABLE.find('a').cloned();
   |                                           ^^^^^^
   |
   = note: the method `cloned` exists but the following trait bounds were not satisfied:
           `&mut std::option::Option<()> : std::iter::Iterator`
