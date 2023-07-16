plain
[00:46:56] ....................................................................................................
[00:47:00] ....................................................................................................
[00:47:06] ....................................................................................................
[00:47:12] ....................................................................................................
[00:47:18] .................................................................F..................................
[00:47:30] .............i......................................................................................
[00:47:35] ................................ii..................................................................
[00:47:35] ................................ii..................................................................
[00:47:42] ................F...................................................................................
  let i_think_continually = 2;
[00:47:50] -    |         ^^^^^^^^^^^^^^^^^^^ help: consider using `_i_think_continually` instead
[00:47:50] +    |         ^^^^^^^^^^^^^^^^^^^ help: consider using `_i_think_continually` instead: `_i_think_continually`
[00:47:50] 7 note: lint level defined here
[00:47:50] -   --> $DIR/issue-47390-unused-variable-in-struct-pattern.rs:15:9
[00:47:50] +   --> $DIR/issue-47390-unused-variable-in-struct-pattern.rs:16:9
[00:47:50] 9    |
[00:47:50] 9    |
[00:47:50] 10 LL | #![warn(unused)] // UI tests pass `-A unused` (#43896)
[00:47:50] 
[00:47:50] 12    = note: #[warn(unused_variables)] implied by #[warn(unused)]
[00:47:50] 13 
[00:47:50] 13 
[00:47:50] 14 warning: unused variable: `mut_unused_var`
[00:47:50] +   --> $DIR/issue-47390-unused-variable-in-struct-pattern.rs:39:13
[00:47:50] 16    |
[00:47:50] 16    |
[00:47:50] 17 LL |     let mut mut_unused_var = 1;
[00:47:50] -    |             ^^^^^^^^^^^^^^ help: consider using `_mut_unused_var` instead
[00:47:50] +    |             ^^^^^^^^^^^^^^ help: consider using `_mut_unused_var` instead: `_mut_unused_var`
[00:47:50] 20 warning: unused variable: `var`
[00:47:50] -   --> $DIR/issue-47390-unused-variable-in-struct-pattern.rs:40:14
[00:47:50] +   --> $DIR/issue-47390-unused-variable-in-struct-pattern.rs:41:14
[00:47:50] 22    |
[00:47:50] 22    |
[00:47:50] 23 LL |     let (mut var, unused_var) = (1, 2);
[00:47:50] -    |              ^^^ help: consider using `_var` instead
[00:47:50] +    |              ^^^ help: consider using `_var` instead: `_var`
[00:47:50] 26 warning: unused variable: `unused_var`
[00:47:50] -   --> $DIR/issue-47390-unused-variable-in-struct-pattern.rs:40:19
[00:47:50] +   --> $DIR/issue-47390-unused-variable-in-struct-pattern.rs:41:19
[00:47:50] 28    |
[00:47:50] 28    |
[00:47:50] 29 LL |     let (mut var, unused_var) = (1, 2);
[00:47:50] -    |                   ^^^^^^^^^^ help: consider using `_unused_var` instead
[00:47:50] +    |                   ^^^^^^^^^^ help: consider using `_unused_var` instead: `_unused_var`
[00:47:50] 31 
[00:47:50] 32 warning: unused variable: `corridors_of_light`
[00:47:50] +   --> $DIR/issue-47390-unused-variable-in-struct-pattern.rs:43:26
[00:47:50] 34    |
[00:47:50] 34    |
[00:47:50] 35 LL |     if let SoulHistory { corridors_of_light,
[00:47:50] 36    |                          ^^^^^^^^^^^^^^^^^^ help: try ignoring the field: `corridors_of_light: _`
[00:47:50] 37 
[00:47:50] 37 
[00:47:50] 38 warning: variable `hours_are_suns` is assigned to, but never used
[00:47:50] +   --> $DIR/issue-47390-unused-variable-in-struct-pattern.rs:44:30
[00:47:50] 40    |
[00:47:50] 40    |
[00:47:50] 41 LL |                          mut hours_are_suns,
[00:47:50] 
[00:47:50] 
[00:47:50] 44    = note: consider using `_hours_are_suns` instead
[00:47:50] 45 
[00:47:50] 46 warning: value assigned to `hours_are_suns` is never read
[00:47:50] -   --> $DIR/issue-47390-unused-variable-in-struct-pattern.rs:45:9
[00:47:50] +   --> $DIR/issue_ } => {}
[00:47:50] 
[00:47:50] 
[00:47:50]     // Referenced struct
[00:47:50]     match &bag {
[00:47:50]         &Large::Suit { case: _ } => {}
[00:47:50] 
[00:47:50] 
[00:47:50]     // Boxed struct
[00:47:50]     match box bag {
[00:47:50]         box Large::Suit { case: _ } => {}
[00:47:50] 
[00:47:50] 
[00:47:50]     // Tuple with struct
[00:47:50]     match (bag,) {
[00:47:50]         (Large::Suit { case: _ },) => {}
[00:47:50] 
[00:47:50] 
[00:47:50]     // Slice with struct
[00:47:50]     match [bag] {
[00:47:50]         [Large::Suit { case: _ }] => {}
[00:47:50] 
[00:47:50] 
[00:47:50]     // Tuple struct with struct
[00:47:50]     match Tuple(bag, ()) {
[00:47:50]         Tuple(Large::Suit { case: _ }, ()) => {}
[00:47:50] }
[00:47:50] 
[00:47:50] 
[00:47:50] 
[00:47:50] 
[00:47:50] The actual fixed differed from the expected fixed.
[00:47:50] Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.fixed
[00:47:50] To update references, run this command from build directory:
[00:47:50] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'lint/issue-47390-unused-variable-in-struct-pattern.rs'
[00:47:50] error: 2 errors occurred comparing output.
[00:47:50] status: exit code: 0
[00:47:50] status: exit code: 0
[00:47:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:47:50] ------------------------------------------
[00:47:50] 
[00:47:50] ------------------------------------------
[00:47:50] stderr:
[00:47:50] stderr:
[00:47:50] ------------------------------------------
[00:47:50] {"message":"unused variable: `i_think_continually`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs","byte_start":823,"byte_end":842,"line_start":32,"line_end":32,"column_start":9,"column_end":28,"is_primary":true,"text":[{"text":"    let i_think_continually = 2;","highlight_start":9,"highlight_end":28}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs","byte_start":557,"byte_end":563,"line_start":16,"line_end":16,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"#![warn(unused)] // UI tests pass `-A unused` (#43896)","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacemen/issue-47390-unused-variable-in-struct-pattern.rs","byte_start":1050,"byte_end":1053,"line_start":41,"line_end":41,"column_start":14,"column_end":17,"is_primary":true,"text":[{"text":"    let (mut var, unused_var) = (1, 2);","highlight_start":14,"highlight_end":17}],"label":null,"suggested_replacement":"_var","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `var`\n  --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:41:14\n   |\nLL |     let (mut var, unused_var) = (1, 2);\n   |              ^^^ help: consider using `_var` instead: `_var`\n\n"}
[00:47:50] {"message":"unused variable: `unused_var`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs","byte_start":1055,"byte_end":1065,"line_start":41,"line_end":41,"column_start":19,"column_end":29,"is_primary":true,"text":[{"text":"    let (mut var, unused_var) = (1, 2);","highlight_start":19,"highlight_end":29}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"consider using `_unused_var` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs","byte_start":1055,"byte_end":1065,"line_start":41,"line_end":41,"column_start":19,"column_end":29,"is_primary":true,"text":[{"text":"    let (mut var, unused_var) = (1, 2);","highlight_start":19,"highlight_end":29}],"label":null,"suggested_replacement":"_unused_var","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `unused_var`\n  --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:41:19\n   |\nLL |     let (mut var, unused_var) = (1, 2);\n   |                   ^^^^^^^^^^ help: consider using `_unused_var` instead: `_unused_var`\n\n"}
[00:47:50] {"message":"unused variable: `corridors_of_light`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs","byte_start":1103,"byte_end":1121,"line_start":43,"line_end":43,"column_start":26,"column_end":44,"is_primary":true,"text":[{"text":"    if let SoulHistory { corridors_of_light,","highlight_start":26,"highlight_end":44}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"try ignoring the field","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs","byte_start":1103,"byte_end":1121,"line_start":43,"line_end":43,"column_start":26,"column_end":44,"is_primary":true,"text":[{"text":"    if let SoulHistory { corridors_of_light,","highlight_start":26,"highlight_end":44}],"label":null,"suggested_replacement":"corridors_of_light: _","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `corridors_of_light`\n  --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:43:26\n   |\nLL |     if let SoulHistory { corridors_of_light,\n   |                          ^^^^^^^^^^^^^^^^^^ help: try ignoring the field: `corridors_of_light: _`\n\n"}
[00:47:50] {"message":"variable `hours_are_suns` is assigned to, but never used","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs","byte_start":1152,"byte_end":1166,"line_start":44,"line_end":44,"column_start":30,"column_end":44,"is_primary":true,"text":[{"text":"                         mut hours_are_suns,","highlight_start":30,"highlight_end":44}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"consider using `_hours_are_suns` instead","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: variable `hours_are_suns` is assigned to, but never used\n  --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:44:30\n   |\nLL |                          mut hours_are_suns,\n   |                              ^^^^^^^^^^^^^^\n   |\n   = note: consider using `_hours_are_suns` instead\n\n"}
[00:47:50] {"message":"value assigned to `hours_are_suns` is never read","code":{"code":"unused_assignments","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs","byte_start":1262,"byte_end":1276,"line_start":46,"line_end":46,"column_start":9,"column_end":23,"is_primary":true,"text":[{"text":"        hours_are_suns = false;","highlight_start":9,"highlight_end":23}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-patterntruct-pattern.rs:60:24\n   |\nLL |         &Large::Suit { case } => {}\n   |                        ^^^^ help: try ignoring the field: `case: _`\n\n"}
[00:47:50] {"message":"unused variable: `case`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs","byte_start":1577,"byte_end":1581,"line_start":65,"line_end":65,"column_start":27,"column_end":31,"is_primary":true,"text":[{"text":"        box Large::Suit { case } => {}","highlight_start":27,"highlight_end":31}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"try ignoring the field","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs","byte_start":1577,"byte_end":1581,"line_start":65,"line_end":65,"column_start":27,"column_end":31,"is_primary":true,"text":[{"text":"        box Large::Suit { case } => {}","highlight_start":27,"highlight_end":31}],"label":null,"suggested_replacement":"case: _","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `case`\n  --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:65:27\n   |\nLL |         box Large::Suit { case } => {}\n   |                           ^^^^ help: try ignoring the field: `case: _`\n\n"}
[00:47:50] {"message":"unused variable: `case`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs","byte_start":1665,"byte_end"ct-pattern.rs","byte_start":1754,"byte_end":1758,"line_start":75,"line_end":75,"column_start":24,"column_end":28,"is_primary":true,"text":[{"text":"        [Large::Suit { case }] => {}","highlight_start":24,"highlight_end":28}],"label":null,"suggested_replacement":"case: _","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `case`\n  --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:75:24\n   |\nLL |         [Large::Suit { case }] => {}\n   |                        ^^^^ help: try ignoring the field: `case: _`\n\n"}
[00:47:50] {"message":"unused variable: `case`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs","byte_start":1863,"byte_end":1867,"line_start":80,"line_end":80,"column_start":29,"column_end":33,"is_primary":true,"text":[{"text":"        Tuple(Large::Suit { case }, ()) => {}","highlight_start":29,"highlight_end":33}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"try ignoring the field","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs","byte_start":1863,"byte_end":1867,"line_start":80,"line_end":80,"column_start":29,"column_end":33,"is_primary":true,"text":[{"text":"        Tuple(Large::Suit { case }, ()) => {}","highlight_start":29,"highlight_end":33}],"label":null,"suggested_replacement":"case: _","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `case`\n  --> /checkout/src,"highlight_start":9,"highlight_end":13}],"label":null,"suggested_replacement":"","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: variable does not need to be mutable\n  --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:39:9\n   |\nLL |     let mut mut_unused_var = 1;\n   |         ----^^^^^^^^^^^^^^\n   |         |\n   |         help: remove this `mut`\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:16:9\n   |\nLL | #![warn(unused)] // UI tests pass `-A unused` (#43896)\n   |         ^^^^^^\n   = note: #[warn(unused_mut)] implied by #[warn(unused)]\n\n"}
[00:47:50] {"message":"variable does not need to be mutable","code":{"code":"unused_mut","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs","byte_start":1046,"byte_end":1053,"line_start":41,"line_end":41,"column_start":10,"column_end":17,"is_primary":true,"text":[{"text":"    let (mut var, unused_var) = (1, 2);","highlight_start":10,"highlight_end":17}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"remove this `mut`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs","byte_start":1046,"byte_end":1050,"line_start":41,"line_end":41,"column_start":10,"column_end":14,"is_primary":true,"text":[{"text":"    let (mut var, unused_var) = (1, 2);","highlight_start":10,"highlight_end":14}],"label":null,"suggested_replacement":"","expansion":null}],"children":[]," 101
[00:47:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-24690.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-24690.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-24690.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:47:50] ------------------------------------------
[00:47:50] 
[00:47:50] ------------------------------------------
[00:47:50] stderr:
[00:47:50] stderr:
[00:47:50] ------------------------------------------
[00:47:50] {"message":"unused variable: `theOtherTwo`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/span/issue-24690.rs","byte_start":889,"byte_end":900,"line_start":23,"line_end":23,"column_start":9,"column_end":20,"is_primary":true,"text":[{"text":"    let theOtherTwo = 2; //~ WARN should have a snake case name","highlight_start":9,"highlight_end":20}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/span/issue-24690.rs","byte_start":752,"byte_end":758,"line_start":18,"line_end":18,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"#![warn(unused)]","highlightt":9,"highlight_end":15}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"#[warn(non_snake_case)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: variable `theTwo` should have a snake case name such as `the_two`\n  --> /checkout/src/test/ui/span/issue-24690.rs:22:9\n   |\nLL |     let theTwo = 2; //~ WARN should have a snake case name\n   |         ^^^^^^\n   |\n   = note: #[warn(non_snake_case)] on by default\n\n"}
[00:47:50] {"message":"variable `theOtherTwo` should have a snake case name such as `the_other_two`","code":{"code":"non_snake_case","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/span/issue-24690.rs","byte_start":889,"byte_end":900,"line_start":23,"line_end":23,"column_start":9,"column_end":20,"is_primary":true,"text":[{"text":"    let theOtherTwo = 2; //~ WARN should have a snake case name","highlight_start":9,"highlight_end":20}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"warning: variable `theOtherTwo` should have a snake case name such as `the_other_two`\n  --> /checkout/src/test/ui/span/issue-24690.rs:23:9\n   |\nLL |     let theOtherTwo = 2; //~ WARN should have a snake case name\n   |         ^^^^^^^^^^^\n\n"}
[00:47:50] {"message":"compilation successful","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/span/issue-24690.rs","byte_start":777,"byte_end":1004,"line_start":21,"line_end":26,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn main() { //~ ERROR compilation successful","highlight_
