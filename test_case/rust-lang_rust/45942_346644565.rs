
[00:45:25]  error[E0423]: expected function, found enum `Option`
[00:45:25]    --> $DIR/issue-43871-enum-instead-of-variant.rs:14:13
[00:45:25]     |
[00:45:25]  14 |     let x = Option(1);
[00:45:25]     |             ^^^^^^
[00:45:25]     |
[00:45:25]     = note: did you mean to use one of the following variants?
[00:45:25] -           - `std::prelude::v1::Option::None`
[00:45:25] -           - `std::prelude::v1::Option::Some`
[00:45:25] +           - `std::option::Option::None`
[00:45:25] +           - `std::option::Option::Some`
[00:45:25]  
[00:45:25]  error[E0532]: expected tuple struct/variant, found enum `Option`
[00:45:25]    --> $DIR/issue-43871-enum-instead-of-variant.rs:16:12
[00:45:25]     |
[00:45:25]  16 |     if let Option(_) = x {
[00:45:25]     |            ^^^^^^
[00:45:25]     |
[00:45:25]     = note: did you mean to use one of the following variants?
[00:45:25] -           - `std::prelude::v1::Option::None`
[00:45:25] -           - `std::prelude::v1::Option::Some`
[00:45:25] +           - `std::option::Option::None`
[00:45:25] +           - `std::option::Option::Some`
[00:45:25]  
[00:45:25]  error[E0532]: expected tuple struct/variant, found enum `Example`
[00:45:25]    --> $DIR/issue-43871-enum-instead-of-variant.rs:22:12
[00:45:25]     |
[00:45:25]  22 |     if let Example(_) = y {
[00:45:25]     |            ^^^^^^^
[00:45:25]     |
[00:45:25]     = note: did you mean to use one of the following variants?
[00:45:25]             - `Example::Ex`
[00:45:25]             - `Example::NotEx`
[00:45:25]  
[00:45:25]  error: aborting due to 3 previous errors
[00:45:25]  
