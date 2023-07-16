\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/issue-39544.rs","byte_start":1336,"byte_end":1344,"line_start":52,"line_end":52,"column_start":13,"column_end":21,"is_primary":true,"text":[{"text":"    let _ = &mut w.x; //~ ERROR cannot borrow","highlight_start":13,"highlight_end":21}],"label":"cannot borrow as mutable","suggested_replacement":null,"expansion":null}],"children":[{"message":"the value which is causing this path not to be mutable is...: `*w`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0596]: cannot borrow immutable item `w.x` as mutable\n  --> /checkout/src/test/ui/did_you_mean/issue-39544.rs:52:13\n   |\nLL |     let _ = &mut w.x; //~ ERROR cannot borrow\n   |             ^^^^^^^^ cannot borrow as mutable\n   |\n   = note: the value which is causing this path not to be mutable is...: `*w`\n\n"}
[00:49:37] {"message":"cannot assign to immutable item `*x.0`","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/did_you_mean/issue-39544.rs","byte_start":1437,"byte_end":1445,"line_start":58,"line_end":58,"column_start":5,"column_end":13,"is_primary":true,"text":[{"text":"    *x.0 = 1;","highlight_start":5,"highlight_end":13}],"label":"cannot mutate","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0594]: cannot assign to immutable item `*x.0`\n  --> /checkout/src/test/ui/did_you_mean/issue-39544.rs:58:5\n   |\nLL |     *x.0 = 1;\n   |     ^^^^^^^^ cannot mutate\n\n"}
[00:49:37] {"message":"aborting due to 12 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 12 previous errors\n\n"}
[00:49:37] {"message":"Some errors occurred: E0594, E0596.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0594, E0596.\n"}
[00:49:37] {"message":"For more information about an error, try `rustc --explain E0594`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0594`.\n"}
[00:49:37]
[00:49:37] ------------------------------------------
[00:49:37]
[00:49:37] thread '[ui (nll)] ui/did_you_mean/issue-39544.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2919:9
[00:49:37]
[00:49:37] ---- [ui (nll)] ui/error-codes/E0389.rs stdout ----
[00:49:37]  diff of stderr:
[00:49:37]
[00:49:37] - error[E0594]: cannot assign to immutable item `fancy_ref.num`
[00:49:37] + error[E0594]: cannot assign to data in a `&` reference
[00:49:37] 2   --> $DIR/E0389.rs:18:5
[00:49:37] 3    |
[00:49:37] + LL |     let fancy_ref = &(&mut fancy);
[00:49:37] +    |                     ------------- help: consider changing this to be a mutable reference: `&mut`
[00:49:37] 4 LL |     fancy_ref.num = 6; //~ ERROR E0389
[00:49:37] -    |     ^^^^^^^^^^^^^^^^^ cannot mutate
[00:49:37] -    |
[00:49:37] -    = note: Value not mutable causing this error: `*fancy_ref`
[00:49:37] +    |     ^^^^^^^^^^^^^^^^^ `fancy_ref` is a `&` reference, so the data it refers to cannot be written
---
[00:49:37] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'error-codes/E0389.rs'
[00:49:37]
[00:49:37] error: 1 errors occurred comparing output.
[00:49:37] status: exit code: 101
[00:49:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0389.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0389.stage2-x86_64-unknown-linux-gnu" "-Znll" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0389.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:49:37] {"message":"cannot assign to data in a `&` reference","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0389.rs","byte_start":592,"byte_end":609,"line_start":18,"line_end":18,"column_start":5,"column_end":22,"is_primary":true,"text":[{"text":"    fancy_ref.num = 6; //~ ERROR E0389","highlight_start":5,"highlight_end":22}],"label":"`fancy_ref` is a `&` reference, so the data it refers to cannot be written","suggested_replacement":null,"expansion":null}],"children":[{"message":"consider changing this to be a mutable reference: `&mut`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0389.rs","byte_start":573,"byte_end":586,"line_start":17,"line_end":17,"column_start":21,"column_end":34,"is_primary":true,"text":[{"text":"    let fancy_ref = &(&mut fancy);","highlight_start":21,"highlight_end":34}],"label":null,"suggested_replacement":"","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0594]: cannot assign to data in a `&` reference\n  --> /checkout/src/test/ui/error-codes/E0389.rs:18:5\n   |\nLL |     let fancy_ref = &(&mut fancy);\n   |                     ------------- help: consider changing this to be a mutable reference: `&mut`\nLL |     fancy_ref.num = 6; //~ ERROR E0389\n   |     ^^^^^^^^^^^^^^^^^ `fancy_ref` is a `&` reference, so the data it refers to cannot be written\n\n"}
[00:49:37] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:49:37] {"message":"For more information about this error, try `rustc --explain E0594`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0594`.\n"}
[00:49:37]
[00:49:37] ------------------------------------------
[00:49:37]
[00:49:37] thread '[ui (nll)] ui/error-codes/E0389.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2919:9
[00:49:37]
[00:49:37] ---- [ui (nll)] ui/issue-36400.rs stdout ----
[00:49:37]  diff of stderr:
[00:49:37]
[00:49:37] 4 LL |     f(&mut *x); //~ ERROR cannot borrow immutable
[00:49:37] 5    |       ^^^^^^^ cannot borrow as mutable
[00:49:37] 6    |
[00:49:37] -    = note: Value not mutable causing this error: `x`
[00:49:37] +    = note: the value which is causing this path not to be mutable is...: `x`
---
[00:49:37] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'issue-36400.rs'
[00:49:37]
[00:49:37] error: 1 errors occurred comparing output.
[00:49:37] status: exit code: 101
[00:49:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-36400.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-36400.stage2-x86_64-unknown-linux-gnu" "-Znll" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-36400.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:49:37] {"message":"cannot borrow immutable item `*x` as mutable","code":{"code":"E0596","explanation":"\nThis error occurs because you tried to mutably borrow a non-mutable variable.\n\nExample of erroneous code:\n\n