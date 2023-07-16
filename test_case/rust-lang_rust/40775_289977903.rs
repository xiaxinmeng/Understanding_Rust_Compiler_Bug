rust


error[E0573]: expected type, found variant `Some`
  --> $DIR/issue-35675.rs:27:13
   |
27 | fn qux() -> Some {
   |             ^^^^ not a type
   |
   = help: there is an enum variant `std::prelude::v1::Some`, did you mean to use `std::prelude::v1`?
   = help: there is an enum variant `std::prelude::v1::Option::Some`, did you mean to use `std::prelude::v1::Option`?
