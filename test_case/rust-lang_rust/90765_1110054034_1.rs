rs
//! dependent, which uses no `unsafe` code:

fn main() {
    match! ::a::fancy_block!() {
        ( $some_ident:ident {} ) => (
            $some_ident {
                ::core::hint::unreachable_unchecked()
            }
        );
    }
}
