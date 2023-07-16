rust
> fn f(u: MyUnion) {
>     unsafe {
>         match u {
>             MyUnion { f1: 10 } => { println!("ten"); }
>             MyUnion { f2 } => { println!("{}", f2); }
>         }
>     }
> }
> 