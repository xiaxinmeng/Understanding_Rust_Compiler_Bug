\n\nWith this approach, x and y share ownership of the data via the `Rc` (reference\ncount type). `RefCell` essentially performs runtime borrow checking: ensuring\nthat at most one writer or multiple readers can access orrowed
[00:43:19] 110   --> $DIR/closure-borrow-spans.rs:77:13
[00:43:19] 
[00:43:19] 116 LL |     let y = x; //~ ERROR
[00:43:19] 117    |             ^ move out of `x` occurs here
[00:43:19] 118 LL |     f.use_ref();
[00:43:19] +    |     - borrow used here in later iteration of loop
[00:43:19] 120 
[00:43:19] 120 
[00:43:19] 121 error[E0501]: cannot borrow `x` as immutable because previous closure requires unique access
[00:43:19] 122   --> $DIR/closure-borrow-spans.rs:83:13
[00:43:19] 
[00:43:19] 128 LL |     let y = &x; //~ ERROR
[00:43:19] 129    |             ^^ borrow occurs here
[00:43:19] 130 LL |     f.use_ref();
[00:43:19] +    |     - borrow used here in later iteration of loop
[00:43:19] 132 
[00:43:19] 132 
[00:43:19] 133 error[E0501]: cannot borrow `x` as mutable because previous closure requires unique access
[00:43:19] 134   --> $DIR/closure-borrow-spans.rs:89:13
[00:43:19] 
[00:43:19] 140 LL |     let y = &mut x; //~ ERROR
[00:43:19] 141    |             ^^^^^^ borrow occurs here
[00:43:19] 142 LL |     f.use_ref();
[00:43:19] +    |     - borrow used here in later iteration of loop
[00:43:19] 144 
[00:43:19] 144 
[00:43:19] 145 error[E0597]: `x` does not live long enough
[00:43:19] 146   --> $DIR/closure-borrow-spans.rs:98:17
[00:43:19] 
[00:43:19] 152 LL |     }
[00:43:19] 153    |     - `x` dropped here while still borrowed
[00:43:19] 154 LL |     f.use_ref();
[00:43:19] +    |     - borrow used here in later iteration of loop
[00:43:19] +    |     - borrow used here in later iteration of loop
[00:/src/test/ui/nll/closure-borrow-spans.rs","byte_start":610,"byte_end":612,"line_start":16,"line_end":16,"column_start":13,"column_end":15,"is_primary":false,"text":[{"text":"    let f = || x.len();","highlight_start":13,"highlight_end":15}],"label":"borrow of `x` occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/closure-borrow-spans.rs","byte_start":634,"byte_end":635,"line_start":17,"line_end":17,"column_start":13,"column_end":14,"is_primary":true,"text":[{"text":"    let y = x; //~ ERROR","highlight_start":13,"highlight_end":14}],"label":"move out of `x` occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/closure-borrow-spans.rs","byte_start":613,"byte_end":614,"line_start":16,"line_end":16,"column_start":16,"column_end":17,"is_primary":false,"text":[{"text":"    let f = || x.len();","highlight_start":16,"highlight_end":17}],"label":"borrow occurs due to use in closure","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/closure-borrow-spans.rs","byte_start":651,"byte_end":652,"line_start":18,"line_end":18,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    f.use_ref();","highlight_start":5,"highlight_end":6}],"label":"borrow used here in later iteration of loop","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0505]: cannot move out of `x` because it is borrowed\n  --> /checkout/src/test/ui/nll/closure-borrow-spans.rs:17:13\n   |\nLL |     let f = || x.len();\n   |             -- - borrow occurs due to use in closure\n   |             |\n   |             borrow of `x` occurs here\nLL |     let y = x; //~ ERROR\n   |             ^ move out of `x` occurs here\nLL |     f.use_ref();\n   |     - borrow used here in later iteration of loop\n\n"}
[00:43:19] {"message":"cannot borrow `x` as mutable because it is also borrowed as immutable","code":{"code":"E0502","explanation":"\nThis error indicates that you are trying to borrow a variable as mutable when it\nhas already been borrowed as immutable.\n\nExample of erroneous code:\n\n