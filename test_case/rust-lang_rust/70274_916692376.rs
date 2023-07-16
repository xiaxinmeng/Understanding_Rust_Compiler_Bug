
> error[E0499]: cannot borrow `x` as mutable more than once at a time
>   --> src/main.rs:12:13
>    |
> 10 |     let result = foo(&mut x);
>    |                      ------ first mutable borrow occurs here
> 11 |     drop(result);
> 12 |     let _ = x.bar();
>    |             ^
>    |             |
>    |             second mutable borrow occurs here
>    |             first borrow later used here
> 