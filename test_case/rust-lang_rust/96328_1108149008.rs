rust
> fn main() {
>     let mut builder = cc::Build::new();
>     builder.file("foo.c");
>     builder.compile("foo");
> 
>     println!("cargo:rustc-link-lib=z");
> }
> 