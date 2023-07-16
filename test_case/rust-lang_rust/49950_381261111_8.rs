\n"},"level":"error","spans":[],"children":[{"message":"...which requires computing the supertraits of `t2`...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issue-12511.rs","byte_start":467,"byte_end":480,"line_start":11,"line_end":11,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"trait t1 : t2 {","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which again requires computing the supertraits of `t1`, completing the cycle","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issue-12511.rs","byte_start":486,"byte_end":499,"line_start":14,"line_end":14,"column_start":1,"column_end":14,"is_primary":true,"text":[{"text":"trait t2 : t1 {","highlight_start":1,"highlight_end":14}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when computing the supertraits of `t1`\n   |\nnote: ...which requires computing the supertraits of `t2`...\n  --> /checkout/src/test/ui/issue-12511.rs:11:1\n   |\nLL | trait t1 : t2 {\n   | ^^^^^^^^^^^^^\nnote: ...which again requires computing the supertraits of `t1`, completing the cycle\n  --> /checkout/src/test/ui/issue-12511.rs:14:1\n   |\nLL | trait t2 : t1 {\n   | ^^^^^^^^^^^^^\n\n"}
[00:45:12] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:45:12] {"message":"For more information about this error, try `rustc --explain E0391`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0391`.\n"}
---
[00:45:12] - error[E0391]: cyclic dependency detected
[00:45:12] -   --> $DIR/issue-23302-1.rs:14:9
[00:45:12] + error[E0391]: cycle detected when const-evaluating `X::A::{{initializer}}`
[00:45:12] 3    |
[00:45:12] - LL |     A = X::A as isize, //~ ERROR E0391
[00:45:12] -    |         ^^^^^^^^^^^^^ cyclic reference
[00:45:12] -    |
[00:45:12] - note: the cycle begins when const-evaluating `X::A::{{initializer}}`...
[00:45:12] + note: ...which requires computing layout of `X`...
[00:45:12] 8   --> $DIR/issue-23302-1.rs:14:9
[00:45:12] 9    |
[00:45:12] 10 LL |     A = X::A as isize, //~ ERROR E0391
[00:45:12]
[00:45:12] -    |         ^^^^^^^^^^^^^
[00:45:12] - note: ...which then requires computing layout of `X`...
[00:45:12] -   --> $DIR/issue-23302-1.rs:14:9
[00:45:12] -    |
[00:45:12] - LL |     A = X::A as isize, //~ ERROR E0391
[00:45:12] 16    |         ^^^^
[00:45:12] -    = note: ...which then again requires const-evaluating `X::A::{{initializer}}`, completing the cycle.
[00:45:12] + note: ...which again requires const-evaluating `X::A::{{initializer}}`, completing the cycle
---
[00:45:12] Actual stderr san}\n