text
error: lifetime bounds cannot be used in this context
 --> src/main.rs:9:41
  |
9 | fn test<L>(func: L) where L : for<'b,'a:'b> Fn(&'b <&'a String as Test<'a>>::Inner) { }
  |                                         ^^

error: aborting due to previous error
