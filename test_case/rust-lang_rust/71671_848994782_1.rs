
  Compiling playground v0.0.1 (/playground)
error: implementation of `Robot` is not general enough
  --> src/lib.rs:22:20
   |
22 |     let _my_task = this_is_send(async move {
   |                    ^^^^^^^^^^^^ implementation of `Robot` is not general enough
   |
   = note: `Box<(dyn Robot<Id = u32> + Send + '0)>` must implement `Robot`, for any lifetime `'0`...
   = note: ...but `Robot` is actually implemented for the type `Box<(dyn Robot<Id = u32> + Send + 'static)>`

error: aborting due to previous error

error: could not compile `playground`

To learn more, run the command again with --verbose.
