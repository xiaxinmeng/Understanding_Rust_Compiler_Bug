\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-36400.rs","byte_start":532,"byte_end":539,"line_start":15,"line_end":15,"column_start":7,"column_end":14,"is_primary":true,"text":[{"text":"    f(&mut *x); //~ ERROR cannot borrow immutable","highlight_start":7,"highlight_end":14}],"label":"cannot borrow as mutable","suggested_replacement":null,"expansion":null}],"children":[{"message":"the value which is causing this path not to be mutable is...: `x`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0596]: cannot borrow immutable item `*x` as mutable\n  --> /checkout/src/test/ui/issue-36400.rs:15:7\n   |\nLL |     f(&mut *x); //~ ERROR cannot borrow immutable\n   |       ^^^^^^^ cannot borrow as mutable\n   |\n   = note: the value which is causing this path not to be mutable is...: `x`\n\n"}
[00:49:37] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:49:37] {"message":"For more information about this error, try `rustc --explain E0596`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0596`.\n"}
[00:49:37]
[00:49:37] ------------------------------------------
[00:49:37]
[00:49:37] thread '[ui (nll)] ui/issue-36400.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2919:9
[00:49:37]
[00:49:37] ---- [ui (nll)] ui/rfc-2005-default-binding-mode/explicit-mut.rs stdout ----
[00:49:37]  diff of stderr:
[00:49:37]
[00:49:37] - error[E0594]: cannot assign to immutable item `*n`
[00:49:37] + error[E0594]: cannot assign to data in a `&` reference
[00:49:37] 2   --> $DIR/explicit-mut.rs:17:13
[00:49:37] 3    |
[00:49:37] + LL |         Some(n) => {
[00:49:37] +    |              - help: consider changing this to be a mutable reference: `&mut`
[00:49:37] 4 LL |             *n += 1; //~ ERROR cannot assign to immutable
[00:49:37] -    |             ^^^^^^^ cannot mutate
[00:49:37] +    |             ^^^^^^^
[00:49:37] 6
[00:49:37] - error[E0594]: cannot assign to immutable item `*n`
[00:49:37] + error[E0594]: cannot assign to data in a `&` reference
[00:49:37] 8   --> $DIR/explicit-mut.rs:25:13
[00:49:37] 9    |
[00:49:37] + LL |         Some(n) => {
[00:49:37] +    |              - help: consider changing this to be a mutable reference: `&mut`
[00:49:37] 10 LL |             *n += 1; //~ ERROR cannot assign to immutable
[00:49:37] -    |             ^^^^^^^ cannot mutate
[00:49:37] +    |             ^^^^^^^
[00:49:37] 12
[00:49:37] - error[E0594]: cannot assign to immutable item `*n`
[00:49:37] + error[E0594]: cannot assign to data in a `&` reference
[00:49:37] 14   --> $DIR/explicit-mut.rs:33:13
[00:49:37] 15    |
[00:49:37] + LL |         Some(n) => {
[00:49:37] +    |              - help: consider changing this to be a mutable reference: `&mut`
[00:49:37] 16 LL |             *n += 1; //~ ERROR cannot assign to immutable
---
[00:49:37] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-bi"column_end":20,"is_primary":true,"text":[{"text":"            *n += 1; //~ ERROR cannot assign to immutable","highlight_start":13,"highlight_end":20}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"consider changing this to be a mutable reference: `&mut`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs","byte_start":639,"byte_end":640,"line_start":16,"line_end":16,"column_start":14,"column_end":15,"is_primary":true,"text":[{"text":"        Some(n) => {","highlight_start":14,"highlight_end":15}],"label":null,"suggested_replacement":"","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0594]: cannot assign to data in a `&` reference\n  --> /checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs:17:13\n   |\nLL |         Some(n) => {\n   |              - help: consider changing this to be a mutable reference: `&mut`\nLL |             *n += 1; //~ ERROR cannot assign to immutable\n   |             ^^^^^^^\n\n"}
[00:49:37] {"message":"cannot assign to data in a `&` reference","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs","byte_start":828,"byte_end":835,"line_start":25,"line_end":25,"column_start":13,"column_end":20,"is_primary":true,"text":[{"text":"            *n += 1; //~ ERROR cannot assign to immutable","highlight_start":13,"highlight_end":20}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"consider changing this to be a mutable reference: `&mut`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs","byte_start":808,"byte_end":809,"line_start":24,"line_end":24,"column_start":14,"column_end":15,"is_primary":true,"text":[{"text":"        Some(n) => {","highlight_start":14,"highlight_end":15}],"label":null,"suggested_replacement":"","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0594]: cannot assign to data in a `&` reference\n  --> /checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs:25:13\n   |\nLL |         Some(n) => {\n   |              - help: consider changing this to be a mutable reference: `&mut`\nLL |             *n += 1; //~ ERROR cannot assign to immutable\n   |             ^^^^^^^\n\n"}
[00:49:37] {"message":"cannot assign to data in a `&` reference","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs","byte_start":997,"byte_end":1004,"line_start":33,"line_end":33,"column_start":13,"column_end":20,"is_primary":true,"text":[{"text":"            *n += 1; //~ ERROR cannot assign to immutable","highlight_start":13,"highlight_end":20}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"consider changing this to be a mutable reference: `&mut`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs","byte_start":977,"byte_end":978,"line_start":32,"line_end":32,"column_start":14,"column_end":15,"is_primary":true,"text":[{"text":"        Some(n) => {","highlight_start":14,"highlight_end":15}],"label":null,"suggested_replacement":"","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0594]: cannot assign to data in a `&` reference\n  --> /checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs:33:13\n   |\nLL |         Some(n) => {\n   |              - help: consider changing this to be a mutable reference: `&mut`\nLL |             *n += 1; //~ ERROR cannot assign to immutable\n   |             ^^^^^^^\n\n"}
[00:49:37] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:49:37] {"message":"For more information about this error, try `rustc --explain E0594`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0594`.\n"}
[00:49:37]
[00:49:37] ------------------------------------------
[00:49:37]
[00:49:37] thread '[ui (nll)] ui/rfc-2005-default-binding-mode/explicit-mut.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2919:9
[00:49:37]
[00:49:37] ---- [ui (nll)] ui/rfc-2005-default-binding-mode/enum.rs stdout ----
[00:49:37]  diff of stderr:
[00:49:37]
[00:49:37] - error[E0594]: cannot assign to immutable item `*x`
[00:49:37] + error[E0594]: cannot assign to data in a `&` reference
[00:49:37] 2   --> $DIR/enum.rs:19:5
[00:49:37] 3    |
[00:49:37] + LL |     let Wrap(x) = &Wrap(3);
[00:49:37] +    |              - help: consider changing this to be a mutable reference: `&mut`
[00:49:37] 4 LL |     *x += 1; //~ ERROR cannot assign to immutable
[00:49:37] -    |     ^^^^^^^ cannot mutate
[00:49:37] +    |     ^^^^^^^
[00:49:37] 6
[00:49:37] - error[E0594]: cannot assign to immutable item `*x`
[00:49:37] + error[E0594]: cannot assign to data in a `&` reference
[00:49:37] 8   --> $DIR/enum.rs:23:9
[00:49:37] 9    |
[00:49:37] + LL |     if let Some(x) = &Some(3) {
[00:49:37] +    |                 - help: consider changing this to be a mutable reference: `&mut`
[00:49:37] 10 LL |         *x += 1; //~ ERROR cannot assign to immutable
[00:49:37] -    |         ^^^^^^^ cannot mutate
[00:49:37] +    |         ^^^^^^^
[00:49:37] 12
[00:49:37] - error[E0594]: cannot assign to immutable item `*x`
[00:49:37] + error[E0594]: cannot assign to data in a `&` reference
[00:49:37] 14   --> $DIR/enum.rs:29:9
[00:49:37] 15    |
[00:49:37] + LL |     while let Some(x) = &Some(3) {
[00:49:37] +    |                    - help: consider changing this to be a mutable reference: `&mut`
[00:49:37] 16 LL |         *x += 1; //~ ERROR cannot assign to immutable
---
[00:49:37] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'rfc-2005-default-binding-mode/enum.rs'
[00:49:37]
[00:49:37] error: 1 errors occurred comparing output.
[00:49:37] status: exit code: 101
[00:49:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/enum.stage2-x86_64-unknown-linux-gnu" "-Znll" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/enum.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:49:37] {"message":"cannot assign to data in a `&` reference","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs","byte_start":568,"byte_end":575,"line_start":19,"line_end":19,"column_start":5,"column_end":12,"is_primary":true,"text":[{"text":"    *x += 1; //~ ERROR cannot assign to immutable","highlight_start":5,"highlight_end":12}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"consider changing this to be a mutable reference: `&mut`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs","byte_start":549,"byte_end":550,"line_start":18,"line_end":18,"column_start":14,"column_end":15,"is_primary":true,"text":[{"text":"    let Wrap(x) = &Wrap(3);","highlight_start":14,"highlight_end":15}],"label":null,"suggested_replacement":"","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0594]: cannot assign to data in a `&` reference\n  --> /checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs:19:5\n   |\nLL |     let Wrap(x) = &Wrap(3);\n   |              - help: consider changing this to be a mutable reference: `&mut`\nLL |     *x += 1; //~ ERROR cannot assign to immutable\n   |     ^^^^^^^\n\n"}
[00:49:37] {"message":"cannot assign to data in a `&` reference","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs","byte_start":656,"byte_end":663,"line_start":23,"line_end":23,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        *x += 1; //~ ERROR cannot assign to immutable","highlight_start":9,"highlight_end":16}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"consider changing this to be a mutable reference: `&mut`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs","byte_start":632,"byte_end":633,"line_start":22,"line_end":22,"column_start":17,"column_end":18,"is_primary":true,"text":[{"text":"    if let Some(x) = &Some(3) {","highlight_start":17,"highlight_end":18}],"label":null,"suggested_replacement":"","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0594]: cannot assign to data in a `&` reference\n  --> /checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs:23:9\n   |\nLL |     if let Some(x) = &Some(3) {\n   |                 - help: consider changing this to be a mutable reference: `&mut`\nLL |         *x += 1; //~ ERROR cannot assign to immutable\n   |         ^^^^^^^\n\n"}
[00:49:37] {"message":"cannot assign to data in a `&` reference","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs","byte_start":783,"byte_end":790,"line_start":29,"line_end":29,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        *x += 1; //~ ERROR cannot assign to immutable","highlight_start":9,"highlight_end":16}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"consider changing this to be a mutable reference: `&mut`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs","byte_start":759,"byte_end":760,"line_start":28,"line_end":28,"column_start":20,"column_end":21,"is_primary":true,"text":[{"text":"    while let Some(x) = &Some(3) {","highlight_start":20,"highlight_end":21}],"label":null,"suggested_replacement":"","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0594]: cannot assign to data in a `&` reference\n  --> /checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs:29:9\n   |\nLL |     while let Some(x) = &Some(3) {\n   |                    - help: consider changing this to be a mutable reference: `&mut`\nLL |         *x += 1; //~ ERROR cannot assign to immutable\n   |         ^^^^^^^\n\n"}
[00:49:37] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:49:37] {"message":"For more information about this error, try `rustc --explain E0594`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0594`.\n"}
[00:49:37]
[00:49:37] ------------------------------------------
[00:49:37]
[00:49:37] thread '[ui (nll)] ui/rfc-2005-default-binding-mode/enum.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2919:9
[00:49:37]
[00:49:37] ---- [ui (nll)] ui/span/borrowck-call-is-borrow-issue-12224.rs stdout ----
[00:49:37]  diff of stderr:
[00:49:37]
[00:49:37] 24 LL |     f.f.call_mut(())
[00:49:37] 25    |     ^^^ cannot borrow as mutable
[00:49:37] 26    |
[00:49:37] -    = note: Value not mutable causing this error: `*f`
[00:49:37] +    = note: the value which is causing this path not to be mutable is...: `*f`
---
[00:49:37] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'span/borrowck-call-is-borrow-issue-12224.rs'
[00:49:37]
[00:49:37] error: 1 errors occurred comparing output.
[00:49:37] status: exit code: 101
[00:49:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/borrowck-call-is-borrow-issue-12224.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/borrowck-call-is-borrow-issue-12224.stage2-x86_64-unknown-linux-gnu" "-Znll" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/borrowck-call-is-borrow-issue-12224.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:49:37] {"message":"cannot borrow `f` as mutable more than once at a time","code":{"code":"E0499","explanation":"\nA variable was borrowed as mutable more than once. Erroneous code example:\n\n