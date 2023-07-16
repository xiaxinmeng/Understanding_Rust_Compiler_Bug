
error[E0599]: no variant named `Three` found for type `Test` in the current scope
  --> src/main.rs:21:13
   |
1  | enum Test {
   | --------- variant `Three` not found here
...
21 |     let y = Test::Three; // here we have the expected error: `variant not found in `Test``
   |             ^^^^^^^^^^^ variant not found in `Test`

error[E0599]: no variant named `Four` found for type `Test` in the current scope
  --> src/main.rs:25:13
   |
1  | enum Test {
   | --------- variant `Four` not found here
...
25 |     let y = Test::Four("yolo".to_owned()); // here we have the expected error: `variant not found in `Test``
   |             ^^^^^^^^^^ variant not found in `Test`

error: no variant `Two` on enum `Test`
  --> src/main.rs:29:13
   |
29 |     let y = Test::Two{foo: "yolo".to_owned()}; // here, the error is not what it should be (`ambiguous associated type`)
   |             ^^^^^^^^^ unknown variant

error: aborting due to 3 previous errors
