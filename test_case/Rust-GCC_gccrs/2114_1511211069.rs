c++
> // Parses crate and dumps AST to stderr, recursively.
> template <typename ManagedTokenSource>
> void
> Parser<ManagedTokenSource>::debug_dump_ast_output (AST::Crate &crate,
> 						   std::ostream &out)
> {
>   out << crate.as_string ();
> }
> 