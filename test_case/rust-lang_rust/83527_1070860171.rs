rust
// Another example

#![feature(macro_metavar_expr)]

macro_rules! mac {
    ( $( [ $( ( $($i:ident)* ) )* ] )* ) => {
        [
            // ****** 6 `ident` repetitions *****
            //
            // 6 (a, b, c, d, e f)
            ${count(i)},
            
            // ****** 3 `[...]` repetitions *****
            //
            // 2 (a, b)
            // 4 (c d e f)
            // 0
            $( ${count(i)}, )*
            
            // ****** 5 `(...)` repetitions *****
            //
            // 2 (a, b)
            // 0
            // 1 (c)
            // 0
            // 3 (d e f)
            $( $( ${count(i)}, )* )*
        ]
    }
}

fn main() {
    let array = mac!([(a b) ()] [(c) () (d e f)] []);
    dbg!(array);
}
