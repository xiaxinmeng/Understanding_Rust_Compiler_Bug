rust
> #[lang = "sized"]
> pub trait Sized {}
> #[lang = "freeze"]
> pub trait Freeze {}
> #[lang = "copy"]
> pub trait Copy {}
> 
> impl<T: ?Sized> Copy for *mut T {}
> 