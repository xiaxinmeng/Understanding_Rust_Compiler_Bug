plain
[00:51:47] ....................................................................................................
[00:51:50] ....................................................................................................
[00:51:52] ....................................................................................................
[00:51:55] .................................................................................................i..
[00:51:59] .................................F.F..F.FFF.FF...F..................................................
[00:52:05] ....................................................................................................
[00:52:08] ........................i...........................................................................
[00:52:12] ....................................................................................................
[00:52:14] ....................................................................................................
[00:52:14] ....................................................................................................
[00:52:17] ....................................................................i...............................
[00:52:18] .............................
[00:52:18] failures:
[00:52:18] 
[00:52:18] ---- [ui] ui/rfc1598-generic-associated-types/collections.rs stdout ----
[00:52:18] diff of stderr:
[00:52:18] 
[00:52:18] + warning: generic associated type parameters as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes
[00:52:18] +   --> $DIR/collections.rs:22:5
[00:52:18] +    |
[00:52:18] + LL |     type Iter<'iter>: Iterator<Item=&'iter T>;
[00:52:18] + 
[00:52:18] + 
[00:52:18] + warning: generic associated type parameters as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes
[00:52:18] +   --> $DIR/collections.rs:25:5
[00:52:18] +    |
[00:52:18] + LL | /     type Sibling<U>: Collection<U> =
[00:52:18] + LL | |         <<Self as Collection<T>>::Family as CollectionFamily>::Member<U>;
[00:52:18] + 
[00:52:18] + 
[00:52:18] + warning: generic associated type parameters as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes
[00:52:18] +   --> $DIR/collections.rs:38:5
[00:52:18] +    |
[00:52:18] + LL |     type Member<T>: Collection<T, Family = Self>;
[00:52:18] + 
[00:52:18] + 
[00:52:18] + warning: generic associated type arguments as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes
[00:52:18] +   --> $DIR/collections.rs:44:5
[00:52:18] +    |
[00:52:18] + LL |     type Member<T> = Vec<T>;
[00:52:18] + 
[00:52:18] + 
[00:52:18] + warning: generic associated type arguments as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes
[00:52:18] +   --> $DIR/collections.rs:48:5
[00:52:18] +    |
[00:52:18] + LL |     type Iter<'iter> = std::slice::Iter<'iter, T>;
[00:52:18] + 
[00:52:18] + 
[00:52:18] 1 error[E0109]: type parameters are not allowed on this type
[00:52:18] 2   --> $DIR/collections.rs:65:90
[00:52:18] 
[00:52:18] 
[00:52:18] The actual stderr differed from the expected stderr.
[00:52:18] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1598-generic-associated-types/collections/collections.stderr
[00:52:18] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1598-generic-associated-types/collections/collections.stderr
[00:52:18] To update references, rerun the tests and pass the `--bless` flag
[00:52:18] To only update this specific test, also pass `--test-args rfc1598-generic-associated-types/collections.rs`
[00:52:18] error: 1 errors occurred comparing output.
[00:52:18] status: exit code: 1
[00:52:18] status: exit code: 1
[00:52:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc1598-generic-associated-types/collections.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1598-generic-associated-types/collections/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1598-generic-associated-types/collections/auxiliary" "-A" "unused"
[00:52:18] ------------------------------------------
[00:52:18] 
[00:52:18] ------------------------------------------
[00:52:18] stderr:
[00:52:18] stderr:
[00:52:18] ------------------------------------------
[00:52:18] {"message":"generic associated type parameters as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/rfc1598-generic-associated-types/collections.rs","byte_start":855,"byte_end":897,"line_start":22,"line_end":22,"column_start":5,"column_end":47,"is_primary":true,"text":[{"text":"    type Iter<'iter>: Iterator<Item=&'iter T>;","highlight_start":5,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: generic associated type parameters as part of the `generic_associa}],"children":[],"rendered":"warning: generic associated type arguments as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes\n  --> /checkout/src/test/ui/rfc1598-generic-associated-types/collections.rs:44:5\n   |\nLL |     type Member<T> = Vec<T>;\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:52:18] {"message":"generic associated type arguments as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/rfc1598-generic-associated-types/collections.rs","byte_start":1563,"byte_end":1609,"line_start":48,"line_end":48,"column_start":5,"column_end":51,"is_primary":true,"text":[{"text":"    type Iter<'iter> = std::slice::Iter<'iter, T>;","highlight_start":5,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: generic associated type arguments as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes\n  --> /checkout/src/test/ui/rfc1598-generic-associated-types/collections.rs:48:5\n   |\nLL |     type Iter<'iter> = std::slice::Iter<'iter, T>;\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:52:18] {"message":"type parameters are not allowed on this type","code":{"code":"E0109","explanation":"\nYou tried to give a type parameter to a type which doesn't need it. Erroneous\ncode example:\n\n