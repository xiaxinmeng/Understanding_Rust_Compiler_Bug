rust
unsafe impl<'ast, Id> Send for AstType<'ast, Id> where Id: Send {}
unsafe impl<'ast, Id> Sync for AstType<'ast, Id> where Id: Sync {}
