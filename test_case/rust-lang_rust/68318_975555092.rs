
> #![feature(negative_impls)]
> 
> pub struct Test{
> 
> }
> 
> impl !Drop for Test {}
> 
> fn foo(){
>     drop(Test{})
> }
> 