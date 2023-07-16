
record path_ = P {global: bool, idents: [ident], types: [@ty]}
   : ast_util::same_len_non_empty(*.idents, *.types);
