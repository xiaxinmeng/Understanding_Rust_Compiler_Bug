rust
> fn main() {
>     let mut v1: [Vec<i32>; 1] = [vec![1, 2, 3]];
>     
>     let go = || {
>         if v1[0][1] == 2 {  // v1 borrowed immutably
>             println!("ok");
>         }
>     };
>     
>       if v1.len() >= 1 {
>           return go();   // closure may be called, borrow must last at least this long
>       }
>       for i in v1.iter_mut() {  // v1 borrowed mutably....
>           println!("{i:?}");
>       }
> 
>       if v1.len() >= 1 {
>           return go();    // but again, closure may be called, so borrow must last at least this long,
>                                   // so the previous immutable borrow is illegal
>       }
>       for i in v1.iter_mut() {
>           println!("{i:?}");
>       }
>       [ ... ]
> }
> 