
error: `RawVec::<T>::new` is not yet stable as a const fn
   --> library\alloc\src\vec\mod.rs:420:20
    |
420 |         Vec { buf: RawVec::new(), len: 0 }
    |                    ^^^^^^^^^^^^^
    |
    = help: Const-stable functions can only call other const-stable functions

error: aborting due to previous error
