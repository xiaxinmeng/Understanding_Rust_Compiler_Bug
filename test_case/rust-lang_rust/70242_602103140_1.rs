
2020-03-21T20:51:34.3370976Z - 
2020-03-21T20:51:34.3371259Z 35 This error occurs when the compiler was unable to infer the concrete type of a
2020-03-21T20:51:34.3371688Z 36 variable. It can happen in several cases, the most common being a mismatch
2020-03-21T20:51:34.3372091Z 37 between the type that the compiler inferred for a variable based on its
2020-03-21T20:51:34.3372331Z 
2020-03-21T20:51:34.3372614Z 38 initializing expression, on the one hand, and the type the author explicitly
2020-03-21T20:51:34.3373157Z - assigned to the variable, on the other hand."
2020-03-21T20:51:34.3373451Z + assigned to the variable, on the other hand.
2020-03-21T20:51:34.3378380Z 40 "},"level":"error","spans":[{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":681,"byte_end":682,"line_start":19,"line_end":19,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"    let s : String = 1","highlight_start":22,"highlight_end":23}],"label":"expected struct `std::string::String`, found integer","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":672,"byte_end":678,"line_start":19,"line_end":19,"column_start":13,"column_end":19,"is_primary":false,"text":[{"text":"    let s : String = 1","highlight_start":13,"highlight_end":19}],"label":"expected due to this","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try using a conversion method","code":null,"level":"help","spans":[{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":681,"byte_end":682,"line_start":19,"line_end":19,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"    let s : String = 1","highlight_start":22,"highlight_end":23}],"label":null,"suggested_replacement":"1.to_string()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"$DIR/json-bom-plus-crlf-multifile-aux.rs:19:22: error[E0308]: mismatched types
2020-03-21T20:51:34.3381680Z 41 "}
2020-03-21T20:51:34.3382082Z 42 {"message":"mismatched types","code":{"code":"E0308","explanation":"Expected type did not match the received type.
2020-03-21T20:51:34.3382587Z 52 //      |
2020-03-21T20:51:34.3382813Z 53 //    type `i32` assigned to variable `x`
2020-03-21T20:51:34.3383041Z 54 