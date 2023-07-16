rust
macro_rules! fnptr_impsl_argsptr {
   ($($arg: ident),+) => {
     fnptr_impls_args! { $($arg),+ };
     fnptr_impls_args! { $(&$arg),+ };
     fnptr_impls_args! { $(&mut $arg),+ };
   };
   () => {
     fnptr_impls_args! {};
   };
}

fnptr_impls_argsptr! { }
fnptr_impls_argsptr! { A }
fnptr_impls_argsptr! { A, B }
fnptr_impls_argsptr! { A, B, C }
fnptr_impls_argsptr! { A, B, C, D }
fnptr_impls_argsptr! { A, B, C, D, E }
fnptr_impls_argsptr! { A, B, C, D, E, F }
fnptr_impls_argsptr! { A, B, C, D, E, F, G }
fnptr_impls_argsptr! { A, B, C, D, E, F, G, H }
fnptr_impls_argsptr! { A, B, C, D, E, F, G, H, I }
fnptr_impls_argsptr! { A, B, C, D, E, F, G, H, I, J }
fnptr_impls_argsptr! { A, B, C, D, E, F, G, H, I, J, K }
fnptr_impls_argsptr! { A, B, C, D, E, F, G, H, I, J, K, L }
