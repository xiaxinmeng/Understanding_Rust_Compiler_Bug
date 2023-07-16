rust
The package `hyper v0.13.10` currently triggers the following future incompatibility lints:
> warning: the type `[httparse::Header; 100]` does not permit being left uninitialized
>    --> /home/jess/.cargo/registry/src/github.com-1ecc6299db9ec823/hyper-0.13.10/src/proto/h1/role.rs:120:77
>     |
> 120 |             let mut headers: [httparse::Header<'_>; MAX_HEADERS] = unsafe { mem::uninitialized() };
>     |                                                                             ^^^^^^^^^^^^^^^^^^^^
>     |                                                                             |
>     |                                                                             this code causes undefined behavior when executed
>     |                                                                             help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
>     |
>     = note: `#[allow(mem_uninitialized)]` on by default
>     = note: for more information, see FIXME: fill this in
> note: references must be non-null (in this struct field)
>    --> /home/jess/.cargo/registry/src/github.com-1ecc6299db9ec823/httparse-1.7.1/src/lib.rs:660:5
>     |
> 660 |     pub name: &'a str,
>     |     ^^^^^^^^^^^^^^^^^
> 
