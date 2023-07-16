
> fn main(v: Vec<uint>) {
>     for x in v.iter() {
>         let x: uint = if true {
>             break;
>         } else {
>             42u
>         }
>     }
> }
> 