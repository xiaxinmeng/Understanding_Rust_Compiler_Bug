rust
> // Assume there is also an
> // extern crate foo;
> mod foo;
> 
> fn main() {
>     // Does this call the module or the crate? (Or is it an error?)
>     foo::bar();
>     // How would I call the other?
> }
> 