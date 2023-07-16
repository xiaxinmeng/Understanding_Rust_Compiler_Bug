
error: expected token: `,`
  --> <println macros>:4:29
   |
1  | / (  ) => ( print ! ( "\n" ) ) ; ( $ ( $ arg : tt ) * ) => (
2  | | {
3  | | # [ cfg ( not ( stage0 ) ) ] {
4  | | ( $ crate :: io :: _print ( format_args_nl ! ( $ ( $ arg ) * ) ) ) ; } # [
   | |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
5  | | cfg ( stage0 ) ] { print ! ( "{}\n" , format_args ! ( $ ( $ arg ) * ) ) } } )
   | |_____________________________________________________________________________- in this expansion of `println!`
   |
  ::: ../../src/test/ui/macros/missing-comma.rs:12:5
   |
12 |       println!("{}" a);
   |       ----------------- in this macro invocation
