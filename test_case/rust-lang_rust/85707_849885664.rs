
> after fixes were automatically applied the compiler reported errors within these files:
> 
>   * tests/nested.rs
> 
> <snip>
> 
> The following errors were reported:
> error[E0433]: failed to resolve: use of undeclared crate or module `required_option`
>   --> tests/nested.rs:31:10
>    |
> 31 |         <required_option::SuperOpt as ClapMe>::from_iter(&["", "--arg-arg", "7", "--other", "hello"]).unwrap());
>    |          ^^^^^^^^^^^^^^^ use of undeclared crate or module `required_option`
>    
> <snip>
> 