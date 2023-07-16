rust
macro_rules! define_queries {
    $($category:ident {
        $(queries:tt);* // note `;` instead of comma
    }),* => {
        define_queries_inner!(
            $($(category<$category> $queries);*)* // note the two `*`s and `;`
        )
    }
}
