
> error[E0107]: missing generics for associated type `C::DType`
>   --> src/lib.rs:14:10
>    |
> 14 |     type DType<T>: D<T, CType = Self>;
>    |          ^^^^^ expected 1 type argument
>    |
> note: associated type defined here, with 1 type parameter: `T`
>   --> src/lib.rs:14:10
>    |
> 14 |     type DType<T>: D<T, CType = Self>;
>    |          ^^^^^ -
> help: use angle brackets to add missing type argument
>    |
> 14 |     type DType<T><T>: D<T, CType = Self>;
>    |               ^^^
> 