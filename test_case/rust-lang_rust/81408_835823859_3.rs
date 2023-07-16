
     Running `target\release\example.exe`
thread 'main' panicked at 'assertion failed: state_and_queue & STATE_MASK == RUNNING', library\std\src\sync\once.rs:425:21
stack backtrace:
   0: std::panicking::begin_panic_handler
             at /rustc/881c1ac408d93bb7adaa3a51dabab9266e82eee8\/library\std\src\panicking.rs:493      
   1: core::panicking::panic_fmt
             at /rustc/881c1ac408d93bb7adaa3a51dabab9266e82eee8\/library\core\src\panicking.rs:92      
   2: core::panicking::panic
             at /rustc/881c1ac408d93bb7adaa3a51dabab9266e82eee8\/library\core\src\panicking.rs:50      
   3: std::sync::once::Once::call_inner
             at /rustc/881c1ac408d93bb7adaa3a51dabab9266e82eee8\/library\std\src\sync\once.rs:425      
   4: std::sync::once::Once::call_once
   5: example::lazy::SyncOnceCell<T>::get_or_init
   6: example::lazy::SyncLazy<T,F>::force
   7: core::ops::function::FnOnce::call_once{{vtable.shim}}
