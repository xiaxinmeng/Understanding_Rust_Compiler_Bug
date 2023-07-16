
error[E0308]: if and else have incompatible types
  --> src/main.rs:6:5
   |
6  | /     if  timestamp % 2 == 0 {
7  | |         variant1()
8  | |     } else {
9  | |         variant2()
10 | |     }
   | |_____^ expected anonymized type, found a different anonymized type
   |
   = note: expected type `impl std::string::ToString` (anonymized type)
              found type `impl std::string::ToString` (anonymized type)

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
