 rust
macro_rules! parse {
    ( @RET($i:expr); @ $t_ty:ty , $e_ty:ty : $e:expr ) => { ... };
    ( @RET($i:expr); $e:expr ) => { ... };
    ( @ERR($i:expr); @ $t_ty:ty , $e_ty:ty : $e:expr ) => { ... };
    // ...
    ( $i:expr ; err $($tail:tt)+ ) => { ... };
    // ...
    ( $i:expr ) => { $i };
}
