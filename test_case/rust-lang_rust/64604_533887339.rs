plain
2019-09-22T13:40:50.1143669Z test [ui] ui/float_cmp_const.rs ... ok
2019-09-22T13:40:50.6580476Z test [ui] ui/fn_to_numeric_cast.rs ... ok
2019-09-22T13:40:50.7552302Z test [ui] ui/for_kv_map.rs ... ok
2019-09-22T13:40:51.4112598Z test [ui] ui/for_loop_over_option_result.rs ... ok
2019-09-22T13:40:52.2298157Z test [ui] ui/for_loop_unfixable.rs ... ok
2019-09-22T13:40:53.1455730Z test [ui] ui/for_loop_fixable.rs ... ok
2019-09-22T13:40:53.7396784Z test [ui] ui/format.rs ... ok
2019-09-22T13:40:54.2781639Z test [ui] ui/functions.rs ... ok
2019-09-22T13:40:54.4426206Z test [ui] ui/functions_maxlines.rs ... ok
2019-09-22T13:40:55.4300371Z test [ui] ui/get_last_with_len.rs ... ok
---
2019-09-22T13:42:08.2132073Z normalized stderr:
2019-09-22T13:42:08.2132489Z error: This generic shadows the built-in type `u32`
2019-09-22T13:42:08.2132773Z   --> $DIR/builtin-type-shadow.rs:4:8
2019-09-22T13:42:08.2133102Z    |
2019-09-22T13:42:08.2133463Z LL | fn foo<u32>(a: u32) -> u32 {
2019-09-22T13:42:08.2133862Z    |
2019-09-22T13:42:08.2134254Z    = note: `-D clippy::builtin-type-shadow` implied by `-D warnings`
2019-09-22T13:42:08.2134460Z 
2019-09-22T13:42:08.2134577Z error[E0308]: mismatched types
2019-09-22T13:42:08.2134577Z error[E0308]: mismatched types
2019-09-22T13:42:08.2135219Z   --> $DIR/builtin-type-shadow.rs:5:5
2019-09-22T13:42:08.2135328Z    |
2019-09-22T13:42:08.2135847Z LL | fn foo<u32>(a: u32) -> u32 {
2019-09-22T13:42:08.2136338Z    |                        --- expected `u32` because of return type
2019-09-22T13:42:08.2136696Z    |     ^^ expected type parameter, found integer
2019-09-22T13:42:08.2136786Z    |
2019-09-22T13:42:08.2136864Z    = note: expected type `u32`
2019-09-22T13:42:08.2136864Z    = note: expected type `u32`
2019-09-22T13:42:08.2136936Z               found type `{integer}`
2019-09-22T13:42:08.2137433Z    = help: type parameters must be constrained to match other types
2019-09-22T13:42:08.2137911Z    = note: for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters
2019-09-22T13:42:08.2138417Z error: aborting due to 2 previous errors
2019-09-22T13:42:08.2138498Z 
2019-09-22T13:42:08.2139005Z For more information about this error, try `rustc --explain E0308`.
2019-09-22T13:42:08.2139226Z 
2019-09-22T13:42:08.2139226Z 
2019-09-22T13:42:08.2139297Z 
2019-09-22T13:42:08.2139377Z expected stderr:
2019-09-22T13:42:08.2139710Z error: This generic shadows the built-in type `u32`
2019-09-22T13:42:08.2140155Z   --> $DIR/builtin-type-shadow.rs:4:8
2019-09-22T13:42:08.2140373Z    |
2019-09-22T13:42:08.2140702Z LL | fn foo<u32>(a: u32) -> u32 {
2019-09-22T13:42:08.2140994Z    |
2019-09-22T13:42:08.2141359Z    = note: `-D clippy::builtin-type-shadow` implied by `-D warnings`
2019-09-22T13:42:08.2141431Z 
2019-09-22T13:42:08.2141647Z error[E0308]: mismatched types
2019-09-22T13:42:08.2141647Z error[E0308]: mismatched types
2019-09-22T13:42:08.2141966Z   --> $DIR/builtin-type-shadow.rs:5:5
2019-09-22T13:42:08.2142188Z    |
2019-09-22T13:42:08.2142513Z LL | fn foo<u32>(a: u32) -> u32 {
2019-09-22T13:42:08.2142985Z    |                        --- expected `u32` because of return type
2019-09-22T13:42:08.2143333Z    |     ^^ expected type parameter, found integer
2019-09-22T13:42:08.2143421Z    |
2019-09-22T13:42:08.2143608Z    = note: expected type `u32`
2019-09-22T13:42:08.2143608Z    = note: expected type `u32`
2019-09-22T13:42:08.2143709Z               found type `{integer}`
2019-09-22T13:42:08.2143872Z error: aborting due to 2 previous errors
2019-09-22T13:42:08.2143918Z 
2019-09-22T13:42:08.2144281Z For more information about this error, try `rustc --explain E0308`.
2019-09-22T13:42:08.2144465Z 
2019-09-22T13:42:08.2144465Z 
2019-09-22T13:42:08.2144551Z 
2019-09-22T13:42:08.2144634Z diff of stderr:
2019-09-22T13:42:08.2144689Z 
2019-09-22T13:42:08.2145455Z  error: This generic shadows the built-in type `u32`
2019-09-22T13:42:08.2145957Z    --> $DIR/builtin-type-shadow.rs:4:8
2019-09-22T13:42:08.2146178Z     |
2019-09-22T13:42:08.2146505Z  LL | fn foo<u32>(a: u32) -> u32 {
2019-09-22T13:42:08.2146826Z     |
2019-09-22T13:42:08.2147195Z     = note: `-D clippy::builtin-type-shadow` implied by `-D warnings`
2019-09-22T13:42:08.2147279Z  
2019-09-22T13:42:08.2147359Z  error[E0308]: mismatched types
2019-09-22T13:42:08.2147359Z  error[E0308]: mismatched types
2019-09-22T13:42:08.2147679Z    --> $DIR/builtin-type-shadow.rs:5:5
2019-09-22T13:42:08.2147914Z     |
2019-09-22T13:42:08.2148244Z  LL | fn foo<u32>(a: u32) -> u32 {
2019-09-22T13:42:08.2148713Z     |                        --- expected `u32` because of return type
2019-09-22T13:42:08.2149078Z     |     ^^ expected type parameter, found integer
2019-09-22T13:42:08.2149170Z     |
2019-09-22T13:42:08.2149267Z     = note: expected type `u32`
2019-09-22T13:42:08.2149267Z     = note: expected type `u32`
2019-09-22T13:42:08.2149358Z                found type `{integer}`
2019-09-22T13:42:08.2149487Z +   = help: type parameters must be constrained to match other types
2019-09-22T13:42:08.2149911Z +   = note: for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters
2019-09-22T13:42:08.2150268Z  error: aborting due to 2 previous errors
2019-09-22T13:42:08.2150359Z  
2019-09-22T13:42:08.2150718Z  For more information about this error, try `rustc --explain E0308`.
2019-09-22T13:42:08.2150952Z  
---
2019-09-22T13:42:08.2156956Z 
2019-09-22T13:42:08.2157206Z ------------------------------------------
2019-09-22T13:42:08.2157295Z stderr:
2019-09-22T13:42:08.2157754Z ------------------------------------------
2019-09-22T13:42:08.2159267Z {"message":"This generic shadows the built-in type `u32`","code":{"code":"clippy::builtin_type_shadow","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/builtin-type-shadow.rs","byte_start":78,"byte_end":81,"line_start":4,"line_end":4,"column_start":8,"column_end":11,"is_primary":true,"text":[{"text":"fn foo<u32>(a: u32) -> u32 {","highlight_start":8,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::builtin-type-shadow` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: This generic shadows the built-in type `u32`\n  --> tests/ui/builtin-type-shadow.rs:4:8\n   |\nLL | fn foo<u32>(a: u32) -> u32 {\n   |        ^^^\n   |\n   = note: `-D clippy::builtin-type-shadow` implied by `-D warnings`\n\n"}
2019-09-22T13:42:08.2163352Z {"message":"mismatched types","code":{"code":"E0308","explanation":"\nThis error occurs when the compiler was unable to infer the concrete type of a\nvariable. It can occur for several cases, the most common of which is a\nmismatch in the expected type that the compiler inferred for a variable's\ninitializing expression, and the actual type explicitly assigned to the\nvariable.\n\nFor example:\n\n