plain
[00:50:40] .................................................................................................... 2500/4545
[00:50:45] .................................................................................................... 2600/4545
[00:50:48] .................................................................................................... 2700/4545
[00:50:51] .................................................................................................... 2800/4545
[00:50:55] .......................................................F............................................ 2900/4545
[00:51:01] ....................................................................................i.i..ii......... 3100/4545
[00:51:05] .................................................................................................... 3200/4545
[00:51:08] .................................................................................................... 3300/4545
[00:51:11] ....................i............................................................................... 3400/4545
---
[00:51:40] ..............................i..................................................................... 4200/4545
[00:51:43] .................................................................................................... 4300/4545
[00:51:46] .................................................................................................... 4400/4545
[00:51:49] ....................................................................................i............... 4500/4545
L |     let mut x = &mut 1; //[lexical]~ ERROR: variable does not need to be mutable
[00:51:50] -    |         |
[00:51:50] -    |         help: remove this `mut`
[00:51:50] - 
[00:51:50] - error: variable does not need to be mutable
[00:51:50] - error: variable does not need to be mutable
[00:51:50] -   --> $DIR/lint-unused-mut-variables.rs:84:9
[00:51:50] -    |
[00:51:50] - LL |     let mut v : &mut Vec<()> = &mut vec![]; //[lexical]~ ERROR: variable does not need to be mutable
[00:51:50] -    |         |
[00:51:50] -    |         help: remove this `mut`
[00:51:50] - 
[00:51:50] - error: variable does not need to be mutable
[00:51:50] - error: variable does not need to be mutable
[00:51:50] 120   --> $DIR/lint-unused-mut-variables.rs:61:13
[00:51:50] 121    |
[00:51:50] 122 LL |     fn what(mut foo: isize) {} //[lexical]~ ERROR: variable does not need to be mutable
[00:51:50] 125    |             help: remove this `mut`
[00:51:50] 126 
[00:51:50] 127 error: variable does not need to be mutable
[00:51:50] -   --> $DIR/lint-unused-mut-variables.rs:79:20
[00:51:50] -   --> $DIR/lint-unused-mut-variables.rs:79:20
[00:51:50] -    |
[00:51:50] - LL |     fn mut_ref_arg(mut arg : &mut [u8]) -> &mut [u8] {
[00:51:50] -    |                    |
[00:51:50] -    |                    help: remove this `mut`
[00:51:50] - 
[00:51:50] - error: variable does not need to be mutable
[00:51:50] - error: variable does not need to be mutable
[00:51:50] 136   --> $DIR/lint-unused-mut-variables.rs:143:9
[00:51:50] 137    |
[00:51:50] 138 LL |     let mut b = vec![2]; //[lexical]~ ERROR: variable does not need to be mutable
[00:51:50] 
[00:51:50] 146 LL | #[deny(unused_mut)]
[00:51:50] 148 
[00:51:50] - error: aborting due to 17 previous errors
[00:51:50] + error: aborting due to 12 previous errors
[00:51:50] 150 
[00:51:50] 150 
[00:51:50] 151 
[00:51:50] 
[00:51:50] 
[00:51:50] The actual stderr differed from the expected stderr.
[00:51:50] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unused-mut-variables.nll/lint-unused-mut-variables.nll.stderr
[00:51:50] To update references, rerun the tests and pass the `--bless` flag
[00:51:50] To only update this specific test, also pass `--test-args lint/lint-unused-mut-variables.rs`
[00:51:50] 
[00:51:50] error in revision `nll`: 1 errors occurred comparing output.
[00:51:50] status: exit code: 1
[00:51:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-unused-mut-variables.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unused-mut-variables.nll/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unused-mut-variables.nll/auxiliary" "-A" "unused"
[00:51:50] ------------------------------------------
[00:51:50] 
[00:51:50] ------------------------------------------
[00:51:50] stderr:
[00:51:50] stderr:
[00:51:50] ------------------------------------------
[00:51:50] {"message":"variable does not need to be mutable","code":{"code":"unused_mut","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-unused-mut-variables.rs","byte_start":2295,"byte_end":2300,"line_start":59,"line_end":59,"column_start":14,"column_end":19,"is_primary":true,"text":[{"text":"    let x = |mut y: isize| 10; //[lexical]~ ERROR: variable does not need to be mutable","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-unused-mut-variables.rs","byte_start":687,"byte_end":697,"line_start":19,"line_end":19,"column_start":9,"column_end":19,"is_primary":true,"text":[{"text":"#![deny(unused_mut)]","highlight_start":9,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"remove this `mut`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-unused-mut-variables.rs","byte_start":2295,"byte_end":2299,"line_start":59,"line_end":59,"column_start":14,"column_end":18,"is_primary":true,"text":[{"text":"    let x = |mut y: isize| 10; //[lexical]~ ERROR: variable does not need to be mutable","highlight_start":14,"highlight_end":18}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: variable does not need to be mutable\n  --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:59:14\n   |\nLL |     let x = |mut y: isize| 10; //[lexical]~ ERROR: variable does not need to b] {"message":"variable does not need to be mutable","code":{"code":"unused_mut","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-unused-mut-variables.rs","byte_start":893,"byte_end":898,"line_start":26,"line_end":26,"column_start":9,"column_end":14,"is_primary":true,"text":[{"text":"    let mut a = 2; //[lexical]~ ERROR: variable does not need to be mutable","highlight_start":9,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove this `mut`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-unused-mut-variables.rs","byte_start":893,"byte_end":897,"line_start":26,"line_end":26,"column_start":9,"column_end":13,"is_primary":true,"text":[{"text":"    let mut a = 2; //[lexical]~ ERROR: variable does not need to be mutable","highlight_start":9,"highlight_end":13}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: variable does not need to be mutable\n  --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:26:9\n   |\nLL |     let mut a = 2; //[lexical]~ ERROR: variable does not need to be mutable\n   |         ----^\n   |         |\n   |         help: remove this `mut`\n\n"}
[00:51:50] {"message":"variable does not need to be mutable","code":{"code":"unused_mut","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-unused-mut-variables.rs","byte_start":1042,"byte_end":1047,"line_start":28,"line_end":28,"column_start":9,"column_end":14,"is_primary":true,"text":[{"text":"    let mut b = 3; //[lexical]~ ERROR: variable does not need to be mutable","highlight_start":9,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove this `mut`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-unused-mut-variables.rs","byte_start":1042,"byte_end":1046,"line_start":28,"line_end":28,"column_start":9,"column_end":13,"is_primary":true,"text":[{"text":"    let mut b = 3; //[lexical]~ ERROR: variable does not need to be mutable","highlight_start":9,"highlight_end":13}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: variable does not need to be mutable\n  --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:28:9\n   |\nLL |     let mut b = 3; //[lexical]~ ERROR: variable does not need to be mutable\n   |         ----^\n   |         |\n   |         help: remove this `mut`\n\n"}
[00:51:50] {"message":"variable does not need to be mutable","code":{"code":"unused_mut","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-unused-mut-variables.rs","byte_start":1191,"byte_end":1196,"line_start":30,"line_end":30,"column_start":9,"column_end":14,"is_primary":true,"text":[{"text":"    let mut a = vec![3]; //[lexical]~ ERROR: variable does not need to be mutable","highlight_start":9,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"childrry":true,"text":[{"text":"    let (mut a, b) = (1, 2); //[lexical]~ ERROR: variable does not need to be mutable","highlight_start":10,"highlight_end":14}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: variable does not need to be mutable\n  --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:32:10\n   |\nLL |     let (mut a, b) = (1, 2); //[lexical]~ ERROR: variable does not need to be mutable\n   |          ----^\n   |          |\n   |          help: remove this `mut`\n\n"}
[00:51:50] {"message":"variable does not need to be mutable","code":{"code":"unused_mut","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-unused-mut-variables.rs","byte_start":1521,"byte_end":1526,"line_start":34,"line_end":34,"column_start":9,"column_end":14,"is_primary":true,"text":[{"text":"    let mut a; //[lexical]~ ERROR: variable does not need to be mutable","highlight_start":9,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove this `mut`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-unused-mut-variables.rs","byte_start":1521,"byte_end":1525,"line_start":34,"line_end":34,"column_start":9,"column_end":13,"is_primary":true,"text":[{"text":"    let mut a; //[lexical]~ ERROR: variable does not need to be mutable","highlight_start":9,"highlight_end":13}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"childrp: remove this `mut`\n\n"}
[00:51:50] {"message":"variable does not need to be mutable","code":{"code":"unused_mut","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-unused-mut-variables.rs","byte_start":1894,"byte_end":1899,"line_start":47,"line_end":47,"column_start":9,"column_end":14,"is_primary":true,"text":[{"text":"        mut x => {} //[lexical]~ ERROR: variable does not need to be mutable","highlight_start":9,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove this `mut`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-unused-mut-variables.rs","byte_start":1894,"byte_end":1898,"line_start":47,"line_end":47,"column_start":9,"column_end":13,"is_primary":true,"text":[{"text":"        mut x => {} //[lexical]~ ERROR: variable does not need to be mutable","highlight_start":9,"highlight_end":13}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: variable does not need to be mutable\n  --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:47:9\n   |\nLL |         mut x => {} //[lexical]~ ERROR: variable does not need to be mutable\n   |         ----^\n   |         |\n   |         help: remove this `mut`\n\n"}
[00:51:50] {"message":"variable does not need to be mutable","code":{"code":"unused_mut","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-unused-mut-variables.rs","byte_start":2070,"byte_end":2075,"linon_applicability":null,"expansion":null}],"children":[{"message":"remove this `mut`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-unused-mut-variables.rs","byte_start":2467,"byte_end":2471,"line_start":61,"line_end":61,"column_start":13,"column_end":17,"is_primary":true,"text":[{"text":"    fn what(mut foo: isize) {} //[lexical]~ ERROR: variable does not need to be mutable","highlight_start":13,"highlight_end":17}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: variable does not need to be mutable\n  --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:61:13\n   |\nLL |     fn what(mut foo: isize) {} //[lexical]~ ERROR: variable does not need to be mutable\n   |             ----^^^\n   |             |\n   |             help: remove this `mut`\n\n"}
[00:51:50] {"message":"variable does not need to be mutable","code":{"code":"unused_mut","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-unused-mut-variables.rs","byte_start":4687,"byte_end":4692,"line_start":143,"line_end":143,"column_start":9,"column_end":14,"is_primary":true,"text":[{"text":"    let mut b = vec![2]; //[lexical]~ ERROR: variable does not need to be mutable","highlight_start":9,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-unused-mut-variables.rs","byte_start":4611,"byte_end":4621,"line_start":139,"line_end":139,"column_start":8,"column_end":18,"is_primary":true,"text":[{"text":"#[deny(unused_mut)]","highlight_start":8,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"remove this `mut`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-unused-mut-variables.rs","byte_start":4687,"byte_end":4691,"line_start":143,"line_end":143,"column_start":9,"column_end":13,"is_primary":true,"text":[{"text":"    let mut b = vec![2]; //[lexical]~ ERROR: variable does not need to be mutable","highlight_start":9,"highlight_end":13}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: variable does not need to be mutable\n  --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:143:9\n   |\nLL |     let mut b = vec![2]; //[lexical]~ ERROR: variable does not need to be mutable\n   |         ----^\n   |         |\n   |         help: remove this `mut`\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:139:8\n   |\nLL | #[deny(unused_mut)]\n   |        ^^^^^^^^^^\n\n"}
[00:51:50] {"message":"aborting due to 12 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 12 previous errors\n\n"}
[00:51:50] ------------------------------------------
[00:51:50] 
[00:51:50] thread '[ui] ui/lint/lint-unused-mut-variables.rs#nll' panicked at 'explicit panic', tools/compilet
travis_time:end:070df06c:start=1538595173702401235,finish=1538598284618638358,duration=3110916237123
