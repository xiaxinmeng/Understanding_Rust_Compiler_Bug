\n\nYou can find more information about borrowing in the rust-book:\nhttp://doc.rust-lang.org/book/first-edition/references-and-borrowing.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-2590.rs","byte_start":632,"byte_end":643,"line_start":21,"line_end":21,"column_start":9,"column_end":20,"is_primary":true,"text":[{"text":"        self.tokens //~ ERROR cannot move out of borrowed content","highlight_start":9,"highlight_end":20}],"label":"cannot move out of borrowed content","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0507]: cannot move out of borrowed content\n  --> /checkout/src/test/ui/issues/issue-2590.rs:21:9\n   |\nLL |         self.tokens //~ ERROR cannot move out of borrowed content\n   |         ^^^^^^^^^^^ cannot move out of borrowed content\n\n"}
[01:20:27] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:20:27] {"message":"For more information about this error, try `rustc --explain E0507`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0507`.\n"}
[01:20:27] ------------------------------------------
[01:20:27] 
[01:20:27] thread '[ui (nll)] ui/issues/issue-2590.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[01:20:27] 
[01:20:27] 
[01:20:27] ---- [ui (nll)] ui/mut/mutable-class-fields.rs#ast stdout ----
[01:20:27] diff of stderr:
[01:20:27] 
[01:20:27] 1 error[E0594]: cannot assign to `nyan.how_hungry`, as `nyan` is not declared as mutable
[01:20:27] 3    |
[01:20:27] 3    |
[01:20:27] - LL |   let nyan : cat = cat(52, 99);
[01:20:27] + LL |   let nyan : Cat = cat(52, 99);
[01:20:27] 5    |       ---- help: consider changing this to be mutable: `mut nyan`
[01:20:27] 6 LL |   nyan.how_hungry = 0; //[ast]~ ERROR cannot assign
[01:20:27] 
[01:20:27] 
[01:20:27] The actual stderr differed from the expected stderr.
[01:20:27] The actual stderr differed from the expected stderr.
[01:20:27] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mut/mutable-class-fields.ast.nll/mutable-class-fields.ast.nll.stderr
[01:20:27] To update references, rerun the tests and pass the `--bless` flag
[01:20:27] To only update this specific test, also pass `--test-args mut/mutable-class-fields.rs`
[01:20:27] 
[01:20:27] error in revision `ast`: 1 errors occurred comparing output.
[01:20:27] status: exit code: 1
[01:20:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mut/mutable-class-fields.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "ast" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mut/mutable-class-fields.ast.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mut/mutable-class-fields.ast.nll/auxiliary" "-A" "unused"
[01:20:27] ------------------------------------------
[01:20:27] 
[01:20:27] ------------------------------------------
[01:20:27] stderr:
[01:20:27] stderr:
[01:20:27] ------------------------------------------
[01:20:27] {"message":"cannot assign to `nyan.how_hungry`, as `nyan` is not declared as mutable","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/mut/mutable-class-fields.rs","byte_start":738,"byte_end":757,"line_start":28,"line_end":28,"column_start":3,"column_end":22,"is_primary":true,"text":[{"text":"  nyan.how_hungry = 0; //[ast]~ ERROR cannot assign","highlight_start":3,"highlight_end":22}],"label":"cannot assign","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider changing this to be mutable","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/mut/mutable-class-fields.rs","byte_start":710,"byte_end":714,"line_start":27,"line_end":27,"column_start":7,"column_end":11,"is_primary":true,"text":[{"text":"  let nyan : Cat = cat(52, 99);","highlight_start":7,"highlight_end":11}],"label":null,"suggested_replacement":"mut nyan","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0594]: cannot assign to `nyan.how_hungry`, as `nyan` is not declared as mutable\n  --> /checkout/src/test/ui/mut/mutable-class-fields.rs:28:3\n   |\nLL |   let nyan : Cat = cat(52, 99);\n   |       ---- help: consider changing this to be mutable: `mut nyan`\nLL |   nyan.how_hungry = 0; //[ast]~ ERROR cannot assign\n   |   ^^^^^^^^^^^^^^^^^^^ cannot assign\n\n"}
[01:20:27] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:20:27] {"message":"For more information about this error, try `rustc --explain E0594`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0594`.\n"}
[01:20:27] ------------------------------------------
[01:20:27] 
[01:20:27] thread '[ui (nll)] ui/mut/mutable-class-fields.rs#ast' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[01:20:27] 
[01:20:27] 
[01:20:27] ---- [ui (nll)] ui/regions/regions-creating-enums.rs stdout ----
[01:20:27] diff of stderr:
[01:20:27] 
[01:20:27] 1 error[E0515]: cannot return reference to temporary value
[01:20:27] 2   --> $DIR/regions-creating-enums.rs:33:16
[01:20:27] 3    |
[01:20:27] - LL |         return &ast::num((*f)(x)); //~ ERROR borrowed value does not live long enough
[01:20:27] + LL |         return &Ast::Num((*f)(x)); //~ ERROR borrowed value does not live long enough
[01:20:27] 5    |                ^-----------------
[01:20:27] 6    |                ||
[01:20:27] 7    |                |temporary value created here
[01:20:27] 10 error[E0515]: cannot return reference to temporary value
[01:20:27] 11   --> $DIR/regions-creating-enums.rs:38:16
[01:20:27] 12    |
[01:20:27] 12    |
[01:20:27] - LL |         return &ast::add(m_x, m_y);  //~ ERROR borrowed value does not live long enough
[01:20:27] + LL |         return &Ast::Add(m_x, m_y);  //~ ERROR borrowed value does not live long enough
[01:20:27] 14    |                ^------------------
[01:20:27] 15    |                ||
[01:20:27] 16    |                |temporary value created here
[01:20:27] 
[01:20:27] The actual stderr differed from the expected stderr.
[01:20:27] The actual stderr differed from the expected stderr.
[01:20:27] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-creating-enums.nll/regions-creating-enums.nll.stderr
[01:20:27] To update references, rerun the tests and pass the `--bless` flag
[01:20:27] To only update this specific test, also pass `--test-args regions/regions-creating-enums.rs`
[01:20:27] error: 1 errors occurred comparing output.
[01:20:27] status: exit code: 1
[01:20:27] status: exit code: 1
[01:20:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-creating-enums.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-creating-enums.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-creating-enums.nll/auxiliary" "-A" "unused"
[01:20:28] ------------------------------------------
[01:20:28] 
[01:20:28] ------------------------------------------
[01:20:28] stderr:
[01:20:28] stderr:
[01:20:28] ------------------------------------------
[01:20:28] {"message":"cannot return reference to temporary value","code":{"code":"E0515","explanation":"\nCannot return value that references local variable\n\nLocal variables, function parameters and temporaries are all dropped before the\nend of the function body. So a reference to them cannot be returned.\n\n