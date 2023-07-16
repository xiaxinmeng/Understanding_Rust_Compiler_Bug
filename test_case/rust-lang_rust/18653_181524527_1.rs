 text
hello.rs:41:32: 41:42 error: `collection` does not live long enough
hello.rs:41         let __arg0 = Get::get(&collection);
                                           ^~~~~~~~~~
note: reference must be valid for the static lifetime...
hello.rs:38:44: 46:2 note: ...but borrowed value is only valid for the block suffix following statement 0 at 38:43
hello.rs:38     let mut collection: Wrap<_> = WrapNone;
hello.rs:39 
hello.rs:40     {
hello.rs:41         let __arg0 = Get::get(&collection);
hello.rs:42         let __args_cell = RefCell::new(__arg0);
hello.rs:43         constrain(__args_cell);
            ...
