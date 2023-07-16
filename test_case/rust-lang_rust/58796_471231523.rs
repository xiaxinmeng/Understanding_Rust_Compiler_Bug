
error: expected identifier, found `,`
  --> <::log::macros::error macros>:4:28
   |
1  | / ( target : $ target : expr , $ ( $ arg : tt ) * ) => (
2  | | log ! ( target : $ target , $ crate :: Level :: Error , $ ( $ arg ) * ) ; ) ;
3  | | ( $ ( $ arg : tt ) * ) => (
4  | | log ! ( $ crate :: Level :: Error , $ ( $ arg ) * ) ; )
   | |____________________________^__________________________- in this expansion of `error!`
   |                              |
   |                              expected identifier
   |                              help: remove this comma
   |
  ::: src/main.rs:29:5
   |
29 |       error!();
   |       --------- in this macro invocation
