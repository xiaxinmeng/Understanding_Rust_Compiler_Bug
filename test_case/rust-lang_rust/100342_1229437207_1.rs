rust
The package `crossbeam v0.2.12` currently triggers the following future incompatibility lints:
> warning: the type `T` is generic, and might not permit being left uninitialized
>   --> /home/jess/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-0.2.12/src/sync/ms_queue.rs:66:45
>    |
> 66 |             payload: Payload::Data(unsafe { mem::uninitialized() }),
>    |                                             ^^^^^^^^^^^^^^^^^^^^
>    |                                             |
>    |                                             this code causes undefined behavior when executed
>    |                                             help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
>    |
>    = note: `#[allow(mem_uninitialized)]` on by default
>    = note: for more information, see FIXME: fill this in
>    = note: type might not be allowed to be left uninitialized
> 
> warning: the type `[std::cell::UnsafeCell<(T, std::sync::atomic::AtomicBool)>; 32]` is generic, and might not permit being left uninitialized
>     --> /home/jess/.cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-0.2.12/src/sync/seg_queue.rs:40:28
>      |
> 40   |             data: unsafe { mem::uninitialized() },
>      |                            ^^^^^^^^^^^^^^^^^^^^
>      |                            |
>      |                            this code causes undefined behavior when executed
>      |                            help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
>      |
>      = note: `#[allow(mem_uninitialized)]` on by default
>      = note: for more information, see FIXME: fill this in
> note: type might not be allowed to be left uninitialized (in this struct field)
>     --> /home/jess/src/rust/library/core/src/cell.rs:1869:5
>      |
> 1869 |     value: T,
>      |     ^^^^^^^^
> 
