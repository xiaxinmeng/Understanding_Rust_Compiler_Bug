plain
[00:49:56] ....................................................................................................
[00:50:00] ....................................................................................................
[00:50:05] ....................................................................................................
[00:50:11] ....................................................................................................
[00:50:17] ...................................................................F................................
[00:50:28] ................i...................................................................................
[00:50:34] ...................................ii...............................................................
[00:50:34] ...................................ii...............................................................
[00:50:41] ...................F................................................................................
[00:50:48] 40    |
[00:50:48] 40    |
[00:50:48] 41 LL |                          mut hours_are_suns,
[00:50:48] 
[00:50:48] 
[00:50:48] 44    = note: consider using `_hours_are_suns` instead
[00:50:48] 45 
[00:50:48] 46 warning: value assigned to `hours_are_suns` is never read
[00:50:48] +   --> $DIR/issue-47390-unused-variable-in-struct-pattern.rs:46:9
[00:50:48] 48    |
[00:50:48] 48    |
[00:50:48] 49 LL |         hours_are_suns = false;
[00:50:48] 
[00:50:48] 51    |
[00:50:48] 52 note: lint level defined here
[00:50:48] -   --> $DIR/issue-47390-unused-variable-in-struct-pattern.rs:15:9
[00:50:48] -   --> $DIR/issue-47390-unused-variable-in-struct-pattern.rs:15:9
[00:50:48] +   --> $DIR/issue-47390-unused-variable-in-struct-pattern.rs:16:9
[00:50:48] 54    |
[00:50:48] 55 LL | #![warn(unused)] // UI tests pass `-A unused` (#43896)
[00:50:48] 
[00:50:48] 57    = note: #[warn(unused_assignments)] implied by #[warn(unused)]
[00:50:48] 58 
[00:50:48] 58 
[00:50:48] 59 warning: unused variable: `case`
[00:50:48] +   --> $DIR/issue-47390-unused-variable-in-struct-pattern.rs:55:23
[00:50:48] 61    |
[00:50:48] 61    |
[00:50:48] 62 LL |         Large::Suit { case } => {}
[00:50:48] 63    |                       ^^^^ help: try ignoring the field: `case: _`
[00:50:48] 64 
[00:50:48] 64 
[00:50:48] 65 warning: unused variable: `case`
[00:50:48] -   --> $DIR/issue-47390-unused-variable-in-struct-pattern.rs:59:24
[00:50:48 if let SoulHistory { corridors_of_light: _,
[00:50:48]                          mut hours_are_suns,
[00:50:48]                          endless_and_singing: true } = who_from_the_womb_remembered {
[00:50:48]         hours_are_suns = false;
[00:50:48] 
[00:50:48] 
[00:50:48]     let bag = Large::Suit {
[00:50:48]         case: ()
[00:50:48] 
[00:50:48] 
[00:50:48]     // Plain struct
[00:50:48]     match bag {
[00:50:48]         Large::Suit { case: _ } => {}
[00:50:48] 
[00:50:48] 
[00:50:48]     // Referenced struct
[00:50:48]     match &bag {
[00:50:48]         &Large::Suit { case: _ } => {}
[00:50:48] 
[00:50:48] 
[00:50:48]     // Boxed struct
[00:50:48]     match box bag {
[00:50:48]         box Large::Suit { case: _ } => {}
[00:50:48] 
[00:50:48] 
[00:50:48]     // Tuple with struct
[00:50:48]     match (bag,) {
[00:50:48]         (Large::Suit { case: _ },) => {}
[00:50:48] 
[00:50:48] 
[00:50:48]     // Slice with struct
[00:50:48]     match [bag] {
[00:50:48]         [Large::Suit { case: _ }] => {}
[00:50:48] 
[00:50:48] 
[00:50:48]     // Tuple struct with struct
[00:50:48]     match Tuple(bag, ()) {
[00:50:48]         Tuple(Large::Suit { case: _ }, ()) => {}
[00:50:48] }
[00:50:48] 
[00:50:48] 
[00:50:48] 
[00:50:48] 
[00:50:48] The actual fixed differed from the expected fixed.
[00:50:48] Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.fixed
[00:50:48] To update references, run this command from build directory:
[00:50:48] /checkout/src/test/ui/up"file_name":"/checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs","byte_start":1046,"byte_end":1050,"line_start":41,"line_end":41,"column_start":10,"column_end":14,"is_primary":true,"text":[{"text":"    let (mut var, unused_var) = (1, 2);","highlight_start":10,"highlight_end":14}],"label":null,"suggested_replacement":"","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: variable does not need to be mutable\n  --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:41:10\n   |\nLL |     let (mut var, unused_var) = (1, 2);\n   |          ----^^^\n   |          |\n   |          help: remove this `mut`\n\n"}
[00:50:48] ------------------------------------------
[00:50:48] 
[00:50:48] thread '[ui] ui/lint/issue-47390-unused-variable-in-struct-pattern.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[00:50:48] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:50:48] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:50:48] 
[00:50:48] ---- [ui] ui/span/issue-24690.rs stdout ----
[00:50:48]  diff of stderr:
[00:50:48] 
[00:50:48] 2   --> $DIR/issue-24690.rs:23:9
[00:50:48] 3    |
[00:50:48] 4 LL |     let theOtherTwo = 2; //~ WARN should have a snake case name
[00:50:48] -    |         ^^^^^^^^^^^ help: consider using `_theOtherTwo` instead
[00:50:48] +    |         ^^^^^^^^^^^ help: consider using `_theOtherTwo` instead: `_theOtherTwo`
[00:50:48] 7 note: lint level defined here
[00:50:48] 8   --> $DIR/issue-24690.rs:18:9
[00:50:48] 
[00:50:48] 
[00:50:48] 
[00:50:48] The actual stderr differed from the expected stderr.
[00:50:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-24690.stderr
[00:50:48] To update references, run this command from build directory:
[00:50:48] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'span/issue-24690.rs'
[00:50:48] error: 1 errors occurred comparing output.
[00:50:48] status: exit code: 101
[00:50:48] status: exit code: 101
[00:50:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-24690.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-24690.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-24690.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:50:48] ------------------------------------------
[00:50:48] 
[00:50:48] ------------------------------------------
[00:50:48] stderr:
[00:50:48] stderr:
[00:50:48] ------------------------------------------
[00:50:48] {"message":"unused variable: `theOtherTwo`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/span/issue-24690.rs","byte_start":889,"byte_end":900,"line_start":23,"line_end":23,"column_start":9,"column_end":20,"is_primary":true,"text":[{"text":"    let theOtherTwo = 2; //~ WARN should have a snake case name","highlight_start":9,"he a snake case name such as `the_two`","code":{"code":"non_snake_case","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/span/issue-24690.rs","byte_start":830,"byte_end":836,"line_start":22,"line_end":22,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"    let theTwo = 2; //~ WARN should have a snake case name","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"#[warn(non_snake_case)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: variable `theTwo` should have a snake case name such as `the_two`\n  --> /checkout/src/test/ui/span/issue-24690.rs:22:9\n   |\nLL |     let theTwo = 2; //~ WARN should have a snake case name\n   |         ^^^^^^\n   |\n   = note: #[warn(non_snake_case)] on by default\n\n"}
[00:50:48] {"message":"variable `theOtherTwo` should have a snake case name such as `the_other_two`","code":{"code":"non_snake_case","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/span/issue-24690.rs","byte_start":889,"byte_end":900,"line_start":23,"line_end":23,"column_start":9,"column_end":20,"is_primary":true,"text":[{"text":"    let theOtherTwo = 2; //~ WARN should have a snake case name","highlight_start":9,"highlight_end":20}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"warning: variable `theOtherTwo` should have a snake case name such as `the_other_two`\n  --> /checkout/src/test/ui/span/issue-24690.rs:23:9\n   |\nLL |     let theOtherTwo = 2; //~ WARN should havetravis_time:start:071f2ad3
