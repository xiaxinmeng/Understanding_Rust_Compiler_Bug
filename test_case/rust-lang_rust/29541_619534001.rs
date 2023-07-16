
error[E0642]: patterns aren't allowed in functions without bodies
   --> /home/jubilee/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-serialize-0.3.16/src/serialize.rs:145:45
    |
145 | ...                   &f_name: &str,
    |                       ^^^^^^^ pattern not allowed in function without body

   Compiling byteorder v0.3.13
   Compiling lzw v0.8.0
   Compiling color_quant v1.0.0
error: aborting due to previous error

For more information about this error, try `rustc --explain E0642`.
error: could not compile `rustc-serialize`.

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
