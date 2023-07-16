
fn yam<T>(foo: Wrapper<T>)
//~^ ERROR the size for values of type `T` cannot be known at compilation time
where
    T
    :
    ?
    Sized {
    //
}
