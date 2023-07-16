rust
macro_rules! define_queries {
    $($category:ident {
        $(queries:tt),*
    }),* => {
        define_queries_inner!(
            $($(category<$category> $queries)), //issue
        )
    }
}
