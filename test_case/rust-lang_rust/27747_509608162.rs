rust
> impl<S: Borrow<BStr>> SliceConcatExt for [S] {
>     type Output = BString;
>     // ...
> }
> 