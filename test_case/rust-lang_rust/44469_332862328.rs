
[00:32:17] error[E0599]: no method named `as_str` found for type `syntax_pos::symbol::InternedString` in the current scope
[00:32:17]     --> /checkout/src/librustdoc/clean/mod.rs:1942:69
[00:32:17]      |
[00:32:17] 1942 |                 let path = external_path(cx, &cx.tcx.item_name(did).as_str(),
[00:32:17]      |                                                                     ^^^^^^
[00:32:17] 
[00:32:19] error: aborting due to previous error
