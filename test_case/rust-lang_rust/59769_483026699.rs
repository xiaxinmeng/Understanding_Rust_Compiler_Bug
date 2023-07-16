
[...]
thread 'rustc' panicked at '`"*"` is not a valid identifier', src\libsyntax_ext\proc_macro_server.rs:341:13
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
{"message":"proc macro panicked","code":null,"level":"error","spans":[{"file_name":"C:\\projects\\rust\\src/test\\ui\\proc-macro\\invalid-punct-ident-2.rs","byte_start":85,"byte_end":102,"line_start":6,"line_end":6,"column_start":1,"column_end":18,"is_primary":true,"text":[{"text":"invalid_ident!(); //~ ERROR proc macro panicked","highlight_start":1,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"message: `\"*\"` is not a valid identifier","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: proc macro panicked\n  --> C:\\projects\\rust\\src/test\\ui\\proc-macro\\invalid-punct-ident-2.rs:6:1\n   |\nLL | invalid_ident!(); //~ ERROR proc macro panicked\n   | ^^^^^^^^^^^^^^^^^\n   |\n   = help: message: `\"*\"` is not a valid identifier\n\n"}
{"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[...]
failures:
    [ui] ui\proc-macro\invalid-punct-ident-1.rs
    [ui] ui\proc-macro\invalid-punct-ident-2.rs
    [ui] ui\proc-macro\invalid-punct-ident-3.rs
