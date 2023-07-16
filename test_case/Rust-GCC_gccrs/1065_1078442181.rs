rust
> arthur@platypus ~/G/gccrs (laxer-parser-on-stmt-fragment) [1]> wget https://www.cl.cam.ac.uk/~mgk25/ucs/examples/UTF-8-test.txt -O incorrect.utf # snip
> arthur@platypus ~/G/gccrs (laxer-parser-on-stmt-fragment)> cat test.rs
> fn main() {
>     println!("{}", include_str!("incorrect.utf"));
> }
> arthur@platypus ~/G/gccrs (laxer-parser-on-stmt-fragment)> rustc test.rs
> error: incorrect.utf wasn't a utf-8 file
>  --> test.rs:2:20
>   |
> 2 |     println!("{}", include_str!("incorrect.utf"));
>   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
>   |
>   = note: this error originates in the macro `include_str` (in Nightly builds, run with -Z macro-backtrace for more info)
> 
> error: aborting due to previous error
> 