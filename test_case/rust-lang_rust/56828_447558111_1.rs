rust
> struct try { x: i32 }
> fn main() {
>     let x = 1;
>     match try { x } {
>         try { .. } => {}
>     }
> }
> 