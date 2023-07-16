 rust
> #[path = "thread_files"]
> mod thread {
>     // Load the `local_data` module from `thread_files/tls.rs`
>     #[path = "tls.rs"]
>     mod local_data;
> }
> 