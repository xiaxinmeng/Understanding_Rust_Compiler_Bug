 rust
macro_rules! m {
    ( $( $i:ident )* ) => {
        m!( $($i)* $($i)* )
    };
}

fn main() {
    let i = m!(m);
}
