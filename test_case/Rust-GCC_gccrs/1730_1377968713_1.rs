
Reuse TypeCheckPattern on LetStmt's

Update Rust type-checking to reuse TypeCheckPattern on HIR::LetStmts.
This will unify the paths and improve error handling.
