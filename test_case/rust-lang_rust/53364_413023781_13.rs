\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc1598-generic-associated-types/construct_with_other_type.rs","byte_start":1134,"byte_end":1136,"line_start":34,"line_end":34,"column_start":40,"column_end":42,"is_primary":true,"text":[{"text":"    type Baa<'a> = &'a <T as Foo>::Bar<'a, 'static>;","highlight_start":40,"highlight_end":42}],"label":"lifetime parameter not allowed","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0110]: lifetime parameters are not allowed on this type\n  --> /checkout/src/test/ui/rfc1598-generic-associated-types/construct_with_other_type.rs:34:40\n   |\nLL |     type Baa<'a> = &'a <T as Foo>::Bar<'a, 'static>;\n   |                                        ^^ lifetime parameter not allowed\n\n"}
[00:52:18] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:52:18] {"message":"For more information about this error, try `rustc --explain E0110`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0110`.\n"}
[00:52:18] ------------------------------------------
[00:52:18] 
[00:52:18] thread '[ui] ui/rfc1598-generic-associated-types/construct_with_other_type.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3166:9
[00:52:18] 
[00:52:18] 
[00:52:18] ---- [ui] ui/rfc1598-generic-associated-types/generic-associated-types-where.rs stdout ----
[00:52:18] normalized stderr:
[00:52:18] warning: where clauses on associated types as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes
[00:52:18]    |
[00:52:18]    |
[00:52:18] LL |     type Assoc where Self: Sized;
[00:52:18] 
[00:52:18] 
[00:52:18] warning: generic associated type parameters as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes
[00:52:18]    |
[00:52:18]    |
[00:52:18] LL |     type Assoc2<T> where T: Display;
[00:52:18] 
[00:52:18] 
[00:52:18] warning: where clauses on associated types as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes
[00:52:18]    |
[00:52:18]    |
[00:52:18] LL |     type Assoc2<T> where T: Display;
[00:52:18]    |     ^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^
[00:52:18] 
[00:52:18] warning: generic associated type arguments as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes
[00:52:18]    |
[00:52:18]    |
[00:52:18] LL |     type Assoc3<T> where T: Iterator = Vec<T>;
[00:52:18] 
[00:52:18] 
[00:52:18] warning: where clauses on associated types as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes
[00:52:18]    |
[00:52:18]    |
[00:52:18] LL |     type Assoc3<T> where T: Iterator = Vec<T>;
[00:52:18] 
[00:52:18] 
[00:52:18] warning: generic associated type arguments as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes
[00:52:18]    |
[00:52:18]    |
[00:52:18] LL |     type WithDefault<'a, T> = &'a Iterator<T>;
[00:52:18] 
[00:52:18] 
[00:52:18] 
[00:52:18] 
[00:52:18] 
[00:52:18] The actual stderr differed from the expected stderr.
[00:52:18] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1598-generic-associated-types/generic-associated-types-where/generic-associated-types-where.stderr
[00:52:18] To update references, rerun the tests and pass the `--bless` flag
[00:52:18] To only update this specific test, also pass `--test-args rfc1598-generic-associated-types/generic-associated-types-where.rs`
[00:52:18] error: 1 errors occurred comparing output.
[00:52:18] status: exit code: 1
[00:52:18] status: exit code: 1
[00:52:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc1598-generic-associated-types/generic-associated-types-where.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1598-generic-associated-types/generic-associated-types-where/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1598-generic-associated-types/generic-associated-types-where/auxiliary" "-A" "unused"
[00:52:18] ------------------------------------------
[00:52:18] 
[00:52:18] ------------------------------------------
[00:52:18] stderr:
[00:52:18] stderr:
[00:52:18] ------------------------------------------
[00:52:18] {"message":"where clauses on associated types as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/rfc1598-generic-associated-types/generic-associated-types-where.rs","byte_start":646,"byte_end":675,"line_start":19,"line_end":19,"column_start":5,"column_end":34,"is_primary":true,"text":[{"text":"    type Assoc where Self: Sized;","highlight_start":5,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: where clauses on associated types as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes\n  --> /checkout/src/test/ui/rfc1598-generic-associated-types/generic-associated-types-where.rs:19:5\n   |\nLL |     type Assoc where Self: Sized;\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:52:18] {"message":"generic associated type parameters as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/rfc1598-generic-associated-types/generic-associated-types-where.rs","byte_start":680,"byte_end":712,"line_start":20,"line_end":20,"column_start":5,"column_end":37,"is_primary":true,"text":[{"text":"    type Assoc2<T> where T: Display;","highlight_start":5,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: generic associated type parameters as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes\n  --> /checkout/src/test/ui/rfc1598-generic-associated-types/generic-associated-types-where.rs:20:5\n   |\nLL |     type Assoc2<T> where T: Display;\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:52:18] {"message":"where clauses on associated types as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/rfc1598-generic-associated-types/generic-associated-types-where.rs","byte_start":680,"byte_end":712,"line_start":20,"line_end":20,"column_start":5,"column_end":37,"is_primary":true,"text":[{"text":"    type Assoc2<T> where T: Display;","highlight_start":5,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: where clauses on associated types as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes\n  --> /checkout/src/test/ui/rfc1598-generic-associated-types/generic-associated-types-where.rs:20:5\n   |\nLL |     type Assoc2<T> where T: Display;\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:52:18] {"message":"generic associated type parameters as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/rfc1598-generic-associated-types/generic-associated-types-where.rs","byte_start":717,"byte_end":732,"line_start":21,"line_end":21,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    type Assoc3<T>;","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: generic associated type arguments as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes\n  --> /checkout/src/test/ui/rfc1598-generic-associated-types/generic-associated-types-where.rs:30:5\n   |\nLL |     type Assoc2<T> = Vec<T>;\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:52:18] {"message":"generic associated type arguments as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/rfc1598-generic-associated-types/generic-associated-types-where.rs","byte_start":905,"byte_end":947,"line_start":31,"line_end":31,"column_start":5,"column_end":47,"is_primary":true,"text":[{"text":"    type Assoc3<T> where T: Iterator = Vec<T>;","highlight_start":5,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: generic associated type arguments as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes\n  --> /checkout/src/test/ui/rfc1598-generic-associated-types/generic-associated-types-where.rs:31:5\n   |\nLL |     type Assoc3<T> where T: Iterator = Vec<T>;\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:52:18] {"message":"where clauses on associated types as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/rfc1598-generic-associated-types/generic-associated-types-where.rs","byte_start":905,"byte_end":947,"line_start":31,"line_end":31,"column_start":5,"column_end":47,"is_primary":true,"text":[{"text":"    type Assoc3<T> where T: Iterator = Vec<T>;","highlight_start":5,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: where clauses on associated types as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes\n  --> /checkout/src/test/ui/rfc1598-generic-associated-types/generic-associated-types-where.rs:31:5\n   |\nLL |     type Assoc3<T> where T: Iterator = Vec<T>;\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:52:18] {"message":"generic associated type arguments as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/rfc1598-generic-associated-types/generic-associated-types-where.rs","byte_start":952,"byte_end":994,"line_start":32,"line_end":32,"column_start":5,"column_end":47,"is_primary":true,"text":[{"text":"    type WithDefault<'a, T> = &'a Iterator<T>;","highlight_start":5,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansio LL | |     //~^ ERROR lifetime parameters are not allowed on this type [E0110]
[00:52:18] + LL | |         + Deref<Target = Self::Item<'b>>;
[00:52:18] + 
[00:52:18] 1 error[E0261]: use of undeclared lifetime name `'b`
[00:52:18] 2   --> $DIR/generic_associated_type_undeclared_lifetimes.rs:22:37
[00:52:18] 3    |
[00:52:18] 3    |
[00:52:18] 
[00:52:18] 
[00:52:18] The actual stderr differed from the expected stderr.
[00:52:18] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1598-generic-associated-types/generic_associated_type_undeclared_lifetimes/generic_associated_type_undeclared_lifetimes.stderr
[00:52:18] To update references, rerun the tests and pass the `--bless` flag
[00:52:18] To only update this specific test, also pass `--test-args rfc1598-generic-associated-types/generic_associated_type_undeclared_lifetimes.rs`
[00:52:18] error: 1 errors occurred comparing output.
[00:52:18] status: exit code: 1
[00:52:18] status: exit code: 1
[00:52:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc1598-generic-associated-types/generic_associated_type_undeclared_lifetimes.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1598-generic-associated-types/generic_associated_type_undeclared_lifetimes/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1598-generic-associated-types/generic_associated_type_undeclared_lifetimes/auxiliary" "-A" "unused"
[00:52:18] ------------------------------------------
[00:52:18] 
[00:52:18] ------------------------------------------
[00:52:18] stderr:
[00:52:18] stderr:
[00:52:18] ------------------------------------------
[00:52:18] {"message":"generic associated type parameters as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/rfc1598-generic-associated-types/generic_associated_type_undeclared_lifetimes.rs","byte_start":663,"byte_end":677,"line_start":19,"line_end":19,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"    type Item<'a>;","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: generic associated type parameters as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes\n  --> /checkout/src/test/ui/rfc1598-generic-associated-types/generic_associated_type_undeclared_lifetimes.rs:19:5\n   |\nLL |     type Item<'a>;\n   |     ^^^^^^^^^^^^^^\n\n"}
[00:52:18] {"message":"generic associated type parameters as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/rfc1598-generic-associatert":41,"column_end":52,"is_primary":true,"text":[{"text":"    fn iter<'a>(&'a self) -> Self::Iter<'undeclared>;","highlight_start":41,"highlight_end":52}],"label":"undeclared lifetime","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0261]: use of undeclared lifetime name `'undeclared`\n  --> /checkout/src/test/ui/rfc1598-generic-associated-types/generic_associated_type_undeclared_lifetimes.rs:26:41\n   |\nLL |     fn iter<'a>(&'a self) -> Self::Iter<'undeclared>;\n   |                                         ^^^^^^^^^^^ undeclared lifetime\n\n"}
[00:52:18] {"message":"lifetime parameters are not allowed on this type","code":{"code":"E0110","explanation":"\nYou tried to give a lifetime parameter to a type which doesn't need it.\nErroneous code example:\n\n