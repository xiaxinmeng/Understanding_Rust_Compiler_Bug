rust
// Postfix await-as-a-keyword. Looks as if we were accessing a Result<_, _> field,
// unless await is syntax-highlighted
first().await?.second().await?.third().await?
// Macro with method syntax. A few more symbols, but clearly a macro invocation that
// can affect control flow
first().await!()?.second().await!()?.third().await!()?
