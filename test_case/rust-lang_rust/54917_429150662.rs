
[00:45:23] stderr:

[00:45:23] ------------------------------------------

[00:45:23] {"message":"unused return value of `foo` that must be used","code":{"code":"unused_must_use","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/must_use-unit.rs","byte_start":145,"byte_end":151,"line_start":14,"line_end":14,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    foo(); //~ unused return value of `foo`","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/lint/must_use-unit.rs","byte_start":33,"byte_end":48,"line_start":3,"line_end":3,"column_start":9,"column_end":24,"is_primary":true,"text":[{"text":"#![deny(unused_must_use)]","highlight_start":9,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: unused return value of `foo` that must be used\n  --> /checkout/src/test/ui/lint/must_use-unit.rs:14:5\n   |\nLL |     foo(); //~ unused return value of `foo`\n   |     ^^^^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/must_use-unit.rs:3:9\n   |\nLL | #![deny(unused_must_use)]\n   |         ^^^^^^^^^^^^^^^\n\n"}

[00:45:23] {"message":"unused return value of `bar` that must be used","code":{"code":"unused_must_use","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/must_use-unit.rs","byte_start":190,"byte_end":196,"line_start":16,"line_end":16,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    bar(); //~ unused return value of `bar`","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unused return value of `bar` that must be used\n  --> /checkout/src/test/ui/lint/must_use-unit.rs:16:5\n   |\nLL |     bar(); //~ unused return value of `bar`\n   |     ^^^^^^\n\n"}

[00:45:23] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}

[00:45:23] 

[00:45:23] ------------------------------------------

[00:45:23] 
