\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-51714.rs","byte_start":492,"byte_end":504,"line_start":12,"line_end":12,"column_start":14,"column_end":26,"is_primary":true,"text":[{"text":"    |_:  [_; return || {}] | {};","highlight_start":14,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0572]: return statement outside of function body\n  --> /checkout/src/test/ui/issues/issue-51714.rs:12:14\n   |\nLL |     |_:  [_; return || {}] | {};\n   |              ^^^^^^^^^^^^\n\n"}
[01:00:57] {"message":"irrefutable while-let pattern","code":{"code":"E0165","explanation":"\nA while-let pattern attempts to match the pattern, and enters the body if the\nmatch was successfubout an error, try `rustc --explain E0165`.\n"}
[01:00:57] ------------------------------------------
[01:00:57] 
[01:00:57] thread '[ui] ui/issues/issue-51714.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:00:57] 
[01:00:57] 
[01:00:57] ---- [ui] ui/issues/issue-54302-cases.rs stdout ----
[01:00:57] diff of stderr:
