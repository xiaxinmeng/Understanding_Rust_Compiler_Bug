
error: unexpected closing delimiter: `}`
  --> main.rs:20:1
   |
16 | fn struct_generic(x: Vec<i32>) {
   |                                - this opening brace...
...
19 |     } // <-- incorrect closing brace
   |     - ...matches this closing brace, but they don't match indentation
20 | }
   | ^ unexpected closing delimiter
