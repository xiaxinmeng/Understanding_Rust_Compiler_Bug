
rustc 1.19.0-nightly (6a5fc9eec 2017-05-02)
error[E0308]: mismatched types
 --> <anon>:3:29
  |
3 |     let bar: fn(&mut u32) = |_| {};
  |                             ^^^^^^ expected concrete lifetime, found bound lifetime parameter
  |
  = note: expected type `fn(&mut u32)`
             found type `[closure@<anon>:3:29: 3:35]`

error: aborting due to previous error
