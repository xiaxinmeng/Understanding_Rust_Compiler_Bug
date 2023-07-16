rust
> let mut got_true = true;
> for v in h
>     .iter_linear(1_000)
>     .skip_while(|v| v.quantile() < 0.01)
>     .take_while(move |v| {
>         if got_true {
>             // we've already yielded when condition was true
>             return false;
>         }
>         if v.quantile() > 0.99 {
>             // this must be the first time condition returns true
>             // we should yield i, and then no more
>             got_true = true;
>         }
>         // we should keep yielding
>         true
>     })
> 