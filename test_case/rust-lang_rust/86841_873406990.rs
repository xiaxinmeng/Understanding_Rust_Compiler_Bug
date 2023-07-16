plain
     |
1701 |         use crate::rustc_ast_lowering::ResolverAstLowering;
     |                    ^^^^^^^^^^^^^^^^^^ could not find `rustc_ast_lowering` in the crate root

error[E0599]: no method named `opt_local_def_id` found for mutable reference `&mut Resolver<'_>` in the current scope
    --> src/librustdoc/clean/mod.rs:1712:43
     |
1712 |                     .enter_resolver(|r| r.opt_local_def_id(id))
     |                                           ^^^^^^^^^^^^^^^^ method not found in `&mut Resolver<'_>`
     = help: items from traits can only be used if the trait is in scope
     = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
     = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
             `use rustc_ast_lowering::ResolverAstLowering;`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0432, E0599.
For more information about an error, try `rustc --explain E0432`.
