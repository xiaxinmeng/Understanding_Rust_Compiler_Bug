\n\nIf you're using a stable or a beta version of rust18]   --> $DIR/const-int-overflowing.rs:12:27
[00:47:18]    |
[00:47:18] LL |     let x: &'static i32 = &(5_i32.overflowing_add(3)); //~ ERROR does not live long enough
[00:47:18]    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected i32, found tuple
[00:47:18]    = note: expected type `&'static i32`
[00:47:18]    = note: expected type `&'static i32`
[00:47:18]               found type `&(i32, bool)`
[00:47:18] error[E0308]: mismatched types
[00:47:18]   --> $DIR/const-int-overflowing.rs:13:27
[00:47:18]    |
[00:47:18]    |
[00:47:18] LL |     let y: &'static i32 = &(5_i32.overflowing_sub(3)); //~ ERROR does not live long enough
[00:47:18]    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected i32, found tuple
[00:47:18]    = note: expected type `&'static i32`
[00:47:18]    = note: expected type `&'static i32`
[00:47:18]               found type `&(i32, bool)`
[00:47:18] error[E0308]: mismatched types
[00:47:18]   --> $DIR/const-int-overflowing.rs:14:27
[00:47:18]    |
[00:47:18]    |
[00:47:18] LL |     let z: &'static i32 = &(5_i32.overflowing_mul(3)); //~ ERROR does not live long enough
[00:47:18]    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected i32, found tuple
[00:47:18]    = note: expected type `&'static i32`
[00:47:18]    = note: expected type `&'static i32`
[00:47:18]               found type `&(i32, bool)`
[00:47:18] error: aborting due to 3 previous errors
[00:47:18] 
[00:47:18] For more information about this error, try `rustc --explain E0308`.
[00:47:18] 
[00:47:18] 
[00:47:18] 
[00:47:18] 
[00:47:18] The actual stderr differed from the expected stderr.
[00:47:18] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/~~~~~~~~\n//      |             |\n//      |    initializing expression;\n//      |    compiler infers type `&str`\n//      |\n//    type `i32` assigned to variable `x`\n