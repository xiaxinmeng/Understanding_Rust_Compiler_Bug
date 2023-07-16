rust
> eprintln!("[{}:{}] {} = {:#?}", file!(), line!(), stringify!(x), x); // before
> eprintln!("[example.rs:1] x = {:#?}", x); // after
> 