rust
> // Doesn't compile: 
> // error[E0759]: `y` has an anonymous lifetime `'_` but it needs to satisfy a `'static` lifetime requirement
> fn bar(x: &'static str, y: &str, z: &str) -> impl Future<Output=()> {
>     async {
>         println!("{} {}", y, z);
>     }
> }
> 