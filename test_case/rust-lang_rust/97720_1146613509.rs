plain

---- [ui] src/test/ui-fulldeps/internal-lints/rustc_pass_by_value.rs stdout ----
diff of stderr:

94 LL |         reference: &CustomStruct,
95    |                    ^^^^^^^^^^^^^ help: try passing by value: `CustomStruct`
96 
- error: passing `CustomAlias<>` by reference
+ error: passing `CustomAlias<'_>` by reference
99    |
99    |
100 LL |         reference: &CustomAlias,

-    |                    ^^^^^^^^^^^^ help: try passing by value: `CustomAlias<>`
+    |                    ^^^^^^^^^^^^ help: try passing by value: `CustomAlias<'_>`
102 
103 error: passing `WithParameters<T, 1>` by reference


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/rustc_pass_by_value/rustc_pass_by_value.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/rustc_pass_by_value/rustc_pass_by_value.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args internal-lints/rustc_pass_by_value.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/rustc_pass_by_value.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/rustc_pass_by_value" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/rustc_pass_by_value/auxiliary"
stdout: none
--- stderr -------------------------------
error: passing `Ty<'_>` by reference
   |
   |
LL |     ty_ref: &Ty<'_>, //~ ERROR passing `Ty<'_>` by reference
   |             ^^^^^^^ help: try passing by value: `Ty<'_>`
note: the lint level is defined here
  --> /checkout/src/test/ui-fulldeps/internal-lints/rustc_pass_by_value.rs:5:9
   |
LL | #![deny(rustc::pass_by_value)]
LL | #![deny(rustc::pass_by_value)]
   |         ^^^^^^^^^^^^^^^^^^^^

error: passing `TyCtxt<'_>` by reference
  --> /checkout/src/test/ui-fulldeps/internal-lints/rustc_pass_by_value.rs:16:18
   |
LL |     ty_ctxt_ref: &TyCtxt<'_>, //~ ERROR passing `TyCtxt<'_>` by reference

error: passing `Ty<'_>` by reference
  --> /checkout/src/test/ui-fulldeps/internal-lints/rustc_pass_by_value.rs:20:28
   |
   |
LL | fn ty_multi_ref(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>) {}
   |                            ^^^^^^^ help: try passing by value: `Ty<'_>`
error: passing `TyCtxt<'_>` by reference
  --> /checkout/src/test/ui-fulldeps/internal-lints/rustc_pass_by_value.rs:20:55
   |
   |
LL | fn ty_multi_ref(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>) {}

error: passing `Ty<'_>` by reference
  --> /checkout/src/test/ui-fulldeps/internal-lints/rustc_pass_by_value.rs:27:17
   |
   |
LL |         ty_ref: &Ty<'_>, //~ ERROR passing `Ty<'_>` by reference
   |                 ^^^^^^^ help: try passing by value: `Ty<'_>`
error: passing `TyCtxt<'_>` by reference
  --> /checkout/src/test/ui-fulldeps/internal-lints/rustc_pass_by_value.rs:29:22
   |
   |
LL |         ty_ctxt_ref: &TyCtxt<'_>, //~ ERROR passing `TyCtxt<'_>` by reference

error: passing `Ty<'_>` by reference
  --> /checkout/src/test/ui-fulldeps/internal-lints/rustc_pass_by_value.rs:32:41
   |
   |
LL |     fn ty_multi_ref_in_trait(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>);
   |                                         ^^^^^^^ help: try passing by value: `Ty<'_>`
error: passing `TyCtxt<'_>` by reference
  --> /checkout/src/test/ui-fulldeps/internal-lints/rustc_pass_by_value.rs:32:68
   |
   |
LL |     fn ty_multi_ref_in_trait(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>);

error: passing `Ty<'_>` by reference
  --> /checkout/src/test/ui-fulldeps/internal-lints/rustc_pass_by_value.rs:54:17
   |
   |
LL |         ty_ref: &Ty<'_>, //~ ERROR passing `Ty<'_>` by reference
   |                 ^^^^^^^ help: try passing by value: `Ty<'_>`
error: passing `TyCtxt<'_>` by reference
  --> /checkout/src/test/ui-fulldeps/internal-lints/rustc_pass_by_value.rs:56:22
   |
   |
LL |         ty_ctxt_ref: &TyCtxt<'_>, //~ ERROR passing `TyCtxt<'_>` by reference

error: passing `Ty<'_>` by reference
  --> /checkout/src/test/ui-fulldeps/internal-lints/rustc_pass_by_value.rs:60:38
   |
   |
LL |     fn ty_multi_ref_assoc(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>) {}
   |                                      ^^^^^^^ help: try passing by value: `Ty<'_>`
error: passing `TyCtxt<'_>` by reference
  --> /checkout/src/test/ui-fulldeps/internal-lints/rustc_pass_by_value.rs:60:65
   |
   |
LL |     fn ty_multi_ref_assoc(ty_multi: &&Ty<'_>, ty_ctxt_multi: &&&&TyCtxt<'_>) {}


error: passing `CustomEnum` by reference
   |
   |
LL |         reference: &CustomEnum, //~ ERROR passing `CustomEnum` by reference
   |                    ^^^^^^^^^^^ help: try passing by value: `CustomEnum`

error: passing `CustomStruct` by reference
   |
   |
LL | type CustomAlias<'a> = &'a CustomStruct; //~ ERROR passing `CustomStruct` by reference
   |                        ^^^^^^^^^^^^^^^^ help: try passing by value: `CustomStruct`

error: passing `CustomStruct` by reference
   |
   |
LL |         reference: &CustomStruct, //~ ERROR passing `CustomStruct` by reference
   |                    ^^^^^^^^^^^^^ help: try passing by value: `CustomStruct`

error: passing `CustomAlias<'_>` by reference
   |
   |
LL |         reference: &CustomAlias, //~ ERROR passing `CustomAlias<>` by reference
   |                    ^^^^^^^^^^^^ help: try passing by value: `CustomAlias<'_>`

error: passing `WithParameters<T, 1>` by reference
   |
   |
LL |         reference: &'a WithParameters<T, 1>, //~ ERROR passing `WithParameters<T, 1>` by reference
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^ help: try passing by value: `WithParameters<T, 1>`

error: passing `WithParameters<T, 1, u32>` by reference
   |
   |
LL |         reference_with_m: &WithParameters<T, 1, u32>, //~ ERROR passing `WithParameters<T, 1, u32>` by reference
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try passing by value: `WithParameters<T, 1, u32>`

error: passing `WithParameters<T, 1>` by reference
   |
   |
LL |     ) -> &'a WithParameters<T, 1> {
   |          ^^^^^^^^^^^^^^^^^^^^^^^^ help: try passing by value: `WithParameters<T, 1>`

error: passing `WithParameters<_, 1>` by reference
   |
   |
LL |         reference as &WithParameters<_, 1> //~ ERROR passing `WithParameters<_, 1>` by reference
   |                      ^^^^^^^^^^^^^^^^^^^^^ help: try passing by value: `WithParameters<_, 1>`
error: aborting due to 20 previous errors
------------------------------------------


