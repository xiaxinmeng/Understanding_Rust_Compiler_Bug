rust
> struct ExternDeclCompat {
>     Compat,
>     Incompat,
>     Maybe(GuessedCFunctionType) // We unify and infer the actual C type that would make the two declaration compatible
> }
> 