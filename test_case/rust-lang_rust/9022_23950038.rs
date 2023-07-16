
fn_decl: "fn" ident [ generic_decl ] ? "(" args * ")" "->" type block ;
generic_decl: "<" type [ "," type ] * ">" ;
args: arg [ "," arg ] * ;
arg: pat [ ":" type ] ;
