plain
    Checking chalk-engine v0.76.0
error[E0308]: mismatched types
   --> compiler/rustc_ast/src/util/literal.rs:107:57
    |
107 |                 LitKind::Str(symbol, ast::StrStyle::Raw(n))
    |                                                         ^ expected `u16`, found `u8`
    |
help: you can convert a `u8` to a `u16`
    |
107 |                 LitKind::Str(symbol, ast::StrStyle::Raw(n.into()))

error[E0308]: mismatched types
   --> compiler/rustc_ast/src/util/literal.rs:165:75
    |
    |
165 |             LitKind::Str(symbol, ast::StrStyle::Raw(n)) => (token::StrRaw(n), symbol, None),
    |                                                                           ^ expected `u8`, found `u16`
    |
help: you can convert a `u16` to a `u8` and panic if the converted value doesn't fit
    |
165 |             LitKind::Str(symbol, ast::StrStyle::Raw(n)) => (token::StrRaw(n.try_into().unwrap()), symbol, None),

error[E0308]: mismatched types
    --> compiler/rustc_ast/src/ast.rs:1651:47
     |
     |
1651 |             StrStyle::Raw(n) => token::StrRaw(n),
     |                                               ^ expected `u8`, found `u16`
     |
help: you can convert a `u16` to a `u8` and panic if the converted value doesn't fit
     |
1651 |             StrStyle::Raw(n) => token::StrRaw(n.try_into().unwrap()),

    Checking gsgdt v0.1.2
    Checking rls-span v0.5.3
    Checking rls-data v0.19.1
