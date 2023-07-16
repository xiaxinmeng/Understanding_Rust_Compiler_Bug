
> // Basic Example:
> trait Trait { type AssociatedType; }
> fn foo<T>(t: T) where T: Trait<AssociatedType = &'static str> {
>     println!("in foo");
> }
> impl Trait for i8 { type AssociatedType = &'static str; }
> foo(3_i8);
> 
> // For-Loop Example:
> let vs = vec![1, 2, 3, 4];
> for v in &vs {
>     match v {
>         &1 => {}
>         _ => {}
>     }
> }
> 