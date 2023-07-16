plain
   Compiling rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
error: hidden lifetime parameters in types are deprecated
    --> compiler/rustc_middle/src/ty/context.rs:1296:49
     |
1296 |             .contains_pointer_to::<InternedInSet<WithCachedTypeInfo<TyKind<'a>>>>(&InternedInSet(&*self.0.pointer().0))
     |                                    -------------^------------------------------- expected lifetime parameter
     |
     = note: `-D elided-lifetimes-in-paths` implied by `-D warnings`
help: indicate the anonymous lifetime
     |
1296 |             .contains_pointer_to::<InternedInSet<'_, WithCachedTypeInfo<TyKind<'a>>>>(&InternedInSet(&*self.0.pointer().0))

   Compiling rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: could not compile `rustc_middle` due to previous error
