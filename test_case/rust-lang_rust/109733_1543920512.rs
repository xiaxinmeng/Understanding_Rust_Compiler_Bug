rust
> mod s {
>     pub use b::A;
> 
>     pub trait A {}
> }
> 
> use s::A;
> 
> /// [`A`]
> pub fn f() -> () {}
> 