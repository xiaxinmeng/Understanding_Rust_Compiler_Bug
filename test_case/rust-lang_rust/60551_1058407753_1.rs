
> error: unconstrained generic constant
>   --> src\test.rs:18:5
>    |
> 18 |     const NAMES: [&'static str; Self::COUNT];
>    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
>    |
>    = help: try adding a `where` bound using this expression: `where [(); Self::COUNT]:`
> 
> error: unconstrained generic constant
>   --> src\test.rs:22:18
>    |
> 22 |     name_lookup: [resource::UniformLocation; T::COUNT],
>    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
>    |
>    = help: try adding a `where` bound using this expression: `where [(); T::COUNT]:`
> 