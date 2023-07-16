
error[E0308]: mismatched types
   --> src\lib.rs:106:32
    |
106 |         context.viewport(0, 0, canvas.width(), canvas.height());
    |                                ^^^^^^^^^^^^^^ expected i32, found u32

error[E0308]: mismatched types
   --> src\lib.rs:106:48
    |
106 |         context.viewport(0, 0, canvas.width(), canvas.height());
    |                                                ^^^^^^^^^^^^^^^ expected i32, found u32

error[E0308]: mismatched types
   --> src\lib.rs:126:37
    |
126 |         self.context.viewport(0, 0, width, height);
    |                                     ^^^^^ expected i32, found u32

error[E0308]: mismatched types
   --> src\lib.rs:126:44
    |
126 |         self.context.viewport(0, 0, width, height);
    |                                            ^^^^^^ expected i32, found u32
…
