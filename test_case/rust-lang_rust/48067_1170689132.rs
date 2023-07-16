rust
macro_rules! all_owned {
    ($first:ident$(::$rest:ident)* { $($key:ident:$value:expr,)* })  => {
        $first$(::$rest)* {
            $($key: ($value).to_owned(),)*
        }
    };
}
