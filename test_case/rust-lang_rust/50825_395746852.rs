text
error[E0277]: the trait bound `<expression::count::CountStar as expression::Expression>::SqlType: sql_types::ops::Add` is not satisfied
  --> /home/matthew/.cargo/registry/src/github.com-1ecc6299db9ec823/diesel-1.3.0/src/expression/count.rs:58:39
   |
58 | #[derive(Debug, Clone, Copy, QueryId, DieselNumericOps)]
   |                                       ^^^^^^^^^^^^^^^^ the trait `sql_types::ops::Add` is not implemented for `<expression::count::CountStar as expression::Expression>::SqlType`
   |
   = help: consider adding a `where <expression::count::CountStar as expression::Expression>::SqlType: sql_types::ops::Add` bound
   = help: see issue #48214
   = help: add #![feature(trivial_bounds)] to the crate attributes to enable

error[E0277]: the trait bound `<expression::count::CountStar as expression::Expression>::SqlType: sql_types::ops::Sub` is not satisfied
  --> /home/matthew/.cargo/registry/src/github.com-1ecc6299db9ec823/diesel-1.3.0/src/expression/count.rs:58:39
   |
58 | #[derive(Debug, Clone, Copy, QueryId, DieselNumericOps)]
   |                                       ^^^^^^^^^^^^^^^^ the trait `sql_types::ops::Sub` is not implemented for `<expression::count::CountStar as expression::Expression>::SqlType`
   |
   = help: consider adding a `where <expression::count::CountStar as expression::Expression>::SqlType: sql_types::ops::Sub` bound
   = help: see issue #48214
   = help: add #![feature(trivial_bounds)] to the crate attributes to enable

error[E0277]: the trait bound `<expression::count::CountStar as expression::Expression>::SqlType: sql_types::ops::Mul` is not satisfied
  --> /home/matthew/.cargo/registry/src/github.com-1ecc6299db9ec823/diesel-1.3.0/src/expression/count.rs:58:39
   |
58 | #[derive(Debug, Clone, Copy, QueryId, DieselNumericOps)]
   |                                       ^^^^^^^^^^^^^^^^ the trait `sql_types::ops::Mul` is not implemented for `<expression::count::CountStar as expression::Expression>::SqlType`
   |
   = help: consider adding a `where <expression::count::CountStar as expression::Expression>::SqlType: sql_types::ops::Mul` bound
   = help: see issue #48214
   = help: add #![feature(trivial_bounds)] to the crate attributes to enable

error[E0277]: the trait bound `<expression::count::CountStar as expression::Expression>::SqlType: sql_types::ops::Div` is not satisfied
  --> /home/matthew/.cargo/registry/src/github.com-1ecc6299db9ec823/diesel-1.3.0/src/expression/count.rs:58:39
   |
58 | #[derive(Debug, Clone, Copy, QueryId, DieselNumericOps)]
   |                                       ^^^^^^^^^^^^^^^^ the trait `sql_types::ops::Div` is not implemented for `<expression::count::CountStar as expression::Expression>::SqlType`
   |
   = help: consider adding a `where <expression::count::CountStar as expression::Expression>::SqlType: sql_types::ops::Div` bound
   = help: see issue #48214
   = help: add #![feature(trivial_bounds)] to the crate attributes to enable

error: aborting due to 4 previous errors
