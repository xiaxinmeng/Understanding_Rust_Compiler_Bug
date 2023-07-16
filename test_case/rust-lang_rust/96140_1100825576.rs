
error: Undefined Behavior: type validation failed at .b: encountered 0xff, but expected a boolean
  --> src/main.rs:11:18
   |
11 |         unsafe { ::std::mem::transmute(buffer) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .b: encountered 0xff, but expected a boolean
