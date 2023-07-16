rust
> use crate as playground;
> 
> mod inner {
>     //! [playground::inner::f] //~ ERROR no item named `playground` in scope
>     pub fn f() {}
> }
> 