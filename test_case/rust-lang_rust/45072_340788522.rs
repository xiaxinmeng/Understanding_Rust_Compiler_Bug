
error[E0308]: mismatched types
 --> /home/nmatsakis/tmp/bar.rs:3:14
  |
3 |     foo(|_y: &'a u32| {}); // this should be an error: `_y` isn't an `&'a u32`
  |              ^^^^^^^ lifetime mismatch
  |
  = note: expected type `&u32`
             found type `&'a u32`
note: the lifetime 'a as defined on the function body at 2:1...
 --> /home/nmatsakis/tmp/bar.rs:2:1
  |
2 | / fn bar<'a>(_x: &'a u32) {
3 | |     foo(|_y: &'a u32| {}); // this should be an error: `_y` isn't an `&'a u32`
4 | | }
  | |_^
note: ...does not necessarily outlive the anonymous lifetime #2 defined on the body at 3:9
 --> /home/nmatsakis/tmp/bar.rs:3:9
  |
3 |     foo(|_y: &'a u32| {}); // this should be an error: `_y` isn't an `&'a u32`
  |         ^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
 --> /home/nmatsakis/tmp/bar.rs:3:14
  |
3 |     foo(|_y: &'a u32| {}); // this should be an error: `_y` isn't an `&'a u32`
  |              ^^^^^^^ lifetime mismatch
  |
  = note: expected type `&u32`
             found type `&'a u32`
note: the anonymous lifetime #2 defined on the body at 3:9...
 --> /home/nmatsakis/tmp/bar.rs:3:9
  |
3 |     foo(|_y: &'a u32| {}); // this should be an error: `_y` isn't an `&'a u32`
  |         ^^^^^^^^^^^^^^^^
note: ...does not necessarily outlive the lifetime 'a as defined on the function body at 2:1
 --> /home/nmatsakis/tmp/bar.rs:2:1
  |
2 | / fn bar<'a>(_x: &'a u32) {
3 | |     foo(|_y: &'a u32| {}); // this should be an error: `_y` isn't an `&'a u32`
4 | | }
  | |_^
