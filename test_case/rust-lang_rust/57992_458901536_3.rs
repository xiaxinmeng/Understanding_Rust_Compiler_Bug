\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass/async-await.rs","byte_start":218,"byte_end":228,"line_start":12,"line_end":12,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    LocalWaker, Poll, Wake,","highlight_start":5,"highlight_end":15}],"label":"no `LocalWaker` in `task`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/run-pass/async-await.rs","byte_start":236,"byte_end":240,"line_start":12,"line_end":12,"column_start":23,"column_end":27,"is_primary":true,"text":[{"text":"    LocalWaker, Poll, Wake,","highlight_start":23,"highlight_end":27}],"label":"no `Wake` in `task`. Did you mean to use `Waker`?","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/run-pass/async-await.rs","byte_start":246,"byte_end":271,"line_start":13,"line_end":13,"column_start":5,"column_end":30,"is_primary":true,"text":[{"text":"    local_waker_from_nonlocal,","highlight_start":5,"highlight_end":30}],"label":"no `local_waker_from_nonlocal` in `task`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0432]: unresolved imports `std::task::LocalWaker`, `std::task::Wake`, `std::task::local_waker_from_nonlocal`\n  --> /checkout/src/test/run-pass/async-await.rs:12:5\n   |\nLL |     LocalWaker, Poll, Wake,\n   |     ^^^^^^^^^^        ^^^^ no `Wake` in `task`. Did you mean to use `Waker`?\n   |     |\n   |     no `LocalWaker` in `task`\nLL |     local_waker_from_nonlocal,\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^ no `local_waker_from_nonlocal` in `task`\n\n"}
[01:15:39] {"message":"For more information about this error, try `rustc --explain E0432`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0432`.\n"}
[01:15:39] 
[01:15:39] ------------------------------------------
[01:15:39] 
[01:15:39] 
[01:15:39] thread '[run-pass] run-pass/async-await.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3291:9
[01:15:39] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:15:39] 
[01:15:39] ---- [run-pass] run-pass/futures-api.rs stdout ----
[01:15:39] 
[01:15:39] error: test compilation failed although it shouldn't!
[01:15:39] status: exit code: 1
[01:15:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/futures-api.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/futures-api/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/futures-api/auxiliary"
[01:15:39] ------------------------------------------
[01:15:39] 
[01:15:39] ------------------------------------------
[01:15:39] stderr:
[01:15:39] stderr:
[01:15:39] ------------------------------------------
[01:15:39] {"message":"unresolved imports `std::task::Wake`, `std::task::LocalWaker`, `std::task::local_waker`, `std::task::local_waker_from_nonlocal`","code":{"code":"E0432","explanation":"\nAn import was unresolved.\n\nErroneous code example:\n\n