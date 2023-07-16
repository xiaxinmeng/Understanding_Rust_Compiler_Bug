
error[E0381]: use of possibly uninitialized variable: `x`
 --> <source>:4:9
  |
4 |         x
  |         ^ use of possibly uninitialized `x`

error[E0080]: it is undefined behavior to use this value
 --> <source>:2:13
  |
2 |       A: [(); {
  |  _____________^
3 | |         let x: usize;
4 | |         x
5 | |     }],
  | |_____^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
  |
  = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior

error: aborting due to 2 previous errors
