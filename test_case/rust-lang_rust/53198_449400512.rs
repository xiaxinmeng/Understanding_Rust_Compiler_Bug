rust
> fn main() {
>     let mut x = &mut vec![];
>     let mut y = vec![];
>     x.push((
>         {
>             if false { x = &mut y };
>             22
>         },
>         x.len(),
>     ));
> }
> 