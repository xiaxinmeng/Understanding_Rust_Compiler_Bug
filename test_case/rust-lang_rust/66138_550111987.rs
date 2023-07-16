plain
2019-11-06T01:51:00.2803531Z normalized stderr:
2019-11-06T01:51:00.2803836Z error: This generic shadows the built-in type `u32`
2019-11-06T01:51:00.2804237Z   --> $DIR/builtin-type-shadow.rs:4:8
2019-11-06T01:51:00.2804322Z    |
2019-11-06T01:51:00.2804536Z LL | fn foo<u32>(a: u32) -> u32 {
2019-11-06T01:51:00.2804681Z    |
2019-11-06T01:51:00.2804974Z    = note: `-D clippy::builtin-type-shadow` implied by `-D warnings`
2019-11-06T01:51:00.2805049Z 
2019-11-06T01:51:00.2805120Z error[E0308]: mismatched types
2019-11-06T01:51:00.2805120Z error[E0308]: mismatched types
2019-11-06T01:51:00.2805363Z   --> $DIR/builtin-type-shadow.rs:5:5
2019-11-06T01:51:00.2805431Z    |
2019-11-06T01:51:00.2805653Z LL | fn foo<u32>(a: u32) -> u32 {
2019-11-06T01:51:00.2805905Z    |        ---             --- expected `u32` because of return type
2019-11-06T01:51:00.2806118Z    |        this type parameter
2019-11-06T01:51:00.2806202Z LL |     42
2019-11-06T01:51:00.2806286Z    |     ^^ expected type parameter `u32`, found integer
2019-11-06T01:51:00.2806356Z    |
2019-11-06T01:51:00.2806356Z    |
2019-11-06T01:51:00.2806431Z    = note: expected type `u32`
2019-11-06T01:51:00.2806497Z               found type `{integer}`
2019-11-06T01:51:00.2806590Z    = help: type parameters must be constrained to match other types
2019-11-06T01:51:00.2806906Z    = note: for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters
2019-11-06T01:51:00.2807345Z error: aborting due to 2 previous errors
2019-11-06T01:51:00.2807411Z 
2019-11-06T01:51:00.2807688Z For more information about this error, try `rustc --explain E0308`.
2019-11-06T01:51:00.2807759Z 
2019-11-06T01:51:00.2807759Z 
2019-11-06T01:51:00.2807793Z 
2019-11-06T01:51:00.2807849Z expected stderr:
2019-11-06T01:51:00.2808099Z error: This generic shadows the built-in type `u32`
2019-11-06T01:51:00.2808319Z   --> $DIR/builtin-type-shadow.rs:4:8
2019-11-06T01:51:00.2808403Z    |
2019-11-06T01:51:00.2808627Z LL | fn foo<u32>(a: u32) -> u32 {
2019-11-06T01:51:00.2808771Z    |
2019-11-06T01:51:00.2809012Z    = note: `-D clippy::builtin-type-shadow` implied by `-D warnings`
2019-11-06T01:51:00.2809084Z 
2019-11-06T01:51:00.2809423Z error[E0308]: mismatched types
2019-11-06T01:51:00.2809423Z error[E0308]: mismatched types
2019-11-06T01:51:00.2809703Z   --> $DIR/builtin-type-shadow.rs:5:5
2019-11-06T01:51:00.2809771Z    |
2019-11-06T01:51:00.2809997Z LL | fn foo<u32>(a: u32) -> u32 {
2019-11-06T01:51:00.2810272Z    |                        --- expected `u32` because of return type
2019-11-06T01:51:00.2810579Z    |     ^^ expected type parameter, found integer
2019-11-06T01:51:00.2810663Z    |
2019-11-06T01:51:00.2810719Z    = note: expected type `u32`
2019-11-06T01:51:00.2810719Z    = note: expected type `u32`
2019-11-06T01:51:00.2810804Z               found type `{integer}`
2019-11-06T01:51:00.2810900Z    = help: type parameters must be constrained to match other types
2019-11-06T01:51:00.2812545Z    = note: for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters
2019-11-06T01:51:00.2812720Z error: aborting due to 2 previous errors
2019-11-06T01:51:00.2812768Z 
2019-11-06T01:51:00.2813061Z For more information about this error, try `rustc --explain E0308`.
2019-11-06T01:51:00.2813120Z 
2019-11-06T01:51:00.2813120Z 
2019-11-06T01:51:00.2813175Z 
2019-11-06T01:51:00.2813237Z diff of stderr:
2019-11-06T01:51:00.2813279Z 
2019-11-06T01:51:00.2813552Z  error: This generic shadows the built-in type `u32`
2019-11-06T01:51:00.2813847Z    --> $DIR/builtin-type-shadow.rs:4:8
2019-11-06T01:51:00.2813924Z     |
2019-11-06T01:51:00.2814177Z  LL | fn foo<u32>(a: u32) -> u32 {
2019-11-06T01:51:00.2814337Z     |
2019-11-06T01:51:00.2814605Z     = note: `-D clippy::builtin-type-shadow` implied by `-D warnings`
2019-11-06T01:51:00.2814938Z  
2019-11-06T01:51:00.2814995Z  error[E0308]: mismatched types
2019-11-06T01:51:00.2814995Z  error[E0308]: mismatched types
2019-11-06T01:51:00.2815232Z    --> $DIR/builtin-type-shadow.rs:5:5
2019-11-06T01:51:00.2815300Z     |
2019-11-06T01:51:00.2815529Z  LL | fn foo<u32>(a: u32) -> u32 {
2019-11-06T01:51:00.2815802Z -   |                        --- expected `u32` because of return type
2019-11-06T01:51:00.2816065Z +   |        ---             --- expected `u32` because of return type
2019-11-06T01:51:00.2816222Z +   |        this type parameter
2019-11-06T01:51:00.2816305Z  LL |     42
2019-11-06T01:51:00.2816691Z -   |     ^^ expected type parameter, found integer
2019-11-06T01:51:00.2816802Z +   |     ^^ expected type parameter `u32`, found integer
2019-11-06T01:51:00.2816802Z +   |     ^^ expected type parameter `u32`, found integer
2019-11-06T01:51:00.2816869Z     |
2019-11-06T01:51:00.2816942Z     = note: expected type `u32`
2019-11-06T01:51:00.2817021Z                found type `{integer}`
2019-11-06T01:51:00.2817094Z     = help: type parameters must be constrained to match other types
2019-11-06T01:51:00.2817418Z     = note: for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters
2019-11-06T01:51:00.2817580Z  error: aborting due to 2 previous errors
2019-11-06T01:51:00.2817642Z  
2019-11-06T01:51:00.2817894Z  For more information about this error, try `rustc --explain E0308`.
2019-11-06T01:51:00.2817983Z  
---
2019-11-06T01:51:00.2823342Z 
2019-11-06T01:51:00.2823599Z ------------------------------------------
2019-11-06T01:51:00.2823692Z stderr:
2019-11-06T01:51:00.2823947Z ------------------------------------------
2019-11-06T01:51:00.2825395Z {"message":"This generic shadows the built-in type `u32`","code":{"code":"clippy::builtin_type_shadow","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/builtin-type-shadow.rs","byte_start":78,"byte_end":81,"line_start":4,"line_end":4,"column_start":8,"column_end":11,"is_primary":true,"text":[{"text":"fn foo<u32>(a: u32) -> u32 {","highlight_start":8,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::builtin-type-shadow` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: This generic shadows the built-in type `u32`\n  --> tests/ui/builtin-type-shadow.rs:4:8\n   |\nLL | fn foo<u32>(a: u32) -> u32 {\n   |        ^^^\n   |\n   = note: `-D clippy::builtin-type-shadow` implied by `-D warnings`\n\n"}
2019-11-06T01:51:00.2839374Z {"message":"mismatched types","code":{"code":"E0308","explanation":"\nThis error occurs when the compiler was unable to infer the concrete type of a\nvariable. It can occur for several cases, the most common of which is a\nmismatch in the expected type that the compiler inferred for a variable's\ninitializing expression, and the actual type explicitly assigned to the\nvariable.\n\nFor example:\n\n