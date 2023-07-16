`
2021-11-25T11:59:58.4590876Z test [ui] ui/large_const_arrays.rs ... ok
2021-11-25T11:59:58.4609082Z 
2021-11-25T11:59:58.4610210Z failures:
2021-11-25T11:59:58.4610794Z 
2021-11-25T11:59:58.4611536Z failures:
2021-11-25T11:59:58.4613343Z     [ui] ui/crashes/ice-6250.rs
2021-11-25T11:59:58.4614024Z 
2021-11-25T11:59:58.4615404Z test result: FAILED. 661 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out; finished in 7.44s
2021-11-25T11:59:58.4616521Z 
2021-11-25T11:59:58.4617373Z test compile_test ... FAILED
2021-11-25T11:59:58.4618031Z 
2021-11-25T11:59:58.4618796Z failures:
2021-11-25T11:59:58.4619435Z 
2021-11-25T11:59:58.4620611Z ---- compile_test stdout ----
2021-11-25T11:59:58.4621581Z diff of stderr:
2021-11-25T11:59:58.4622137Z 
2021-11-25T11:59:58.4623232Z  error[E0658]: destructuring assignments are unstable
2021-11-25T11:59:58.4624767Z    --> $DIR/ice-6250.rs:12:25
2021-11-25T11:59:58.4625601Z     |
2021-11-25T11:59:58.4626644Z  LL |         Some(reference) = cache.data.get(key) {
2021-11-25T11:59:58.4628046Z     |         --------------- ^
2021-11-25T11:59:58.4628859Z     |         |
2021-11-25T11:59:58.4629847Z     |         cannot assign to this expression
2021-11-25T11:59:58.4630817Z     |
2021-11-25T11:59:58.4633059Z     = note: see issue #71126 <https://github.com/rust-lang/rust/issues/71126> for more information
2021-11-25T11:59:58.4635341Z     = help: add `#![feature(destructuring_assignment)]` to the crate attributes to enable
2021-11-25T11:59:58.4636672Z  
2021-11-25T11:59:58.4637717Z  error[E0601]: `main` function not found in crate `ice_6250`
2021-11-25T11:59:58.4639183Z    --> $DIR/ice-6250.rs:4:1
2021-11-25T11:59:58.4640022Z     |
2021-11-25T11:59:58.4640983Z  LL | / pub struct Cache {
2021-11-25T11:59:58.4641549Z  LL | |     data: Vec<i32>,
2021-11-25T11:59:58.4642035Z  LL | | }
2021-11-25T11:59:58.4642450Z  LL | |
2021-11-25T11:59:58.4642850Z  ...  |
2021-11-25T11:59:58.4643250Z  LL | |     }
2021-11-25T11:59:58.4643667Z  LL | | }
2021-11-25T11:59:58.4644848Z     | |_^ consider adding a `main` function to `$DIR/ice-6250.rs`
2021-11-25T11:59:58.4645485Z  
2021-11-25T11:59:58.4645995Z  error[E0308]: mismatched types
2021-11-25T11:59:58.4646749Z    --> $DIR/ice-6250.rs:12:14
2021-11-25T11:59:58.4647225Z     |
2021-11-25T11:59:58.4647729Z +LL |     for reference in vec![1, 2, 3] {
2021-11-25T11:59:58.4648646Z +   |         --------- expected due to the type of this binding
2021-11-25T11:59:58.4649262Z +...
2021-11-25T11:59:58.4649862Z  LL |         Some(reference) = cache.data.get(key) {
2021-11-25T11:59:58.4650938Z     |              ^^^^^^^^^ expected integer, found `&i32`
2021-11-25T11:59:58.4651499Z     |
2021-11-25T11:59:58.4652215Z  help: consider dereferencing the borrow
2021-11-25T11:59:58.4652809Z     |
2021-11-25T11:59:58.4653403Z  LL |         Some(*reference) = cache.data.get(key) {
2021-11-25T11:59:58.4654013Z     |              +
2021-11-25T11:59:58.4654427Z  
2021-11-25T11:59:58.4654947Z  error[E0308]: mismatched types
2021-11-25T11:59:58.4655789Z    --> $DIR/ice-6250.rs:12:9
2021-11-25T11:59:58.4656266Z     |
2021-11-25T11:59:58.4656856Z  LL |         Some(reference) = cache.data.get(key) {
2021-11-25T11:59:58.4657602Z     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `bool`, found `()`
2021-11-25T11:59:58.4658154Z  
2021-11-25T11:59:58.4658703Z  error: aborting due to 4 previous errors
2021-11-25T11:59:58.4659264Z  
2021-11-25T11:59:58.4659887Z  Some errors have detailed explanations: E0308, E0601, E0658.
2021-11-25T11:59:58.4660952Z  For more information about an error, try `rustc --explain E0308`.
2021-11-25T11:59:58.4661619Z  
2021-11-25T11:59:58.4661887Z 
2021-11-25T11:59:58.4662505Z The actual stderr differed from the expected stderr.
2021-11-25T11:59:58.4664146Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/crashes/ice-6250.stage-id.stderr
2021-11-25T11:59:58.4665723Z To update references, rerun the tests and pass the `--bless` flag
2021-11-25T11:59:58.4670363Z To only update this specific test, also pass `--test-args crashes/ice-6250.rs`
2021-11-25T11:59:58.4671001Z 
