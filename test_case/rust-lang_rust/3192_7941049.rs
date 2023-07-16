
e := { expr* }
   | do path[(expr*)] [|ident*|] expr
   | path { (ident:expr)* }
