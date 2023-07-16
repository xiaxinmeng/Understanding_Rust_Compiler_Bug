\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-12511.rs","byte_start":11,"byte_end":13,"line_start":1,"line_end":1,"column_start":12,"column_end":14,"is_primary":true,"text":[{"text":"trait T1 : T2 {","highlight_start":12,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"...which requires computing the supertraits of `T2`...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-12511.rs","byte_start":56,"byte_end":58,"line_start":5,"line_end":5,"column_start":12,"column_end":14,"is_primary":true,"text":[{"text":"trait T2 : T1 {","highlight_start":12,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which again requires computing the supertraits of `T1`, completing the cycle","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"cycle used when processing ``","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-12511.rs","byte_start":0,"byte_end":13,"line_start":1,"line_end":1ting the supertraits of `T`, completing the cycle
[01:01:44] + note: cycle used when processing ``
[01:01:44] +   --> $DIR/issue-20772.rs:1:1
[01:01:44] +    |
[01:01:44] + LL | / trait T : Iterator<Item=Self::Item>
[01:01:44] + LL | | //~^ ERROR cycle detected
[01:01:44] + LL | | //~| ERROR associated type `Item` not found for `Self`
[01:01:44] + LL | | {}
[01:01:44] 11 
[01:01:44] 11 
[01:01:44] 12 error[E0220]: associated type `Item` not found for `Self`
[01:01:44] 
[01:01:44] 
[01:01:44] The actual stderr differed from the expected stderr.
[01:01:44] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20772/issue-20772.stderr
[01:01:44] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20772/issue-20772.stderr
[01:01:44] To update references, rerun the tests and pass the `--bless` flag
[01:01:44] To only update this specific test, also pass `--test-args issues/issue-20772.rs`
[01:01:44] error: 1 errors occurred comparing output.
[01:01:44] status: exit code: 1
[01:01:44] status: exit code: 1
[01:01:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-20772.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20772/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20772/auxiliary" "-A" "unused"
[01:01:44] ------------------------------------------
[01:01:44] 
[01:01:44] 
[01:01:44] ---------xt":"//~^ ERROR cycle detected","highlight_start":1,"highlight_end":26},{"text":"//~| ERROR associated type `Item` not found for `Self`","highlight_start":1,"highlight_end":55},{"text":"{}","highlight_start":1,"highlight_end":3}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when computing the supertraits of `T`\n  --> /checkout/src/test/ui/issues/issue-20772.rs:1:1\n   |\nLL | / trait T : Iterator<Item=Self::Item>\nLL | | //~^ ERROR cycle detected\nLL | | //~| ERROR associated type `Item` not found for `Self`\nLL | | {}\n   | |__^\n   |\n   = note: ...which again requires computing the supertraits of `T`, completing the cycle\nnote: cycle used when processing ``\n  --> /checkout/src/test/ui/issues/issue-20772.rs:1:1\n   |\nLL | / trait T : Iterator<Item=Self::Item>\nLL | | //~^ ERROR cycle detected\nLL | | //~| ERROR associated type `Item` not found for `Self`\nLL | | {}\n   | |__^\n\n"}
[01:01:44] {"message":"associated type `Item` not found for `Self`","code":{"code":"E0220","explanation":"\nYou used an associated type which isn't defined in the trait.\nErroneous code example:\n\n