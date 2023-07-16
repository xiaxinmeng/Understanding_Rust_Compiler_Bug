plain

---- [ui] src/test/ui/resolve/bad-module.rs stdout ----
diff of stderr:

10 LL |     let foo = foo::bar::baz();
12 
- error: aborting due to 2 previous errors
- error: aborting due to 2 previous errors
+ error[E0434]: `foo` is not a crate or module, maybe you meant to call instance method
+    |
+    |
+ LL |     let foo = foo::bar::baz();
+    |
+ warning: ident is defined at here
+   --> $DIR/bad-module.rs:2:9
+    |
+    |
+ LL |     let foo = thing::len(Vec::new());
14 
- For more information about this error, try `rustc --explain E0433`.
+ error: aborting due to 3 previous errors
+ 
---
To only update this specific test, also pass `--test-args resolve/bad-module.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/bad-module.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/bad-module" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/bad-module/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0433]: failed to resolve: use of undeclared crate or module `thing`
   |
   |
LL |     let foo = thing::len(Vec::new());
   |               ^^^^^ use of undeclared crate or module `thing`
error[E0433]: failed to resolve: use of undeclared crate or module `foo`
  --> /checkout/src/test/ui/resolve/bad-module.rs:5:15
   |
   |
LL |     let foo = foo::bar::baz();


error[E0434]: `foo` is not a crate or module, maybe you meant to call instance method
   |
   |
LL |     let foo = foo::bar::baz();
   |
warning: ident is defined at here
  --> /checkout/src/test/ui/resolve/bad-module.rs:2:9
   |
   |
LL |     let foo = thing::len(Vec::new());

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0433, E0434.
---
---- [ui] src/test/ui/resolve/issue-101749.rs stdout ----
diff of stderr:

3    |
4 LL |     println!("{}", rect1::area());
5    |                    ^^^^^ use of undeclared crate or module `rect1`
+ 
+ error[E0434]: need to fix call instance method: area#0
6    |
6    |
- help: `rect1` is not a crate or module, maybe you meant to call instance method
+ LL |     println!("{}", rect1::area());
8    |
8    |
- LL |     println!("{}", rect1.area());
+ warning: ident is defined at here
+   --> $DIR/issue-101749.rs:18:9
+    |
+    |
+ LL |     let rect1 = Rectangle::new(width, height);
11 
- error: aborting due to previous error
+ error: aborting due to 2 previous errors
13 
---
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-101749/issue-101749.stderr
diff of fixed:

16     let width = 3;
17     let height = 4;
18     let rect1 = Rectangle::new(width, height);
-     println!("{}", rect1.area());
+     println!("{}", rect1::area());
20     //~^ ERROR failed to resolve: use of undeclared crate or module `rect1`
22 


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-101749/issue-101749.fixed
To only update this specific test, also pass `--test-args resolve/issue-101749.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/issue-101749.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-101749" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-101749/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0433]: failed to resolve: use of undeclared crate or module `rect1`
   |
   |
LL |     println!("{}", rect1::area());
   |                    ^^^^^ use of undeclared crate or module `rect1`

error[E0434]: need to fix call instance method: area#0
   |
   |
LL |     println!("{}", rect1::area());
   |
warning: ident is defined at here
  --> /checkout/src/test/ui/resolve/issue-101749.rs:18:9
   |
   |
LL |     let rect1 = Rectangle::new(width, height);

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0433, E0434.
