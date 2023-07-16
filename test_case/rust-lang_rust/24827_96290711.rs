
macro_rules! foo {
    () => ( ... );
    ( $($init:ident,)* $last:ident ) => ( ... foo! { $($init),* } );
}
