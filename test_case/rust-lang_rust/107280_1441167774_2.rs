
thread 'rustc' panicked at 'const variables should not be hashed: Var(_#0c)', compiler/rustc_middle/src/ty/consts/kind.rs:122:35

...
query stack during panic:
#0 [typeck] type-checking `execute::execute`
#1 [thir_body] building THIR for `execute::execute`
#2 [mir_built] building MIR for `execute::execute`
#3 [unsafety_check_result] unsafety-checking `execute::execute`
#4 [mir_const] preparing `execute::execute` for borrow checking
#5 [mir_promoted] processing MIR for `execute::execute`
#6 [mir_borrowck] borrow-checking `execute::execute`
#7 [type_of] computing type of `execute::execute::{opaque#0}`
#8 [check_mod_item_types] checking item types in module `client`
#9 [analysis] running analysis passes on this crate
end of query stack
