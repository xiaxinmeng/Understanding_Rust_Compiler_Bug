plain
[00:43:47] ....................................................................................................
[00:43:52] ....................................................................................................
[00:43:57] ....................................................................................i...............
[00:44:03] .............................................................i......................................
[00:44:07] ................................................................F...................................
[00:44:13] .....................F..............................................................................
[00:44:20] ..........................................................................................i......F..
[00:44:23] ........iiiiiiiii...................................................
[00:44:23] 
[00:44:23] ---- [ui] ui/resolve/token-error-correct-3.rs stdout ----
[00:44:23] diff of stderr:
[00:44:23] 
[00:44:23] 
[00:44:23] 10 LL |             callback(path.as_ref(); //~ ERROR expected one of
[00:44:23] 12 
[00:44:23] 12 
[00:44:23] - error: expected one of `,`, `.`, `?`, or an operator, found `;`
[00:44:23] + error: expected one of `)`, `,`, `.`, `?`, or an operator, found `;`
[00:44:23] 14   --> $DIR/token-error-correct-3.rs:24:35
[00:44:23] 15    |
[00:44:23] 16 LL |             callback(path.as_ref(); //~ ERROR expected one of
[00:44:23] 
[00:44:23] -    |                                   ^ expected one of `,`, `.`, `?`, or an operator here
[00:44:23] +    |                                   ^ expected one of `)`, `,`, `.`, `?`, or an operator here
[00:44:23] 18 
[00:44:23] 19 error: expected one of `.`, `;`, `?`, `}`, or an operator, found `)`
[00:44:23] 20   --> $DIR/token-error-correct-3.rs:30:9
[00:44:23] 
[00:44:23] The actual stderr differed from the expected stderr.
[00:44:23] The actual stderr differed from the expected stderr.
[00:44:23] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/token-error-correct-3/token-error-correct-3.stderr
[00:44:23] To update references, rerun the tests and pass the `--bless` flag
[00:44:23] To only update this specific test, also pass `--test-args resolve/token-error-correct-3.rs`
[00:44:23] error: 1 errors occurred comparing output.
[00:44:23] status: exit code: 101
[00:44:23] status: exit code: 101
[00:44:23] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/token-error-correct-3.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/token-error-correct-3/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/token-error-correct-3/auxiliary" "-A" "unused"
[00:44:23] ------------------------------------------
[00:44:23] 
[00:44:23] ------------------------------------------
[00:44:23] stderr:
[00:44:23] stderr:
[00:44:23] ------------------------------------------
[00:44:23] {"message":"incorrect close delimiter: `}`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/resolve/token-error-correct-3.rs","byte_start":1361,"byte_end":1362,"line_start":30,"line_end":30,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        } else { //~ ERROR: incorrect close delimiter: `}`","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"unclosed delimiter","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/resolve/token-error-correct-3.rs","byte_start":1018,"byte_end":1019,"line_start":24,"line_end":24,"column_start":21,"column_end":22,"is_primary":true,"text":[{"text":"            callback(path.as_ref(); //~ ERROR expected one of","highlight_start":21,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: incorrect close delimiter: `}`\n  --> /checkout/src/test/ui/resolve/token-error-correct-3.rs:30:9\n   |\nLL |         } else { //~ ERROR: incorrect close delimiter: `}`\n   |         ^\n   |\nnote: unclosed delimiter\n  --> /checkout/src/test/ui/resolve/token-error-correct-3.rs:24:21\n   |\nLL |             callback(path.as_ref(); //~ ERROR expected one of\n   |                     ^\n\n"}
[00:44:23] {"message":"expected one of `)`, `,`, `.`, `?`, or an operator, found `;`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/resolve/token-error-correct-3.rs","byte_start":1032,"byte_end":1033,"line_start":24,"line_end":24,"column_start":35,"column_end":36,"is_primary":true,"text":[{"text":"            callback(path.as_ref(); //~ ERROR expected one of","highlight_start":35,"highlight_end":36}],"label":"expected one of `)`, `,`, `.`, `?`, or an operator here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected one of `)`, `,`, `.`, `?`, or an operator, found `;`\n  --> /checkout/src/test/ui/resolve/token-error-correct-3.rs:24:35\n   |\nLL |             callback(path.as_ref(); //~ ERROR expected one of\n   |                                   ^ expected one of `)`, `,`, `.`, `?`, or an operator here\n\n"}
[00:44:23] {"message":"expected one of `.`, `;`, `?`, `}`, or an operator, found `)`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/resolve/token-error-correct-3.rs","byte_start":1120,"byte_end":1120,"line_start":25,"line_end":25,"column_start":61,"column_end":61,"is_primary":false,"text":[{"text":"            fs::create_dir_all(path.as_ref()).map(|()| true) //~ ERROR: mismatched types","highlight_start":61,"highlight_end":61}],"label":"expected one of `.`, `;`, `?`, `}`, or an operator here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/resolve/token-error-correct-3.rs","byte_start":1361,"byte_end":1362,"line_start":30,"line_end":30,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        } else { //~ ERROR: incorrect close delimiter: `}`","highlight_start":9,"highlight_end":10}],"label":"unexpected token","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected one of `.`, `;`, `?`, `}`, or an operator, found `)`\n  --> /checkout/src/test/ui/resolve/token-err     fs::create_dir_all(path.as_ref()).map(|()| true) //~ ERROR: mismatched types","highlight_start":13,"highlight_end":61}],"label":"expected (), found enum `std::result::Result`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected type `()`\n   found type `std::result::Result<bool, std::io::Error>`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try adding a semicolon","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/resolve/token-error-correct-3.rs","byte_start":1120,"byte_end":1120,"line_start":25,"line_end":25,"column_start":61,"column_end":61,"is_primary":true,"text":[{"text":"            fs::create_dir_all(path.as_ref()).map(|()| true) //~ ERROR: mismatched types","highlight_start":61,"highlight_end":61}],"label":null,"suggested_replacement":";","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched types\n  --> /checkout/src/test/ui/resolve/token-error-correct-3.rs:25:13\n   |\nLL |             fs::create_dir_all(path.as_ref()).map(|()| true) //~ ERROR: mismatched types\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^- help: try adding a semicolon: `;`\n   |             |\n   |             expected (), found enum `std::result::Result`\n   |\n   = note: expected type `()`\n              found type `std::result::Result<bool, std::io::Error>`\n\n"}
[00:44:23] {"message":"aborting due to 5 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 5 previous errors\n\n"}
[00:44:23] {"message":"Some errors occurred: E0308, E0425.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0308, E0425.\n"}
[00:44:23] {"message":"For more information about an error, try `rustc --explain E0308`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0308`.\n"}
[00:44:23] ------------------------------------------
[00:44:23] 
[00:44:23] thread '[ui] ui/resolve/token-error-correct-3.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
[00:44:23] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:44:23] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:44:23] 
[00:44:23] ---- [ui] ui/similar-tokens.rs stdout ----
[00:44:23] diff of stderr:
[00:44:23] 
[00:44:23] - error: expected one of `,`, `::`, or `as`, found `.`
[00:44:23] + error: expected one of `,`, `::`, `as`, or `}`, found `.`
[00:44:23] 3    |
[00:44:23] 3    |
[00:44:23] 4 LL | use x::{A. B}; //~ ERROR expected one of `,`, `::`, or `as`, found `.`
[00:44:23] 
[00:44:23] -    |          ^ expected one of `,`, `::`, or `as` here
[00:44:23] +    |          ^ expected one of `,`, `::`, `as`, or `}` here
[00:44:23] 7 error: aborting due to previous error
[00:44:23] 8 
[00:44:23] 
[00:44:23] 
[00:44:23] 
[00:44:23] The actual stderr differed from the expected stderr.
[00:44:23] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/similar-tokens/similar-tokens.stderr
[00:44:23] To update references, rerun the tests and pass the `--bless` flag
[00:44:23] To only update this specific test, also pass `--test-args similar-tokens.rs`
[00:44:23] error: 1 errors occurred comparing output.
[00:44:23] status: exit code: 101
[00:44:23] status: exit code: 101
[00:44:23] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/similar-tokens.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/similar-tokens/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/similar-tokens/auxiliary" "-A" "unused"
[00:44:23] ------------------------------------------
[00:44:23] 
[00:44:23] ------------------------------------------
[00:44:23] stderr:
[00:44:23] stderr:
[00:44:23] ------------------------------------------
[00:44:23] {"message":"expected one of `,`, `::`, `as`, or `}`, found `.`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/similar-tokens.rs","byte_start":595,"byte_end":596,"line_start":17,"line_end":17,"column_start":10,"column_end":11,"is_primary":true,"text":[{"text":"use x::{A. B}; //~ ERROR expected one of `,`, `::`, or `as`, found `.`","highlight_start":10,"highlight_end":11}],"label":"expected one of `,`, `::`, `as`, or `}` here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected one of `,`, `::`, `as`, or `}`, found `.`\n  --> /checkout/src/test/ui/similar-tokens.rs:17:10\n   |\nLL | use x::{A. B}; //~ ERROR expected one of `,`, `::`, or `as`, found `.`\n   |      rror: 1 errors occurred comparing output.
[00:44:23] status: exit code: 101
[00:44:23] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/token/issue-10636-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/token/issue-10636-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/token/issue-10636-2/auxiliary" "-A" "unused"
[00:44:23] ------------------------------------------
[00:44:23] 
[00:44:23] ------------------------------------------
[00:44:23] stderr:
[00:44:23] stderr:
[00:44:23] ------------------------------------------
[00:44:23] {"message":"incorrect close delimiter: `}`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/token/issue-10636-2.rs","byte_start":763,"byte_end":764,"line_start":18,"line_end":18,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"} //~ ERROR: incorrect close delimiter","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"unclosed delimiter","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/token/issue-10636-2.rs","byte_start":696,"byte_end":697,"line_start":15,"line_end":15,"column_start":15,"column_end":16,"is_primary":true,"text":[{"text":"    option.map(|some| 42;","highlight_start":15,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: incorrect close delimiter: `}`\n  --> /checkout/src/test/ui/token/issue-10636-2.rs:18:1\n   |\nLL | } //~ ERROR: incorrect close delimiter\n   | ^\n   |\nnote: unclosed delimiter\n  --> /checkout/src/test/ui/token/issue-10636-2.rs:15:15\n   |\nLL |     option.map(|some| 42;\n   |               ^\n\n"}
[00:44:23] {"message":"expected one of `)`, `,`, `.`, `?`, or an operator, found `;`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/token/issue-10636-2.rs","byte_start":706,"byte_end":707,"line_start":15,"line_end":15,"column_start":25,"column_end":26,"is_primary":true,"text":[{"text":"    option.map(|some| 42;","highlight_start":25,"highlight_end":26}],"label":"expected one of `)`, `,`, `.`, `?`, or an operator here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected one of `)`, `,`, `.`, `?`, or an operator, found `;`\n  --> /checkout/src/test/ui/token/issue-10636-2.rs:15:25\n   |\nLL |     option.map(|some| 42;\n   |                         ^ expected one of `)`, `,`, `.`, `?`, or an operator here\n\n"}
[00:44:23] {"message":"expected expression, found `)`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/token/issue-10636-2.rs","byte_start":763,"byte_end":764,"line_start":18,"line_end":18,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"} //~ ERROR: incorrect close delimiter","highlight_start":1,"highlight_end":2}],"label":"expected expression","suggested_replacement":null,"./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
103608 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
103236 ./obj/build/bootstrap/debug/incremental/bootstrap-c730863262pt
103236 ./obj/build/bootstrap/debug/incremental/bootstrap-c730863262pt
103232 ./obj/build/bootstrap/debug/incremental/bootstrap-c730863262pt/s-f1ernfntbj-agyhfi-17uifkj4ft9sj
92312 ./obj/build/x86_64-unknown-linux-gnu/stage1
92288 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
90764 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
89796 ./src/llvm/test/CodeGen
