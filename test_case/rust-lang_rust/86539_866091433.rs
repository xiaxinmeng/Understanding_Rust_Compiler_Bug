rust
// In a crate `::a`
#[macro_export]
macro_rules! mk_m {( $std:ident ) => (
    #[macro_export]
    macro_rules! m {( $panic:ident ) => ( $std::$panic!() )}
)}
// In a crate `::b`
::a::mk_m!(std);
// In a crate `::c`
m!(panic); // <- `std` is spanned at `::b`, `::` and `!` at `::a` (I think), and `panic` here, at `::c`.
