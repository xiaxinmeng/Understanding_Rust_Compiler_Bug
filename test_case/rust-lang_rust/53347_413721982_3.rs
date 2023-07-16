\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2126-crate-paths/keyword-crate-as-identifier.rs","byte_start":516,"byte_end":521,"line_start":14,"line_end":14,"column_start":9,"column_end":14,"is_primary":true,"text":[{"text":"    let crate = 0;","highlight_start":9,"highlight_end":14}],"label":"not a unit struct/variant or constant","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0532]: expected unit struct/variant or constant, found module `crate`\n  --> /checkout/src/test/ui/rfc-2126-crate-paths/keyword-crate-as-identifier.rs:14:9\n   |\nLL |     let crate = 0;\n   |         ^^^^^ not a unit struct/variant or constant\n\n"}
[00:47:59] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:47:59] {"message":"For more information about this error, try `rustc --explain E0532`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0532`.\n"}
[00:47:59] ------------------------------------------
[00:47:59] 
[00:47:59] thread '[ui] ui/rfc-2126-crate-paths/keyword-crate-as-identifier.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3166:9
[00:47:59] 
