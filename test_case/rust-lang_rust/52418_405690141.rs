
~/personal/rust (rust:master 0☲)$ more src/test/ui/issue-12187-1.stderr
error[E0282]: type annotations needed
  --> $DIR/issue-12187-1.rs:16:10
   |
LL |     let &v = new();
   |         -^
   |         ||
   |         |cannot infer type for `_`
   |         consider giving the pattern a type

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
~/personal/rust (rust:master 0☲)$ more src/test/ui/issue-23041.stderr
error[E0282]: type annotations needed
  --> $DIR/issue-23041.rs:16:22
   |
LL |     b.downcast_ref::<fn(_)->_>(); //~ ERROR E0282
   |                      ^^^^^^^^ cannot infer type for `_`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
~/personal/rust (rust:master 0☲)$ more src/test/ui/issue-12187-2.stderr
error[E0282]: type annotations needed
  --> $DIR/issue-12187-2.rs:16:10
   |
LL |     let &v = new();
   |         -^
   |         ||
   |         |cannot infer type for `_`
   |         consider giving the pattern a type

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
~/personal/rust (rust:master 0☲)$ more src/test/ui/span/issue-42234-unknown-receiver-type.stderr
error[E0282]: type annotations needed
  --> $DIR/issue-42234-unknown-receiver-type.rs:17:5
   |
LL |     let x: Option<_> = None;
   |         - consider giving `x` a type
LL |     x.unwrap().method_that_could_exist_on_some_type();
   |     ^^^^^^^^^^ cannot infer type for `T`
   |
   = note: type must be known at this point

error[E0282]: type annotations needed
  --> $DIR/issue-42234-unknown-receiver-type.rs:22:5
   |
LL | /     data.iter() //~ ERROR 22:5: 23:20: type annotations needed
LL | |         .sum::<_>()
   | |___________________^ cannot infer type for `_`
   |
   = note: type must be known at this point

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0282`.
~/personal/rust (rust:master 0☲)$ more src/test/ui/span/method-and-field-eager-resolution.stderr
error[E0282]: type annotations needed
  --> $DIR/method-and-field-eager-resolution.rs:15:5
   |
LL |     let mut x = Default::default();
   |         ----- consider giving `x` a type
LL |     x.0;
   |     ^ cannot infer type for `_`
   |
   = note: type must be known at this point

error[E0282]: type annotations needed
  --> $DIR/method-and-field-eager-resolution.rs:22:5
   |
LL |     let mut x = Default::default();
   |         ----- consider giving `x` a type
LL |     x[0];
   |     ^ cannot infer type for `_`
   |
   = note: type must be known at this point

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0282`.
~/personal/rust (rust:master 0☲)$ more src/test/ui/error-codes/E0282.stderr
error[E0282]: type annotations needed
  --> $DIR/E0282.rs:12:9
   |
LL |     let x = "hello".chars().rev().collect(); //~ ERROR E0282
   |         ^
   |         |
   |         cannot infer type for `_`
   |         consider giving `x` a type

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
~/personal/rust (rust:master 0☲)$ more src/test/ui/type-check/cannot_infer_local_or_array.stderr
error[E0282]: type annotations needed
  --> $DIR/cannot_infer_local_or_array.rs:12:13
   |
LL |     let x = []; //~ ERROR type annotations needed
   |         -   ^^ cannot infer type for `_`
   |         |
   |         consider giving `x` a type

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
~/personal/rust (rust:master 0☲)$ more src/test/ui/issue-23041.stderr
error[E0282]: type annotations needed
  --> $DIR/issue-23041.rs:16:22
   |
LL |     b.downcast_ref::<fn(_)->_>(); //~ ERROR E0282
   |                      ^^^^^^^^ cannot infer type for `_`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
~/personal/rust (rust:master 0☲)$ more src/test/ui/issue-7813.stderr
error[E0282]: type annotations needed
  --> $DIR/issue-7813.rs:12:13
   |
LL |     let v = &[]; //~ ERROR type annotations needed
   |         -   ^^^ cannot infer type for `_`
   |         |
   |         consider giving `v` a type

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
~/personal/rust (rust:master 0☲)$ more src/test/ui/issue-12187-1.stderr
error[E0282]: type annotations needed
  --> $DIR/issue-12187-1.rs:16:10
   |
LL |     let &v = new();
   |         -^
   |         ||
   |         |cannot infer type for `_`
   |         consider giving the pattern a type

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
