
src/macros/insertable.rs:206:13: 207:32 error: the trait bound `i32: expression::Expression` is not satisfied [E0277]
src/macros/insertable.rs:206             AsExpression::<<$column as Expression>::SqlType>
                                         ^
src/macros/insertable.rs:206:13: 207:32 help: run `rustc --explain E0277` to see a detailed explanation
src/macros/insertable.rs:206:13: 207:32 note: required because of the requirements on the impl of `expression::Expression` for `&i32`
src/macros/insertable.rs:206:13: 207:32 note: required by `expression::AsExpression::as_expression`
src/macros/insertable.rs:206:13: 207:47 error: mismatched types [E0308]
