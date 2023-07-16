rust
> async {
>     let x = String::new();
>     let p = &x as *const _;
>     let _y = x;
>     something.await;
>     dbg!(*p);
> }
> 