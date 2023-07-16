
error: parenthesized lifetime bounds are not supported
 --> /home/igor/dev/sandbox/sample.rs:3:21
  |
3 | fn f<'a, T: Trait + ('a)>() {}
  |                     ^^^^ help: remove the parentheses: `'a`

error: aborting due to previous error
