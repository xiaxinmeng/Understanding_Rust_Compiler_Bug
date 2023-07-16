
error[E0599]: no variant or associated item named `Three` found for type `Test` in the current scope
  --> src/main.rs:21:19
   |
1  | enum Test {
   | --------- variant or associated item `Three` not found here
...
21 |     let y = Test::Three; // here we have the expected error: `variant not found in `Test``
   |                   ^^^^^ variant or associated item not found in `Test`

error[E0599]: no variant or associated item named `Four` found for type `Test` in the current scope
  --> src/main.rs:25:19
   |
1  | enum Test {
   | --------- variant or associated item `Four` not found here
...
25 |     let y = Test::Four("yolo".to_owned()); // here we have the expected error: `variant not found in `Test``
   |                   ^^^^ variant or associated item not found in `Test`

error: no variant `Two` in enum `Test`
  --> src/main.rs:29:19
   |
1  | enum Test {
   | --------- variant `Two` not found here
...
29 |     let y = Test::Two{foo: "yolo".to_owned()}; // here, the error is not what it should be (`ambiguous associated type`)
   |             ------^^^
   |             |
   |             variant not found in `Test`

error: aborting due to 3 previous errors
