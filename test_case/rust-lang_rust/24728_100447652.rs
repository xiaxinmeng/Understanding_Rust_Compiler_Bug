 rust
> trait Trait { type AssociatedType; }
> fn foo<T>(t: T) where T: Trait<AssociatedType=u32> {
>     println!("in foo");
> } 
> impl Trait for i8 { type AssociatedType = &'static str; }
> foo(3_i8);
> 