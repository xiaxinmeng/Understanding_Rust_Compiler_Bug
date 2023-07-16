rust
macro_rules! define_queries {
    (<$tcx:tt> $($category:ident {
        $($(#[$attr:meta])* [$($modifiers:tt)*] fn $name:ident: $node:ident($K:ty) -> $V:ty,)*
    },)*) => {
        define_queries_inner! { <$tcx>
            $($( $(#[$attr])* category<$category> [$($modifiers)*] fn $name: $node($K) -> $V,)*)*
        }
    }
}

macro_rules! define_queries_inner {
    (<$tcx:tt>
     $($(#[$attr:meta])*
        category<$category:ident> [$($modifiers:tt)*] fn $name:ident: $node:ident($K:ty) -> $V:ty,)*) => {
           ....
    }
}
