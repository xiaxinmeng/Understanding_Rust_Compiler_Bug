
   Compiling demo v0.1.0 (/Users/lopopolo/Desktop/demo)
error[E0412]: cannot find type `HashMap` in this scope
   --> src/main.rs:113:16
    |
113 | array_to_ruby!(HashMap<bool, Value>);
    |                ^^^^^^^ not found in this scope
help: possible candidates are found in other modules, you can import them into scope
    |
1   | use std::collections::HashMap;
    |
1   | use std::collections::hash_map::HashMap;
