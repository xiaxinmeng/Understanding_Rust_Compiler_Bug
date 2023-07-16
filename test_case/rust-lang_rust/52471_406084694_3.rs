\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/E0508.rs","byte_start":543,"byte_end":551,"line_start":15,"line_end":15,"column_start":18,"column_end":26,"is_primary":true,"text":[{"text":"    let _value = array[0];  //~ ERROR [E0508]","highlight_start":18,"highlight_end":26}],"label":"cannot move out of here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using a reference instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/E0508.rs","byte_start":543,"byte_end":551,"line_start":15,"line_end":15,"column_start":18,"column_end":26,"is_primary":true,"text":[{"text":"    let _value = array[0];  //~ ERROR [E0508]","highlight_start":18,"highlight_end":26}],"label":null,"suggested_replacement":"&array[0]","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0508]: cannot move out of type `[NonCopy; 1]`, a non-copy array\n  --> /checkout/src/test/ui/E0508.rs:15:18\n   |\nLL |     let _value = array[0];  //~ ERROR [E0508]\n   |                  ^^^^^^^^\n   |                  |\n   |                  cannot move out of here\n   |                  help: consider using a reference instead: `&array[0]`\n\n"}
[00:50:15] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:50:15] {"message":"For more information about this error, try `rustc --explain E0508`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0508`.\n"}
[00:50:15] fatal runtime error: failed to initiate panic, error 5
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] thread '[ui] ui/E0508.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:50:15] 
[00:50:15] 
[00:50:15] ---- [ui] ui/E0663.rs stdout ----
[00:50:15] 
[00:50:15] error: Error: expected failure status (Some(101)) but received status None.
[00:50:15] status: signal: 6
[00:50:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/E0663.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/E0663/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/E0663/auxiliary" "-A" "unused"
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] ------------------------------------------
[00:50:15] stderr:
[00:50:15] stderr:
[00:50:15] ------------------------------------------
[00:50:15] {"message":"input operand constraint contains '+'","code":{"code":"E0663","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/E0663.rs","byte_start":563,"byte_end":570,"line_start":18,"line_end":18,"column_start":12,"column_end":19,"is_primary":true,"text":[{"text":"         : \"+test\"(\"a\") //~ ERROR E0663","highlight_start":12,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0663]: input operand constraint contains '+'\n  --> /checkout/src/test/ui/E0663.rs:18:12\n   |\nLL |          : \"+test\"(\"a\") //~ ERROR E0663\n   |            ^^^^^^^\n\n"}
[00:50:15] fatal runtime error: failed to initiate panic, error 5
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] thread '[ui] ui/E0663.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:50:15] 
[00:50:15] 
[00:50:15] ---- [ui] ui/E0665.rs stdout ----
[00:50:15] 
[00:50:15] error: Error: expected failure status (Some(101)) but received status None.
[00:50:15] status: signal: 6
[00:50:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/E0665.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/E0665/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/E0665/auxiliary" "-A" "unused"
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] ------------------------------------------
[00:50:15] stderr:
[00:50:15] stderr:
[00:50:15] ------------------------------------------
[00:50:15] {"message":"`Default` cannot be derived for enums, only structs","code":{"code":"E0665","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/E0665.rs","byte_start":494,"byte_end":501,"line_start":13,"line_end":13,"column_start":10,"column_end":17,"is_primary":true,"text":[{"text":"#[derive(Default)] //~ ERROR E0665","highlight_start":10,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/E0665.rs","byte_start":494,"byte_end":501,"line_start":13,"line_end":13,"column_start":10,"column_end":17,"is_primary":false,"text":[{"text":"#[derive(Default)] //~ ERROR E0665","highlight_start":10,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(Default)]","def_site_span":null}}],"children":[],"rendered":"error[E0665]: `Default` cannot be derived for enums, only structs\n  --> /checkout/src/test/ui/E0665.rs:13:10\n   |\nLL | #[derive(Default)] //~ ERROR E0665\n   |          ^^^^^^^\n\n"}
[00:50:15] fatal runtime error: failed to initiate panic, error 5
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] thread '[ui] ui/E0665.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:50:15] 
[00:50:15] 
[00:50:15] ---- [ui] ui/E0664.rs stdout ----
[00:50:15] 
[00:50:15] error: Error: expected failure status (Some(101)) but received status None.
[00:50:15] status: signal: 6
[00:50:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/E0664.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/E0664/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/E0664/auxiliary" "-A" "unused"
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] ------------------------------------------
[00:50:15] stderr:
[00:50:15] stderr:
[00:50:15] ------------------------------------------
[00:50:15] {"message":"clobber should not be surrounded by braces","code":{"code":"E0664","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/E0664.rs","byte_start":577,"byte_end":584,"line_start":19,"line_end":19,"column_start":12,"column_end":19,"is_primary":true,"text":[{"text":"         : \"{eax}\" //~ ERROR E0664","highlight_start":12,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0664]: clobber should not be surrounded by braces\n  --> /checkout/src/test/ui/E0664.rs:19:12\n   |\nLL |          : \"{eax}\" //~ ERROR E0664\n   |            ^^^^^^^\n\n"}
[00:50:15] fatal runtime error: failed to initiate panic, error 5
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] thread '[ui] ui/E0664.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:50:15] 
[00:50:15] 
[00:50:15] ---- [ui] ui/allocator-submodule.rs stdout ----
[00:50:15] 
[00:50:15] error: Error: expected failure status (Some(101)) but received status None.
[00:50:15] status: signal: 6
[00:50:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/allocator-submodule.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator-submodule/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator-submodule/auxiliary" "-A" "unused"
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] ------------------------------------------
[00:50:15] stderr:
[00:50:15] stderr:
[00:50:15] ------------------------------------------
[00:50:15] {"message":"`global_allocator` cannot be used in submodules","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/allocator-submodule.rs","byte_start":981,"byte_end":1015,"line_start":37,"line_end":37,"column_start":5,"column_end":39,"is_primary":true,"text":[{"text":"    static MY_HEAP: MyAlloc = MyAlloc; //~ ERROR global_allocator","highlight_start":5,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `global_allocator` cannot be used in submodules\n  --> /checkout/src/test/ui/allocator-submodule.rs:37:5\n   |\nLL |     static MY_HEAP: MyAlloc = MyAlloc; //~ ERROR global_allocator\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:50:15] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:50:15] fatal runtime error: failed to initiate panic, error 5
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] thread '[ui] ui/allocator-submodule.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:50:15] 
[00:50:15] 
[00:50:15] ---- [ui] ui/array-break-length.rs stdout ----
[00:50:15] 
[00:50:15] error: Error: expected failure status (Some(101)) but received status None.
[00:50:15] status: signal: 6
[00:50:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/array-break-length.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-break-length/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-break-length/auxiliary" "-A" "unused"
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] ------------------------------------------
[00:50:15] stderr:
[00:50:15] stderr:
[00:50:15] ------------------------------------------
[00:50:15] {"message":"`break` outside of loop","code":{"code":"E0268","explanation":"\nThis error indicates the use of a loop keyword (`break` or `continue`) outside\nof a loop. Without a loop to break out of or continue in, no sensible action can\nbe taken. Erroneous code example:\n\n