plain
---- [ui] src/test/ui/invalid/invalid-debugger-visualizer-option.rs stdout ----
diff of stderr:

8    = note: OR
9    = note: expected: `gdb_script_file = "..."`
- error: The system cannot find the file specified. (os error 2)
+ error: No such file or directory (os error 2)
12   --> $DIR/invalid-debugger-visualizer-option.rs:3:24
13    |
13    |
14 LL | #![debugger_visualizer(natvis_file = "../foo.random")]
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/invalid/invalid-debugger-visualizer-option/invalid-debugger-visualizer-option.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/invalid/invalid-debugger-visualizer-option/invalid-debugger-visualizer-option.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args invalid/invalid-debugger-visualizer-option.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/invalid/invalid-debugger-visualizer-option.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/invalid/invalid-debugger-visualizer-option" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/invalid/invalid-debugger-visualizer-option/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/invalid/invalid-debugger-visualizer-option.rs:2:24
   |
   |
LL | #![debugger_visualizer(random_file = "../foo.random")] //~ ERROR invalid argument
   |
   |
   = note: expected: `natvis_file = "..."`
   = note: OR
   = note: expected: `gdb_script_file = "..."`
error: No such file or directory (os error 2)
  --> /checkout/src/test/ui/invalid/invalid-debugger-visualizer-option.rs:3:24
   |
   |
LL | #![debugger_visualizer(natvis_file = "../foo.random")] //~ ERROR

error: aborting due to 2 previous errors
------------------------------------------

