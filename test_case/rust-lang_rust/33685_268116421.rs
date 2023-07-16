rust 
>  // so e.g. this:
>  where F: for<'a> Fn() -> &'a u32
>  
>  // becomes this:
>  where F: Fn() -> &'static u32