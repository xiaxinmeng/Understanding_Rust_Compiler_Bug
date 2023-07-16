
warning: unused import: `literal::Literal`
  --> src\flatanf\convert.rs:10:5
   |
10 | use literal::Literal;
   |     ^^^^^^^^^^^^^^^^
   |
   = note: #[warn(unused_imports)] on by default

error[E0072]: recursive type `ast::Decl` has infinite size
  --> src\ast\mod.rs:62:1
   |
62 | pub enum Decl {
   | ^^^^^^^^^^^^^ recursive type has infinite size
63 |     Def(Symbol, Expr),
   |                 ----- recursive without indirection
64 |     Defn(Symbol, Vec<Symbol>, Vec<Expr>, Expr),
   |                                          ----- recursive without indirection
   |
   = help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to make `ast::Decl` representable

error[E0072]: recursive type `ast::Expr` has infinite size
   --> src\ast\mod.rs:112:1
    |
112 | pub enum Expr {
    | ^^^^^^^^^^^^^ recursive type has infinite size
113 |     Call(Box<Expr>, Vec<Expr>),
114 |     Decl(Decl),
    |          ----- recursive without indirection
    |
    = help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to make `ast::Expr` representable

error: aborting due to 2 previous errors
