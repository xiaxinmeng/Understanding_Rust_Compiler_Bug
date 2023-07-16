
<std macros>:4:9: 6:15 error: unreachable expression, #[deny(unreachable_code)] on by default
<std macros>:4 $ msg , {
<std macros>:5 static _FILE_LINE : ( & 'static str , u32 ) = ( file ! (  ) , line ! (  ) ) ;
<std macros>:6 & _FILE_LINE } ) } ) ; ( $ fmt : expr , $ ( $ arg : tt ) + ) => (
src/libsyntax/ext/base.rs:681:13: 683:57 note: in this expansion of panic! (defined in <std macros>)
<std macros>:4:9: 6:15 error: unreachable expression, #[deny(unreachable_code)] on by default
<std macros>:4 $ msg , {
<std macros>:5 static _FILE_LINE : ( & 'static str , u32 ) = ( file ! (  ) , line ! (  ) ) ;
<std macros>:6 & _FILE_LINE } ) } ) ; ( $ fmt : expr , $ ( $ arg : tt ) + ) => (
src/libsyntax/ext/tt/macro_rules.rs:212:17: 212:77 note: in this expansion of panic! (defined in <std macros>)
<std macros>:4:9: 6:15 error: unreachable expression, #[deny(unreachable_code)] on by default
<std macros>:4 $ msg , {
<std macros>:5 static _FILE_LINE : ( & 'static str , u32 ) = ( file ! (  ) , line ! (  ) ) ;
<std macros>:6 & _FILE_LINE } ) } ) ; ( $ fmt : expr , $ ( $ arg : tt ) + ) => (
src/libsyntax/ext/tt/macro_rules.rs:217:5: 217:84 note: in this expansion of panic! (defined in <std macros>)
error: aborting due to 3 previous errors
