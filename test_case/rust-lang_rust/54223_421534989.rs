plain
[00:57:51] ....................................................................................................
[00:57:54] ....................................................i...............................................
[00:57:56] ....................................................................................................
[00:57:59] ....................................................................................................
[00:58:02] iiiiiiiii...........................................................................................
[00:58:08] ....................................................................................................
[00:58:12] .................................................................................i..................
[00:58:14] ....................................................................................................
[00:58:17] ...................................i.i..ii..........................................................
---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:46] 
[01:07:46] running 97 tests
[01:09:40] .FF.FFF.FFF..FF.FF.....FFFF......F.FF.FF..................F.test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
no-std-not-supported.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/derive-no-std-not-supported/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/derive-no-std-not-supported/auxiliary"
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] ------------------------------------------
[01:11:42] stderr:
[01:11:42] stderr:
[01:11:42] ------------------------------------------
[01:11:42] {"message":"field is never used: `x`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/derive-no-std-not-supported.rs","byte_start":647,"byte_end":653,"line_start":23,"line_end":23,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    x: u32,","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: field is never used: `x`\n  --> /checkout/src/test/run-pass-fulldeps/derive-no-std-not-supported.rs:23:5\n   |\nLL |     x: u32,\n   |     ^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] thread '[run-pass] run-pass-fulldeps/derive-no-std-not-supported.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3237:9
[01:11:42] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:11:42] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:11:42] 
[01:11:42] ---- [run-pass] run-pass-fulldeps/ast_stmt_expr_attr.rs stdout ----
[01:11:42] normalized stderr:
[01:11:42] warning: unused import: `syntax::ast::*`
[01:11:42]   --> $DIR/ast_stmt_expr_attr.rs:17:5
[01:11:42]    |
[01:11:42] LL | use syntax::ast::*;
[01:11:42]    |
[01:11:42]    = note: #[warn(unused_imports)] on by default
[01:11:42] 
[01:11:42] warning: unused import: `syntax::parse`
[01:11:42] warning: unused import: `syntax::parse`
[01:11:42]   --> $DIR/ast_stmt_expr_attr.rs:21:5
[01:11:42]    |
[01:11:42] LL | use syntax::parse;
[01:11:42] 
[01:11:42] warning: unused import: `syntax::str::char_at`
[01:11:42]   --> $DIR/ast_stmt_expr_attr.rs:27:5
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL | use syntax::str::char_at;
[01:11:42] 
[01:11:42] warning: unused import: `syntax::parse::attr::*`
[01:11:42]   --> $DIR/ast_stmt_expr_attr.rs:28:5
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL | use syntax::parse::attr::*;
[01:11:42] 
[01:11:42] warning: unused import: `std::fmt`
[01:11:42]   --> $DIR/ast_stmt_expr_attr.rs:30:5
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL | use std::fmt;
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] The actual stderr differed from the expected stderr.
[01:11:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/ast_stmt_expr_attr/ast_stmt_expr_attr.stderr
[01:11:42] To update references, rerun the tests and pass the `--bless` flag
[01:11:42] To only update this specific test, also pass `--test-args ast_stmt_expr_attr.rs`
[01:11:42] error: 1 errors occurred comparing output.
[01:11:42] status: exit code: 0
[01:11:42] status: exit code: 0
[01:11:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/ast_stmt_expr_attr.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/ast_stmt_expr_attr/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/ast_stmt_expr_attr/auxiliary"
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] ------------------------------------------
[01:11:42] stderr:
[01:11:42] stderr:
[01:11:42] ------------------------------------------
[01:11:42] {"message":"unused import: `syntax::ast::*`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/ast_stmt_expr_attr.rs","byte_start":546,"byte_end":560,"line_start":17,"line_end":17,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"use syntax::ast::*;","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_imports)] on by default","code":null,"level":"note","spans":[],"cn   |     ^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:11:42] {"message":"unused import: `syntax::parse::attr::*`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/ast_stmt_expr_attr.rs","byte_start":871,"byte_end":893,"line_start":28,"line_end":28,"column_start":5,"column_end":27,"is_primary":true,"text":[{"text":"use syntax::parse::attr::*;","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: unused import: `syntax::parse::attr::*`\n  --> /checkout/src/test/run-pass-fulldeps/ast_stmt_expr_attr.rs:28:5\n   |\nLL | use syntax::parse::attr::*;\n   |     ^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:11:42] {"message":"unused import: `std::fmt`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/ast_stmt_expr_attr.rs","byte_start":926,"byte_end":934,"line_start":30,"line_end":30,"column_start":5,"column_end":13,"is_primary":true,"text":[{"text":"use std::fmt;","highlight_start":5,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: unused import: `std::fmt`\n  --> /checkout/src/test/run-pass-fulldeps/ast_stmt_expr_attr.rs:30:5\n   |\nLL | use std::fmt;\n   |     ^^^^^^^^\n\n"}
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] thread '[run-pass] run-pass-fulldeps/ast_stmt_expr_attr.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3237:9
[01:11:42] 
[01:11:42] 
[01:11:42] ---- [run-pass] run-pass-fulldeps/deriving-encodable-decodable-box.rs stdout ----
[01:11:42] normalized stderr:
[01:11:42] warning: derive(Decodable) is deprecated in favor of derive(RustcDecodable)
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL | #[derive(Encodable, Decodable)]
[01:11:42] 
[01:11:42] 
[01:11:42] warning: derive(Encodable) is deprecated in favor of derive(RustcEncodable)
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL | #[derive(Encodable, Decodable)]
[01:11:42] 
[01:11:42] 
[01:11:42] warning: unused imports: `Decodable`, `Encodable`
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL | use serialize::{Encodable, Decodable};
[01:11:42]    |
[01:11:42]    = note: #[warn(unused_imports)] on by default
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] The actual stderr differed from the expected stderr.
[01:11:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/deriving-encodable-decodable-box/deriving-encodable-decodable-box.stderr
[01:11:42] To update references, rerun the tests and pass the `--bless` flag
[01:11:42] To only update this specific test, also pass `--test-args deriving-encodable-decodable-box.rs`
[01:11:42] error: 1 errors occurred comparing output.
[01:11:42] status: exit code: 0
[01:11:42] status: exit code: 0
[01:11:42] command: "/checkout/obj/build/x86_64ncodable`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/deriving-encodable-decodable-box.rs","byte_start":561,"byte_end":570,"line_start":17,"line_end":17,"column_start":17,"column_end":26,"is_primary":true,"text":[{"text":"use serialize::{Encodable, Decodable};","highlight_start":17,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/run-pass-fulldeps/deriving-encodable-decodable-box.rs","byte_start":572,"byte_end":581,"line_start":17,"line_end":17,"column_start":28,"column_end":37,"is_primary":true,"text":[{"text":"use serialize::{Encodable, Decodable};","highlight_start":28,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_imports)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused imports: `Decodable`, `Encodable`\n  --> /checkout/src/test/run-pass-fulldeps/deriving-encodable-decodable-box.rs:17:17\n   |\nLL | use serialize::{Encodable, Decodable};\n   |                 ^^^^^^^^^  ^^^^^^^^^\n   |\n   = note: #[warn(unused_imports)] on by default\n\n"}
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] thread '[run-pass] run-pass-fulldeps/deriving-encodable-decodable-box.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3237:9
[01:11:42] 
[01:11:42] 
[01:11:42] ---- [run-pass] run-pass-fulldeps/custom-derive-partial-eq.rs stdout ----
[01:11:42] normalized stderr:
[01:11:42] warning: `#[derive]` for custom traits is deprecated and will be removed in the future. Prefer using procedural macro custom derive.
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL | #[derive(CustomPartialEq)] // Check that this is not a stability error.
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] The actual stderr differed from the expected stderr.
[01:11:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/custom-derive-partial-eq/custom-derive-partial-eq.stderr
[01:11:42] To update references, rerun the tests and pass the `--bless` flag
[01:11:42] To only update this specific test, also pass `--test-args custom-derive-partial-eq.rs`
[01:11:42] error: 1 errors occurred comparing output.
[01:11:42] status: exit code: 0
[01:11:42] status: exit code: 0
[01:11:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/custom-derive-partial-eq.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/custom-derive-partial-eq/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/custom-derive-partial-eq/auxiliary"
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] ------------------------------------------
[01:11:42] stderr:
[01:11:42] stderr:
[01:11:42] ------------------------------------------
[01:11:42] {"message":"`#[derive]` for custom traits is deprecated and will be removed in the future. Prefer using procedural macro custom derive.","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/custom-derive-partial-eq.rs","byte_start":625,"byte_end":640,"line_start":17,"line_end":17,"column_start":10,"column_end":25,"is_primary":true,"text":[{"text":"#[derive(CustomPartialEq)] // Check that this is not a stability error.","highlight_start":10,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: `#[derive]` for custom traits is deprecated and will be removed in the future. Prefer using procedural macro custom derive.\n  --> /checkout/src/test/run-pass-fulldeps/custom-derive-partial-eq.rs:17:10\n   |\nLL | #[derive(CustomPartialEq)] // Check that this is not a stability error.\n   |          ^^^^^^^^^^^^^^^\n\n"}
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] thread '[run-pass] run-pass-fulldeps/custom-derive-partial-eq.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3237:9
[01:11:42] 
[01:11:42] 
[01:11:42] ---- [run-pass] run-pass-fulldeps/deriving-global.rs stdout ----
[01:11:42] normalized stderr:
[01:11:42] warning: derive(Decodable) is deprecated in favor of derive(RustcDecodable)
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL |                Encodable, Decodable)]
[01:11:42] 
[01:11:42] 
[01:11:42] warning: derive(Encodable) is deprecated in favor of derive(RustcEncodable)
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL |                Encodable, Decodable)]
[01:11:42] 
[01:11:42] 
[01:11:42] warning: derive(Decodable) is deprecated in favor of derive(RustcDecodable)
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL |                Encodable, Decodable)]
[01:11:42] 
[01:11:42] 
[01:11:42] warning: derive(Encodable) is deprecated in favor of derive(RustcEncodable)
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL |                Encodable, Decodable)]
[01:11:42] 
[01:11:42] 
[01:11:42] warning: derive(Decodable) is deprecated in favor of derive(RustcDecodable)
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL |                Encodable, Decodable)]
[01:11:42] 
[01:11:42] 
[01:11:42] warning: derive(Encodable) is deprecated in favor of derive(RustcEncodable)
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL |                Encodable, Decodable)]
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] The actual stderr differed from the expected stderr.
[01:11:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/deriving-global/deriving-global.stderr
[01:11:42] To update references, rerun the tests and pass the `--bless` flag
[01:11:42] To only update this specific test, also pass `--test-args deriving-global.rs`
[01:11:42] error: 1 errors occurred comparing output.
[01:11:42] status: exit code: 0
[01:11:42] status: exit code: 0
[01:11:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/deriving-global.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/deriving-global/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/deriving-global/auxiliary"
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] ------------------------------------------
[01:11:42] stderr:
[01:11:42] stderr:
[01:11:42] ------------------------------------------
[01:11:42] {"message":"derive(Decodable) is deprecated in favor of derive(RustcDecodable)","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/deriving-global.rs","byte_start":868,"byte_end":877,"line_start":23,"line_end":23,"column_start":27,"column_end":36,"is_primary":true,"text":[{"text":"               Encodable, Decodable)]","highlight_start":27,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/run-pass-fulldeps/deriving-global.rs","byte_start":868,"byte_end":877,"line_start":23,"line_end":23,"column_start":27,"column_end":3":"               Encodable, Decodable)]","highlight_start":16,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/run-pass-fulldeps/deriving-global.rs","byte_start":1042,"byte_end":1051,"line_start":30,"line_end":30,"column_start":16,"column_end":25,"is_primary":false,"text":[{"text":"               Encodable, Decodable)]","highlight_start":16,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(Encodable)]","def_site_span":null}}],"children":[],"rendered":"warning: derive(Encodable) is deprecated in favor of derive(RustcEncodable)\n  --> /checkout/src/test/run-pass-fulldeps/deriving-global.rs:30:16\n   |\nLL |                Encodable, Decodable)]\n   |                ^^^^^^^^^\n\n"}
[01:11:42] {"message":"derive(Decodable) is deprecated in favor of derive(RustcDecodable)","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/deriving-global.rs","byte_start":1238,"byte_end":1247,"line_start":37,"line_end":37,"column_start":27,"column_end":36,"is_primary":true,"text":[{"text":"               Encodable, Decodable)]","highlight_start":27,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/run-pass-fulldeps/deriving-global.rs","byte_start":1238,"byte_end":1247,"line_start":37,"line_end":37,"column_start":27,"column_end":36,"is_primary":false,"text":[{"text":"               Encodable, Decodable)]","highlight_start":27n\n"}
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] thread '[run-pass] run-pass-fulldeps/deriving-global.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3237:9
[01:11:42] 
[01:11:42] 
[01:11:42] ---- [run-pass] run-pass-fulldeps/deriving-hygiene.rs stdout ----
[01:11:42] normalized stderr:
[01:11:42] warning: derive(Encodable) is deprecated in favor of derive(RustcEncodable)
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL | #[derive(Ord,Eq,PartialOrd,PartialEq,Debug,Decodable,Encodable,Hash)]
[01:11:42] 
[01:11:42] 
[01:11:42] warning: derive(Decodable) is deprecated in favor of derive(RustcDecodable)
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL | #[derive(Ord,Eq,PartialOrd,PartialEq,Debug,Decodable,Encodable,Hash)]
[01:11:42] 
[01:11:42] 
[01:11:42] warning: constant `other` should have an upper case name such as `OTHER`
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL | pub const other: u8 = 1;
[01:11:42]    |
[01:11:42]    = note: #[warn(non_upper_case_globals)] on by default
[01:11:42] 
[01:11:42] 
[01:11:42] warning: constant `f` should have an upper case name such as `F`
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL | pub const f: u8 = 1;
[01:11:42] 
[01:11:42] 
[01:11:42] warning: constant `d` should have an upper case name such as `D`
[01:1_64-unknown-linux-gnu/test/run-pass-fulldeps/deriving-hygiene/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/deriving-hygiene/auxiliary"
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] ------------------------------------------
[01:11:42] stderr:
[01:11:42] stderr:
[01:11:42] ------------------------------------------
[01:11:42] {"message":"derive(Encodable) is deprecated in favor of derive(RustcEncodable)","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/deriving-hygiene.rs","byte_start":709,"byte_end":718,"line_start":21,"line_end":21,"column_start":54,"column_end":63,"is_primary":true,"text":[{"text":"#[derive(Ord,Eq,PartialOrd,PartialEq,Debug,Decodable,Encodable,Hash)]","highlight_start":54,"highlight_end":63}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/run-pass-fulldeps/deriving-hygiene.rs","byte_start":709,"byte_end":718,"line_start":21,"line_end":21,"column_start":54,"column_end":63,"is_primary":false,"text":[{"text":"#[derive(Ord,Eq,PartialOrd,PartialEq,Debug,Decodable,Encodable,Hash)]","highlight_start":54,"highlight_end":63}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(Encodable)]","def_site_span":null}}],"children":[],"rendered":"warning: derive(Encodable) is deprecated in favor of derive(RustcEncodable)\n  --> /checkout/src/test/run-passs","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/deriving-hygiene.rs","byte_start":519,"byte_end":543,"line_start":14,"line_end":14,"column_start":1,"column_end":25,"is_primary":true,"text":[{"text":"pub const other: u8 = 1;","highlight_start":1,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(non_upper_case_globals)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: constant `other` should have an upper case name such as `OTHER`\n  --> /checkout/src/test/run-pass-fulldeps/deriving-hygiene.rs:14:1\n   |\nLL | pub const other: u8 = 1;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(non_upper_case_globals)] on by default\n\n"}
[01:11:42] {"message":"constant `f` should have an upper case name such as `F`","code":{"code":"non_upper_case_globals","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/deriving-hygiene.rs","byte_start":544,"byte_end":564,"line_start":15,"line_end":15,"column_start":1,"column_end":21,"is_primary":true,"text":[{"text":"pub const f: u8 = 1;","highlight_start":1,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: constant `f` should have an upper case name such as `F`\n  --> /checkout/src/test/run-pass-fulldeps/deriving-hygiene.rs:15:1\n   |\nLL | pub const f: u8 = 1;\n   | ^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:11:42] {"message":"constant `d` should have an upper case name such as `D`","code":{"code":"non_upper_case_globals","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/deriving-hygiene.rs","byte_start":565,"byte_end":585,"line_start":16,"line_end":16,"column_start":1,"column_end":21,"is_primary":true,"text":[{"text":"pub const d: u8 = 1;","highlight_start":1,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: constant `d` should have an upper case name such as `D`\n  --> /checkout/src/test/run-pass-fulldeps/deriving-hygiene.rs:16:1\n   |\nLL | pub const d: u8 = 1;\n   | ^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:11:42] {"message":"constant `s` should have an upper case name such as `S`","code":{"code":"non_upper_case_globals","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/deriving-hygiene.rs","byte_start":586,"byte_end":606,"line_start":17,"line_end":17,"column_start":1,"column_end":21,"is_primary":true,"text":[{"text":"pub const s: u8 = 1;","highlight_start":1,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: constant `s` should have an upper case name such as `S`\n  --> /checkout/src/test/run-pass-fulldeps/deriving-hygiene.rs:17:1\n   |\nLL | pub const s: u8 = 1;\n   | ^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:11:42] {"message":"constant `state` should have an upper case name such as `STATE`","code":{"code":"non_upper_case_globals","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/deriving-hygiene.rs","byte_start":607,"byte_end":631,"line_start":18,"line_end":18,"column_start":1,"column_end":25,"is_primary":true,"text":[{"text":"pub const state: u8 = 1;","highlight_start":1,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: constant `state` should have an upper case name such as `STATE`\n  --> /checkout/src/test/run-pass-fulldeps/deriving-hygiene.rs:18:1\n   |\nLL | pub const state: u8 = 1;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:11:42] {"message":"constant `cmp` should have an upper case name such as `CMP`","code":{"code":"non_upper_case_globals","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/deriving-hygiene.rs","byte_start":632,"byte_end":654,"line_start":19,"line_end":19,"column_start":1,"column_end":23,"is_primary":true,"text":[{"text":"pub const cmp: u8 = 1;","highlight_start":1,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: constant `cmp` should have an upper case name such as `CMP`\n  --> /checkout/src/test/run-pass-fulldeps/deriving-hygiene.rs:19:1\n   |\nLL | pub const cmp: u8 = 1;\n   | ^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] thread '[run-pass] run-pass-fulldeps/deriving-hygiene.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3237:9
[01:11:42] 
[01:11:42] 
[01:11:42] ---- [run-pass] run-pass-fulldeps/deriving-encodable-decodable-cell-refcell.rs stdout ----
[01:11:42] normalized stderr:
[01:11:42] warning: derive(Decodable) is deprecated in favor of derive(RustcDecodable)
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL | #[derive(Encodable, Decodable)]
[01:11:42] 
[01:11:42] 
[01:11:42] warning: derive(Encodable) is deprecated in favor of derive(RustcEncodable)
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL | #[derive(Encodable, Decodable)]
[01:11:42] 
[01:11:42] 
[01:11:42] warning: derive(Decodable) is deprecated in favor of derive(RustcDecodable)
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL | #[derive(Encodable, Decodable)]
[01:11:42] 
[01:11:42] 
[01:11:42] warning: derive(Encodable) is deprecated in favor of derive(RustcEncodable)
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL | #[derive(Encodable, Decodable)]
[01:11:42] 
[01:11:42] 
[01:11:42] warning: unused imports: `Decodable`, `Encodable`
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL | use serialize::{Encodable, Decodable};
[01:11:42]    |
[01:11:42]    = note: #[warn(unused_imports)] on by default
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] The actual stderr differed from the expected stderr.
[01:11:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/deriving-encodable-decodable-cell-refcell/deriving-encodable-decodable-cell-refcell.stderr
[01:11:42] To update references, rerun the tests and pass the `--bless` flag
[01:11:42] To only update this specific test, also pass `--test-args deriving-encodable-decodable-cell-refcell.rs`
[01:11:42] error: 1 errors occurred comparing output.
[01:11:42] status: exit code: 0
[01:11:42] status: exit code: 0
[01:11:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/deriving-encodable-decodable-cell-refcell.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/deriving-encodable-decodable-cell-refcell/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/deriving-encodable-decodable-cell-refcell/auxiliary"
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] ------------------------------------------
[01:11:42] stderr:
[01:11:42] stderr:
[01:11:42] ------------------------------------------
[01:11:42] {"message":"derive(Decodable) is deprecated in favor of derive(RustcDecodable)","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/deriving-encodable-decodable-cell-refcell.rs","byte_start":789,"byte_end":798,"line_start":23,"line_end":23,"column_start":21,"column_end":30,"iolumn_end":19,"is_primary":false,"text":[{"text":"#[derive(Encodable, Decodable)]","highlight_start":10,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(Encodable)]","def_site_span":null}}],"children":[],"rendered":"warning: derive(Encodable) is deprecated in favor of derive(RustcEncodable)\n  --> /checkout/src/test/run-pass-fulldeps/deriving-encodable-decodable-cell-refcell.rs:23:10\n   |\nLL | #[derive(Encodable, Decodable)]\n   |          ^^^^^^^^^\n\n"}
[01:11:42] {"message":"derive(Decodable) is deprecated in favor of derive(RustcDecodable)","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/deriving-encodable-decodable-cell-refcell.rs","byte_start":850,"byte_end":859,"line_start":28,"line_end":28,"column_start":21,"column_end":30,"is_primary":true,"text":[{"text":"#[derive(Encodable, Decodable)]","highlight_start":21,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/run-pass-fulldeps/deriving-encodable-decodable-cell-refcell.rs","byte_start":850,"byte_end":859,"line_start":28,"line_end":28,"column_start":21,"column_end":30,"is_primary":false,"text":[{"text":"#[derive(Encodable, Decodable)]","highlight_start":21,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(Decodable)]","def_site_span":null}}],"children":[],"rendered":"warning: derive(Decodable) is deprecated in favor of derive(RustcDecodable)\n  --> /checkout/src/test/run-pass-fulldeps/deriving-encodable-decodable-cell-refcell.rs:28:21\n   |\nLL | #[derive(Encodable, Decodable)]\n   |                     ^^^^^^^^^\n\n"}
[01:11:42] {"message":"derive(Encodable) is deprecated in favor of derive(RustcEncodable)","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/deriving-encodable-decodable-cell-refcell.rs","byte_start":839,"byte_end":848,"line_start":28,"line_end":28,"column_start":10,"column_end":19,"is_primary":true,"text":[{"text":"#[derive(Encodable, Decodable)]","highlight_start":10,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/run-pass-fulldeps/deriving-encodable-decodable-cell-refcell.rs","byte_start":839,"byte_end":848,"line_start":28,"line_end":28,"column_start":10,"column_end":19,"is_primary":false,"text":[{"text":"#[derive(Encodable, Decodable)]","highlight_start":10,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(Encodable)]","def_site_span":null}}],"children":[],"rendered":"warning: derive(Encodable) is deprecated in favor of derive(RustcEncodable)\n  --> /checkout/src/test/run-pass-fulldeps/deriving-encodable-decodable-cell-refcell.rs:28:10\n   |\nLL | #[derive(Encodable, Decodable)]\n   |          ^^^^^^^^^\n\n"}
[01:11:42] {"message":"unused imports: `Decodable`, `Encodable`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/deriving-encodable-decoe_name":"/checkout/src/test/run-pass-fulldeps/dropck_tarena_sound_drop.rs","byte_start":875,"byte_end":883,"line_start":19,"line_end":19,"column_start":10,"column_end":18,"is_primary":true,"text":[{"text":"#![allow(unstable)]","highlight_start":10,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unknown_lints)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unknown lint: `unstable`\n  --> /checkout/src/test/run-pass-fulldeps/dropck_tarena_sound_drop.rs:19:10\n   |\nLL | #![allow(unstable)]\n   |          ^^^^^^^^\n   |\n   = note: #[warn(unknown_lints)] on by default\n\n"}
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] thread '[run-pass] run-pass-fulldeps/dropck_tarena_sound_drop.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3237:9
[01:11:42] 
[01:11:42] 
[01:11:42] ---- [run-pass] run-pass-fulldeps/issue-11881.rs stdout ----
[01:11:42] normalized stderr:
[01:11:42] warning: derive(Encodable) is deprecated in favor of derive(RustcEncodable)
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL | #[derive(Encodable)]
[01:11:42] 
[01:11:42] 
[01:11:42] warning: derive(Encodable) is deprecated in favor of derive(RustcEncodable)
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL | #[derive(Encodable)]
[01:11:42] 
[01:11:42] warning: unused import: `std::fmt`
[01:11:42]   --> $DIR/issue-11881.rs:18:5
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL | use std::fmt;
[01:11:42]    |
[01:11:42]    = note: #[warn(unused_imports)] on by default
[01:11:42] 
[01:11:42] warning: unused import: `std::slice`
[01:11:42] warning: unused import: `std::slice`
[01:11:42]   --> $DIR/issue-11881.rs:19:5
[01:11:42]    |
[01:11:42] LL | use std::slice;
[01:11:42] 
[01:11:42] warning: unused import: `Encoder`
[01:11:42]   --> $DIR/issue-11881.rs:21:28
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL | use serialize::{Encodable, Encoder};
[01:11:42] 
[01:11:42] warning: struct is never constructed: `Bar`
[01:11:42]   --> $DIR/issue-11881.rs:31:1
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL | struct Bar {
[01:11:42]    |
[01:11:42]    = note: #[warn(dead_code)] on by default
[01:11:42] 
[01:11:42] 
[01:11:42] warning: variant is never constructed: `Opaque`
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL |     Opaque,
[01:11:42] 
[01:11:42] 
[01:11:42] warning: unused `std::result::Result` which must be used
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL |     val.encode(&mut encoder);
[01:11:42]    |
[01:11:42]    |
[01:11:42]    = note: this `Result` may be an `Err` variant, which should be handled
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] The actual stderr differed from the expected stderr.
[01:11:42] The actual stderr differed from the expected stderr.
[01:11:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-11881/issue-11881.stderr
[01:11:42] To update references, rerun the tests and pass the `--bless` flag
[01:11:42] To only update this specific test, also pass `--test-args issue-11881.rs`
[01:11:42] error: 1 errors occurred comparing output.
[01:11:42] status: exit code: 0
[01:11:42] status: exit code: 0
[01:11:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/issue-11881.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-11881/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-11881/auxiliary"
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] ------------------------------------------
[01:11:42] stderr:
[01:11:42] stderr:
[01:11:42] ------------------------------------------
[01:11:42] {"message":"derive(Encodable) is deprecated in favor of derive(RustcEncodable)","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/issue-11881.rs","byte_start":689,"byte_end":698,"line_start":25,"line_end":25,"column_start":10,"column_end":19,"is_primary":true,"text":[{"text":"#[derive(Encodable)]","highlight_start":10,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/run-pass-fulldeps/issue-11881.rs","byte_start":689,"byte_end":698,"line_start":25,"line_end":25,"column_start":10,"column_end":19,"is_primary":false,"text":[{"text":"#[derive(Encodable)]","highlight_start":10,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(Encodable)]","def_site_span":null}}],"children":[],"rendered":"warning: derive(Encodable) is deprecated in favor of derive(RustcEncodable)\n  --> /checkout/src/test/run-pass-fulldeps/issue-11881.rs:25:10\n   |\nLL | #[derive(Encodable)]\n   |          ^^^^^^^^^\n\n"}
[01:11:42] {"message":"derive(Encodable) is deprecated in favor of derive(RustcEncodable)","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/issue-11881.rs","byte_start":741,"byte_end":750,"line_start":30,"line_end":30,"column_start":10,"column_end":19,"is_primary":true,"text":[{"text":"#[derive(Encodable)]","highlight_start":10,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/run-pass-fulldeps/issue-11881.rs","byte_start":741,"byte_end":750,"line_start":30,"line_end":30,"column_start":10,"column_end":19,"is_primary":false,"text":[{"text":"#[derive(Encodable)]","highlight_start":10,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(Encodable)]","def_site_span":null}}],"children":[],"rendered":"warning: derive(Encodable) is deprecated in favor of derive(RustcEncodable)\n  --> /checkout/src/test/run-pass-fulldeps/issue-11881.rs:30:10\n   |\nLL | #[derive(Encodable)]\n   |          ^^^^^^^^^\n\n"}
[01:11:42] {"message":"unused import: `std::fmt`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/issue-11881.rs","byte_start":571,"byte_end":579,"line_start":18,"line_end":18,"column_start":5,"column_end":13,"is_primary":true,"text":[{"text":"use std::fmt;","highlight_start":5,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_imports)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused import: `std::fmt`\n  --> /checkout/src/test/run-pass-fulldeps/issue-11881.rs:18:5\n   |\nLL | use std::fmt;\n   |     ^^^^^^^^\n   |\n   = note: #[warn(unused_imports)] on by default\n\n"}
[01:11:42] {"message":"unused import: `std::slice`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/issue-11881.rs","byte_start":585,"byte_end":595,"line_start":19,"line_end":19,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"use std::slice;","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: unused import: `std::slice`\n  --> /checkout/src/test/run-pass-fulldeps/issue-11881.rs:19:5\n   |\nLL | use std::slice;\n   |     ^^^^^^^^^^\n\n"}
[01:11:42] {"message":"unused import: `Encoder`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/issue-11881.rs","byte_start":625,"byte_end":63warning: derive(Decodable) is deprecated in favor of derive(RustcDecodable)
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL | #[derive(Encodable, Decodable, PartialEq, Debug)]
[01:11:42] 
[01:11:42] 
[01:11:42] warning: derive(Encodable) is deprecated in favor of derive(RustcEncodable)
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL | #[derive(Encodable, Decodable, PartialEq, Debug)]
[01:11:42] 
[01:11:42] warning: unused import: `Encodable`
[01:11:42]   --> $DIR/issue-14021.rs:15:17
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL | use serialize::{Encodable, Decodable};
[01:11:42]    |
[01:11:42]    = note: #[warn(unused_imports)] on by default
[01:11:42] 
[01:11:42] warning: variable does not need to be mutable
[01:11:42] warning: variable does not need to be mutable
[01:11:42]   --> $DIR/issue-14021.rs:27:9
[01:11:42]    |
[01:11:42] LL |     let mut decoded_obj: UnitLikeStruct = Decodable::decode(&mut decoder).unwrap();
[01:11:42]    |         |
[01:11:42]    |         help: remove this `mut`
[01:11:42]    |
[01:11:42]    = note: #[warn(unused_mut)] on by default
[01:11:42]    = note: #[warn(unused_mut)] on by default
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] The actual stderr differed from the expected stderr.
[01:11:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-14021/issue-14021.stderr
[01:11:42] To update references, rerun the tests and pass the `--bless` flag
[01:11:42] To only update this specific test, also pass `--test-args issue-14021.rs`
ht_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(Decodable)]","def_site_span":null}}],"children":[],"rendered":"warning: derive(Decodable) is deprecated in favor of derive(RustcDecodable)\n  --> /checkout/src/test/run-pass-fulldeps/issue-14021.rs:18:21\n   |\nLL | #[derive(Encodable, Decodable, PartialEq, Debug)]\n   |                     ^^^^^^^^^\n\n"}
[01:11:42] {"message":"derive(Encodable) is deprecated in favor of derive(RustcEncodable)","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/issue-14021.rs","byte_start":590,"byte_end":599,"line_start":18,"line_end":18,"column_start":10,"column_end":19,"is_primary":true,"text":[{"text":"#[derive(Encodable, Decodable, PartialEq, Debug)]","highlight_start":10,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/run-pass-fulldeps/issue-14021.rs","byte_start":590,"byte_end":599,"line_start":18,"line_end":18,"column_start":10,"column_end":19,"is_primary":false,"text":[{"text":"#[derive(Encodable, Decodable, PartialEq, Debug)]","highlight_start":10,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(Encodable)]","def_site_span":null}}],"children":[],"rendered":"warning: derive(Encodable) is deprecated in favor of derive(RustcEncodable)\n  --> /checkout/src/test/run-pass-fulldeps/issue-14021.rs:18:10\n   |\nLL | #[derive(Encodable, Decodable, PartialEq, Debug)]\n   |          ^^^nused `std::result::Result` which must be used
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL |         json::encode(&self.v);
[01:11:42]    |
[01:11:42]    = note: #[warn(unused_must_use)] on by default
[01:11:42]    = note: #[warn(unused_must_use)] on by default
[01:11:42]    = note: this `Result` may be an `Err` variant, which should be handled
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] The actual stderr differed from the expected stderr.
[01:11:42] The actual stderr differed from the expected stderr.
[01:11:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-15924/issue-15924.stderr
[01:11:42] To update references, rerun the tests and pass the `--bless` flag
[01:11:42] To only update this specific test, also pass `--test-args issue-15924.rs`
[01:11:42] error: 1 errors occurred comparing output.
[01:11:42] status: exit code: 0
[01:11:42] status: exit code: 0
[01:11:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/issue-15924.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-15924/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-15924/auxiliary"
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] ------------------------------------------
[01:11:42] stderr:
[01:11:42] stderr:
[01:11:42] ------------------------------------------
[01:11:42] evel":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/issue-15924.rs","byte_start":736,"byte_end":758,"line_start":27,"line_end":27,"column_start":9,"column_end":31,"is_primary":true,"text":[{"text":"        json::encode(&self.v);","highlight_start":9,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_must_use)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"this `Result` may be an `Err` variant, which should be handled","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused `std::result::Result` which must be used\n  --> /checkout/src/test/run-pass-fulldeps/issue-15924.rs:27:9\n   |\nLL |         json::encode(&self.v);\n   |         ^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(unused_must_use)] on by default\n   = note: this `Result` may be an `Err` variant, which should be handled\n\n"}
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] thread '[run-pass] run-pass-fulldeps/issue-15924.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3237:9
[01:11:42] 
[01:11:42] 
[01:11:42] ---- [run-pass] run-pass-fulldeps/issue-15149.rs stdout ----
[01:11:42] normalized stderr:
[01:11:42] warning: unused variable: `my_dir`
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL |     let my_dir  = my_path.parent().unwrap();
[01:11:42]    |         ^^^^^^ help: consider using `_my_dir` instead
[01:11:42]    = note: #[warn(unused_variables)] on by default
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] The actual stderr differed from the expected stderr.
[01:11:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-15149/issue-15149.stderr
[01:11:42] To update references, rerun the tests and pass the `--bless` flag
[01:11:42] To only update this specific test, also pass `--test-args issue-15149.rs`
[01:11:42] error: 1 errors occurred comparing output.
[01:11:42] status: exit code: 0
[01:11:42] status: exit code: 0
[01:11:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/issue-15149.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-15149/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-15149/auxiliary"
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] ------------------------------------------
[01:11:42] stderr:
[01:11:42] stderr:
[01:11:42] ------------------------------------------
[01:11:42] {"message":"unused variable: `my_dir`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/issue-15149.rs","byte_start":1312,"byte_end":1318,"line_start":38,"line_end":38,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"    let my_dir  = my_path.parent().unwrap();","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_variables)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `_my_dir` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/issue-15149.rs","byte_start":1312,"byte_end":1318,"line_start":38,"line_end":38,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"    let my_dir  = my_path.parent().unwrap();","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":"_my_dir","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `my_dir`\n  --> /checkout/src/test/run-pass-fulldeps/issue-15149.rs:38:9\n   |\nLL |     let my_dir  = my_path.parent().unwrap();\n   |         ^^^^^^ help: consider using `_my_dir` instead\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] thread '[run-pass] run-pass-fulldeps/issue-15149.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3237:9
[01:11:42] 
[01:11:42] 
[01:11:42] ---- [run-pass] run-pass-fulldeps/issue-24972.rs stdout ----
[01:11:42] normalized stderr:
[01:11:42] warning: method is never used: `get`
[01:11:42]   --> $DIR/issue-24972.rs:31:5
---
[01:11:42] 
[01:11:42] warning: unused variable: `s`
[01:11:42]   --> $DIR/macro-crate-multi-decorator.rs:54:9
[01:11:42]    |
[01:11:42] LL |     let s = MyCopy { number: 42 };
[01:11:42]    |         ^ help: consider using `_s` instead
[01:11:42]    = note: #[warn(unused_variables)] on by default
[01:11:42] 
[01:11:42] 
[01:11:42] warning: field is never used: `number`
[01:11:42]    |
[01:11:42] LL |     number: i32
[01:11:42]    |     ^^^^^^^^^^^
[01:11:42]    |
[01:11:42]    |
[01:11:42]    = note: #[warn(dead_code)] on by default
[01:11:42] 
[01:11:42] warning: field is never used: `number`
[01:11:42]    |
[01:11:42] LL |     number: i32
[01:11:42]    |     ^^^^^^^^^^^
[01:11:42] 
[01:11:42] 
[01:11:42] warning: compiler plugin used as an ordinary library
[01:11:42]   --> $DIR/macro-crate-multi-decorator.rs:19:1
[01:11:42]    |
[01:11:42] LL | extern crate macro_crate_test;
[01:11:42]    |
[01:11:42]    |
[01:11:42]    = note: #[warn(plugin_as_library)] on by default
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] The actual stderr differed from the expected stderr.
[01:11:42] The actual stderr differed from the expected stderr.
[01:11:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-multi-decorator/macro-crate-multi-decorator.stderr
[01:11:42] To update references, rerun the tests and pass the `--bless` flag
[01:11:42] To only update this specific test, also pass `--test-args macro-crate-multi-decorator.rs`
[01:11:42] error: 1 errors occurred comparing output.
[01:11:42] status: exit code: 0
[01:11:42] status: exit code: 0
[01:11:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/macro-crate-multi-decorator.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-multi-decorator/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-multi-decorator/auxiliary"
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] ------------------------------------------
[01:11:42] stderr:
[01:11:42] stderr:
[01:11:42] ------------------------------------------
[01:11:42] {"message":"unused `#[macro_use]` import","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/macro-crate-multi-decorator.rs","byte_start":586,"byte_end":598,"line_start":17,"line_end":17,"column_start":1,"column_end":13,"is_primary":true,"text":[{"text":"#[macro_use]","highlight_start":1,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_imports)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused `#[macro_use]` import\n  --> /checkout/src/test/run-pass-fulldeps/macro-crate-multi-decorator.rs:17:1\n   |\nLL | #[macro_use]\n   | ^^^^^^^^^^^^\n   |\n   = note: #[warn(unused_imports)] on by default\n\n"}
[01:11:42] {"message":"unused variable: `s`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/macro-crate-multi-decorator.rs","byte_start":1299,"byte_end":1300,"line_start":54,"line_end":54,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let s = MyCopy { number: 42 };","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_variables)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `_s` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/macro-crate-multi-decorator.rs","byte_start":1299,"byte_end":1300,"line_start":54,"line_end":54,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let s = MyCopy { number: 42 };","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"_s","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `s`\n  --> /checkout/src/test/run-pass-fulldeps/macro-crate-multi-decorator.rs:54:9\n   |\nLL |     let s = MyCopy { number: 42 };\n   |         ^ help: consider using `_s` instead\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[01:11:42] {"message":"field is never used: `number`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/macro-crate-multi-decorator.rs","byte_start":773,"byte_end":784,"line_start":25,"line_end":25,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    number: i32","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: field is never used: `number`\n  --> /checkout/src/test/run-pass-fulldeps/macro-crate-multi-decorator.rs:25:5\n   |\nLL |     number: i32\n   |     ^^^^^^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[01:11:42] {"message":"field is never used: `number`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/macro-crate-multi-decorator.rs","byte_start":773,"byte_end":784,"line_start":25,"line_end":25,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    number: i32","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: field is never used: `number`\n  --> /checkout/src/test/run-pass-fulldeps/macro-crate-multi-decorator.rs:25:5\n   |\nLL |     number: i32\n   |     ^^^^^^^^^^^\n\n"}
[01:11:42] {"message":"compiler plugin used as an ordinary library","code":{"code":"plugin_as_library","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/macro-crate-multi-decorator.rs","byte_start":610,"byte_end":640,"line_start":19,"line_end":19,"column_start":1,"column_end":31,"is_primary":true,"text":[{"text":"extern crate macro_crate_test;","highlight_start":1,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(plugin_as_library)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: compiler plugin used as an ordinary library\n  --> /checkout/src/test/run-pass-fulldeps/macro-crate-multi-decorator.rs:19:1\n   |\nLL | extern crate macro_crate_test;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(plugin_as_library)] on by default\n\n"}
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] 
[01:1run-pass-fulldeps/macro-crate.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate/auxiliary"
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] ------------------------------------------
[01:11:42] stderr:
[01:11:42] stderr:
[01:11:42] ------------------------------------------
[01:11:42] {"message":"variant is never constructed: `Baz2`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/macro-crate.rs","byte_start":677,"byte_end":700,"line_start":21,"line_end":21,"column_start":1,"column_end":24,"is_primary":true,"text":[{"text":"#[rustc_into_multi_foo]","highlight_start":1,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: variant is never constructed: `Baz2`\n  --> /checkout/src/test/run-pass-fulldeps/macro-crate.rs:21:1\n   |\nLL | #[rustc_into_multi_foo]\n   | ^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[01:11:42] {"message":"function is never used: `foo2`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/macro-crate.rs","byte_start":826,"byte_end":835,"line_start":25,"line_end":25,"column_start":1,"column_end":10,"is_primary":true,"text":[{"text":"fn foo2() {}","highlight_start":1,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: function is never used: `foo2`\n  --> /checkout/src/test/run-pass-fulldeps/macro-crate.rs:25:1\n   |\nLL | fn foo2() {}\n   | ^^^^^^^^^\n\n"}
[01:11:42] {"message":"compiler plugin used as an ordinary library","code":{"code":"plugin_as_library","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/macro-crate.rs","byte_start":610,"byte_end":640,"line_start":18,"line_end":18,"column_start":1,"column_end":31,"is_primary":true,"text":[{"text":"extern crate macro_crate_test;","highlight_start":1,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(plugin_as_library)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: compiler plugin used as an ordinary library\n  --> /checkout/src/test/run-pass-fulldeps/macro-crate.rs:18:1\n   |\nLL | extern crate macro_crate_test;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(plugin_as_library)] on by default\n\n"}
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] 
[01:11:42] thread '[run-pass] run-pass-fulldeps/macro-crate.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3237plicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_variables)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `_x6` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/proc-macro/call-site.rs","byte_start":663,"byte_end":665,"line_start":22,"line_end":22,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"    let x6 = x5;","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":"_x6","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `x6`\n  --> /checkout/src/test/run-pass-fulldeps/proc-macro/call-site.rs:22:9\n   |\nLL |     let x6 = x5;\n   |         ^^ help: consider using `_x6` instead\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] thread '[run-pass] run-pass-fulldeps/proc-macro/call-site.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3237:9
[01:11:42] 
[01:11:42] 
[01:11:42] ---- [run-pass] run-pass-fulldeps/proc-macro/derive-attr-cfg.rs stdout ----
[01:11:42] normalized stderr:
[01:11:42] warning: struct is never constructed: `S`
[01:11:42]   --> $DIR/derive-attr-cfg.rs:19:1
[01:11:42]    |
[01:11:42] LL | struct S {
[01:11:42]    |
[01:11:42]    = note: #[warn(dead_code)] on by default
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] The actual stderr differed from the expected stderr.
[01:11:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/derive-attr-cfg/derive-attr-cfg.stderr
[01:11:42] To update references, rerun the tests and pass the `--bless` flag
[01:11:42] To only update this specific test, also pass `--test-args proc-macro/derive-attr-cfg.rs`
[01:11:42] error: 1 errors occurred comparing output.
[01:11:42] status: exit code: 0
[01:11:42] status: exit code: 0
[01:11:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/proc-macro/derive-attr-cfg.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/derive-attr-cfg/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/derive-attr-cfg/auxiliary"
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] ------------------------------------------
[01:11:42] stderr:
[01:11:42] stderr:
[01:11:42] ------------------------------------------
[01:11:42] {"message":"struct is never constructed: `S`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/proc-macro/derive-attr-cfg.rs","byte_start":596,"byte_end":604,"line_start":19,"line_end":19,"column_start":1,"column_end":9,"is_primary":true,"text":[{"text":"struct S {","highlight_start":1,"highlight_end":9}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: struct is never constructed: `S`\n  --> /checkout/src/test/run-pass-fulldeps/proc-macro/derive-attr-cfg.rs:19:1\n   |\nLL | struct S {\n   | ^^^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] thread '[run-pass] run-pass-fulldeps/proc-macro/derive-attr-cfg.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3237:9
[01:11:42] 
---
[01:11:42] normalized stderr:
[01:11:42] warning: struct is never constructed: `A`
[01:11:42]   --> $DIR/derive-same-struct.rs:18:1
[01:11:42]    |
[01:11:42] LL | struct A;
[01:11:42]    |
[01:11:42]    = note: #[warn(dead_code)] on by default
[01:11:42] 
[01:11:42] warning: path statement with no effect
[01:11:42] warning: path statement with no effect
[01:11:42]   --> $DIR/derive-same-struct.rs:21:5
[01:11:42]    |
[01:11:42] LL |     C;
[01:11:42]    |
[01:11:42]    |
[01:11:42]    = note: #[warn(path_statements)] on by default
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] The actual stderr differed from the expected stderr.
[01:11:42] The actual stderr differed from the expected stderr.
[01:11:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/derive-same-struct/derive-same-struct.stderr
[01:11:42] To update references, rerun the tests and pass the `--bless` flag
[01:11:42] To only update this specific test, also pass `--test-args proc-macro/derive-same-struct.rs`
[01:11:42] error: 2 errors occurred comparing output.
[01:11:42] status: exit code: 0
[01:11:42] status: exit code: 0
[01:11:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/proc-macro/derive-same-struct.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/derive-same-struct/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/derive-same-struct/auxiliary"
[01:11:42] ------------------------------------------
[01:11:42] input1: "struct A;"
[01:11:42] 
[01:11:42] ------------------------------------------
[01:11:42] ------------------------------------------
[01:11:42] stderr:
[01:11:42] ------------------------------------------
[01:11:42] {"message":"struct is never constructed: `A`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/proc-macro/derive-same-struct.rs","byte_start":583,"byte_end":592,"line_start":18,"line_end":18,"column_start":1,"column_end":10,"is_primary":true,"text":[{"text":"struct A;","highlight_start":1,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: struct is never constructed: `A`\n  --> /checkout/src/test/run-pass-fulldeps/proc-macro/derive-same-struct.rs:18:1\n   |\nLL | struct A;\n   | ^^^^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[01:11:42] {"message":"path statement with no effect","code":{"code":"path_statements","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/proc-macro/derive-same-struct.rs","byte_start":610,"byte_end":612,"line_start":21,"line_end":21,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    C;","highlight_start":5,"highlight_end":7}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(path_statements)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: path statement with no effect\n  --> /checkout/src/test/run-pass-fulldeps/proc-macro/derive-same-struct.rs:21:5\n   |\nLL |     C;\n   |     ^^\n   |\n   = note: #[warn(path_statements)] on by default\n\n"}
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] thread '[run-pass] run-pass-fulldeps/proc-macro/derive-same-struct.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3237:9
[01:11:42] 
[01:11:42] 
kout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/derive-two-attrs/derive-two-attrs.stderr
[01:11:42] To update references, rerun the tests and pass the `--bless` flag
[01:11:42] To only update this specific test, also pass `--test-args proc-macro/derive-two-attrs.rs`
[01:11:42] error: 1 errors occurred comparing output.
[01:11:42] status: exit code: 0
[01:11:42] status: exit code: 0
[01:11:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/proc-macro/derive-two-attrs.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/derive-two-attrs/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/derive-two-attrs/auxiliary"
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] ------------------------------------------
[01:11:42] stderr:
[01:11:42] stderr:
[01:11:42] ------------------------------------------
[01:11:42] {"message":"struct is never constructed: `B`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/proc-macro/derive-two-attrs.rs","byte_start":576,"byte_end":585,"line_start":20,"line_end":20,"column_start":1,"column_end":10,"is_primary":true,"text":[{"text":"struct B;","highlight_start":1,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_atage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/proc-macro/derive-union.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/derive-union/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/derive-union/auxiliary"
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] ------------------------------------------
[01:11:42] stderr:
[01:11:42] stderr:
[01:11:42] ------------------------------------------
[01:11:42] {"message":"unused variable: `t`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/proc-macro/derive-union.rs","byte_start":634,"byte_end":635,"line_start":24,"line_end":24,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let t = Test { a: 0 };","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_variables)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `_t` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/proc-macro/derive-union.rs","byte_start":634,"byte_end":635,"line_start":24,"line_end":24,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let t = Test { a: 0 };","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"_t","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `t`\n  --> /checkout/src/test/run-pass-fulldeps/proc-macro/derive-union.rs:24:9\n   |\nLL |     let t = Test { a: 0 };\n   |         ^ help: consider using `_t` instead\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] thread '[run-pass] run-pass-fulldeps/proc-macro/derive-union.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3237:9
[01:11:42] 
[01:11:42] 
[01:11:42] ---- [run-pass] run-pass-fulldeps/proc-macro/hygiene_example.rs stdout ----
[01:11:42] normalized stderr:
[01:11:42] warning: unused macro definition
[01:11:42]   --> $DIR/hygiene_example.rs:22:5
[01:11:42]    |
[01:11:42] LL |     macro_rules! format { () => {} } // does not interfere with `format!` from the proc macro
[01:11:42]    |
[01:11:42]    = note: #[warn(unused_macros)] on by default
[01:11:42] 
[01:11:42] warning: unused macro definition
[01:11:42] warning: unused macro definition
[01:11:42]   --> $DIR/hygiene_example.rs:23:5
[01:11:42]    |
[01:11:42] LL |     macro_rules! hello_helper { () => {} } // similarly does not intefere with the proc macro
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] The actual stderr differed from the expected stderr.
[01:11:42] Actual stderr saved to /checkout/obj/build/x86_64-l":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_macros)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused macro definition\n  --> /checkout/src/test/run-pass-fulldeps/proc-macro/hygiene_example.rs:22:5\n   |\nLL |     macro_rules! format { () => {} } // does not interfere with `format!` from the proc macro\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(unused_macros)] on by default\n\n"}
[01:11:42] {"message":"unused macro definition","code":{"code":"unused_macros","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/proc-macro/hygiene_example.rs","byte_start":860,"byte_end":898,"line_start":23,"line_end":23,"column_start":5,"column_end":43,"is_primary":true,"text":[{"text":"    macro_rules! hello_helper { () => {} } // similarly does not intefere with the proc macro","highlight_start":5,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: unused macro definition\n  --> /checkout/src/test/run-pass-fulldeps/proc-macro/hygiene_example.rs:23:5\n   |\nLL |     macro_rules! hello_helper { () => {} } // similarly does not intefere with the proc macro\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] thread '[run-pass] run-pass-fulldeps/proc-macro/hygiene_example.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3237:9
------------------------------------------
------------------------------------------
[01:11:42] {"message":"struct is never constructed: `S`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/proc-macro/issue-39889.rs","byte_start":591,"byte_end":600,"line_start":18,"line_end":18,"column_start":1,"column_end":10,"is_primary":true,"text":[{"text":"struct S;","highlight_start":1,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: struct is never constructed: `S`\n  --> /checkout/src/test/run-pass-fulldeps/proc-macro/issue-39889.rs:18:1\n   |\nLL | struct S;\n   | ^^^^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] thread '[run-pass] run-pass-fulldeps/proc-macro/issue-39889.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3237:9
[01:11:42] 
[01:11:42] 
[01:11:42] ---- [run-pass] run-pass-fulldeps/proc-macro/issue-50061.rs stdout ----
[01:11:42] normalized stderr:
[01:11:42] warning: path statement with no effect
[01:11:42]   --> $DIR/issue-50061.rs:23:12
[01:11:42]    |
[01:11:42] LL |     inner!(any_token $v)
[01:11:42]    |
[01:11:42]    |
[01:11:42]    = note: #[warn(path_statements)] on by default
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] The actual stderr differed from the expected stderr.
[01:11:42] The actual stderr differed from the expected stderr.
[01:11:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/issue-50061/issue-50061.stderr
[01:11:42] To update references, rerun the tests and pass the `--bless` flag
[01:11:42] To only update this specific test, also pass `--test-args proc-macro/issue-50061.rs`
[01:11:42] error: 1 errors occurred comparing output.
[01:11:42] status: exit code: 0
[01:11:42] status: exit code: 0
[01:11:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/proc-macro/issue-50061.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/issue-50061/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/issue-50061/auxiliary"
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] ------------------------------------------
[01:11:42] stderr:
[01:11:42] stderr:
[01:11:42] ------------------------------------------
[01:11:42] {"message":"path statement with no effect","code":{"code":"path_statements","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/proc-macro/issue-50061.rs","byte_start":640,"byte_end":649,"line_start":23,"line_end":23,"column_start":12,"column_end":21,"is_primary":true,"text":[{"text":"    inner!(any_token $v)","highlight_start":12,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(path_statements)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: path statement with no effect\n  --> /checkout/src/test/run-pass-fulldeps/proc-macro/issue-50061.rs:23:12\n   |\nLL |     inner!(any_token $v)\n   |            ^^^^^^^^^\n   |\n   = note: #[warn(path_statements)] on by default\n\n"}
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] thread '[run-pass] run-pass-fulldeps/proc-macro/issue-50061.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3237:9
[01:11:42] 
[01:11:42] 
[01:11:42] ---- [run-pass] run-pass-fulldeps/proc-macro/lifetimes.rs stdout ----
[01:11:42] normalized stderr:
[01:11:42] warning: unused variable: `l1`
[01:11:42]   --> $DIR/lifetimes.rs:32:9
[01:11:42]    |
[01:11:42] LL |     let l1 = Lifetimes { field: &0 };
[01:11:42]    |         ^^ help: consider using `_l1` instead
[01:11:42]    = note: #[warn(unused_variables)] on by default
[01:11:42] 
[01:11:42] warning: unused variable: `l2`
[01:11:42]   --> $DIR/lifetimes.rs:33:9
[01:11:42]   --> $DIR/lifetimes.rs:33:9
[01:11:42]    |
[01:11:42] LL |     let l2 = m::Lifetimes { field: &1 };
[01:11:42]    |         ^^ help: consider using `_l2` instead
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] The actual stderr differed from the expected stderr.
[01:11:42] The actual stderr differed from the expected stderr.
[01:11:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/lifetimes/lifetimes.stderr
[01:11:42] To update references, rerun the tests and pass the `--bless` flag
[01:11:4,"code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/proc-macro/lifetimes.rs","byte_start":832,"byte_end":834,"line_start":32,"line_end":32,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"    let l1 = Lifetimes { field: &0 };","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":"_l1","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `l1`\n  --> /checkout/src/test/run-pass-fulldeps/proc-macro/lifetimes.rs:32:9\n   |\nLL |     let l1 = Lifetimes { field: &0 };\n   |         ^^ help: consider using `_l1` instead\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[01:11:42] {"message":"unused variable: `l2`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/proc-macro/lifetimes.rs","byte_start":870,"byte_end":872,"line_start":33,"line_end":33,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"    let l2 = m::Lifetimes { field: &1 };","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `_l2` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/proc-macro/lifetimes.rs","byte_start":870,"byte_end":872,"line_start":33,"line_end":33,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"    let l2 = m::Lifetimes { field: &1 };","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":"_l2","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `l2`\n  --> /checkout/src/test/run-pass-fulldeps/proc-macro/lifetimes.rs:33:9\n   |\nLL |     let l2 = m::Lifetimes { field: &1 };\n   |         ^^ help: consider using `_l2` instead\n\n"}
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] thread '[run-pass] run-pass-fulldeps/proc-macro/lifetimes.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3237:9
[01:11:42] 
[01:11:42] 
[01:11:42] ---- [run-pass] run-pass-fulldeps/proc-macro/load-two.rs stdout ----
[01:11:42] normalized stderr:
[01:11:42] warning: struct is never constructed: `C`
[01:11:42]   --> $DIR/load-two.rs:25:1
[01:11:42]    |
[01:11:42] LL | struct C;
[01:11:42]    |
[01:11:42]    = note: #[warn(dead_code)] on by default
[01:11:42] 
[01:11:42] warning: path statement with no effect
[01:11:42] warning: path statement with no effect
[01:11:42]   --> $DIR/load-two.rs:28:5
[01:11:42]    |
[01:11:42] LL |     B;
[01:11:42]    |
[01:11:42]    |
[01:11:42]    = note: #[warn(path_statements)] on by default
[01:11:42] warning: path statement with no effect
[01:11:42]   --> $DIR/load-two.rs:29:5
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL |     D;
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] The actual stderr differed from the expected stderr.
[01:11:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/load-two/load-two.stderr
[01:11:42] To update references, rerun th  D;\n   |     ^^\n\n"}
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] thread '[run-pass] run-pass-fulldeps/proc-macro/load-two.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3237:9
[01:11:42] 
[01:11:42] 
[01:11:42] ---- [run-pass] run-pass-fulldeps/proc-macro/smoke.rs stdout ----
[01:11:42] normalized stderr:
[01:11:42] warning: path statement with no effect
[01:11:42]   --> $DIR/smoke.rs:21:5
[01:11:42]    |
[01:11:42] LL |     A;
[01:11:42]    |
[01:11:42]    |
[01:11:42]    = note: #[warn(path_statements)] on by default
[01:11:42] 
[01:11:42] warning: unused return value of `std::clone::Clone::clone` which must be used
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL |     A.clone();
[01:11:42]    |
[01:11:42]    = note: #[warn(unused_must_use)] on by default
[01:11:42]    = note: #[warn(unused_must_use)] on by default
[01:11:42]    = note: cloning is often expensive and is not expected to have side effects
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] The actual stderr differed from the expected stderr.
[01:11:42] The actual stderr differed from the expected stderr.
[01:11:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/smoke/smoke.stderr
[01:11:42] To update references, rerun the tests and pass the `--bless` flag
[01:11:42] To only update this specific test, also pass `--test-args proc-macro/smoke.rs`
[01:11:42] error: 1 errors occurred comparing output.
[01:11:42] status: exit code: 0
[01:11:42] status: exit code: 0
[01:11:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/proc-macro/smoke.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/smoke/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/smoke/auxiliary"
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] ------------------------------------------
[01:11:42] stderr:
[01:11:42] stderr:
[01:11:42] ------------------------------------------
[01:11:42] {"message":"path statement with no effect","code":{"code":"path_statements","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/proc-macro/smoke.rs","byte_start":622,"byte_end":624,"line_start":21,"line_end":21,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    A;","highlight_start":5,"highlight_end":7}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(path_statements)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: path statement with no effect\n  --> /checkout/src/test/run-pass-fulldeps/proc-macro/smoke.rs:21:5\n   |\nLL |     A;\n   |     ^^\n   |\n   = note: #[warn(path_statements)] on by default\n\n"}
[01:11:42] {"message":"unused return value of `std::clone::Clone::clone` which must be used","code":{"code":"unused_must_use","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/proc-macro/smoke.rs","byte_start":651,"byte_end":661,"line_start":23,"line_end":23,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    A.clone();","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_must_use)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"cloning is often expensive and is not expected to have side effects","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused return value of `std::clone::Clone::clone` which must be used\n  --> /checkout/src/test/run-pass-fulldeps/proc-macro/smoke.rs:23:5\n   |\nLL |     A.clone();\n   |     ^^^^^^^^^^\n   |\n   = note: #[warn(unused_must_use)] on by default\n   = note: cloning is often expensive and is not expected to have side effects\n\n"}
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] thread '[run-pass] run-pass-fulldeps/proc-macro/smoke.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3237:9
[01:11:42] 
[01:11:42] 
[01:11:42] ---- [run-pass] run-pass-fulldeps/proc-macro/struct-field-macro.rs stdout ----
[01:11:42] normalized stderr:
[01:11:42] warning: struct is never constructed: `S`
[01:11:42]   --> $DIR/struct-field-macro.rs:22:1
[01:11:42]    |
[01:11:42] LL | struct S {
[01:11:42]    |
[01:11:42]    = note: #[warn(dead_code)] on by default
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] The actual stderr differed from the expected stderr.
[01:11:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/struct-field-macro/struct-field-macro.stderr
[01:11:42] To update references, rerun the tests and pass the `--bless` flag
[01:11:42] To only update this specific test, also pass `--test-args proc-macro/struct-field-macro.rs`
[01:11:42] error: 1 errors occurred comparing output.
[01:11:42] status: exit code: 0
[01:11:42] status: exit code: 0
[01:11:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/proc-macro/struct-field-macro.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/struct-field-macro/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/struct-field-macro/auxiliary"
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] ------------------------------------------
[01:11:42] stderr:
[01:11:42] stderr:
[01:11:42] ------------------------------------------
[01:11:42] {"message":"struct is never constructed: `S`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/proc-macro/struct-field-macro.rs","byte_start":618,"byte_end":626,"line_start":22,"line_end":22,"column_start":1,"column_end":9,"is_primary":true,"text":[{"text":"struct S {","highlight_start":1,"highlight_end":9}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: struct is never constructed: `S`\n  --> /checkout/src/test/run-pass-fulldeps/proc-macro/struct-field-macro.rs:22:1\n   |\nLL | struct S {\n   | ^^^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] thread '[run-pass] run-pass-fulldeps/proc-macro/struct-field-macro.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3237:9
[01:11:42] 
[01:11:42] 
[01:11:42] ---- [run-pass] run-pass-fulldeps/quote-tokens.rs stdout ----
[01:11:42] normalized stderr:
[01:11:42] warning: unused import: `syntax::parse::PResult`
[01:11:42]   --> $DIR/quote-tokens.rs:18:5
[01:11:42]    |
[01:11:42] LL | use syntax::parse::PResult;
[01:11:42]    |
[01:11:42]    = note: #[warn(unused_imports)] on by default
[01:11:42] 
[01:11:42] 
[01:11:42] warning: unused import: `quote_tokens!(cx, 1 + 2)`
[01:11:42]   --> $DIR/quote-tokens.rs:21:56
[01:11:42]    |
[01:11:42] LL |     let e_toks : Vec<syntax::tokenstream::TokenTree> = quote_tokens!(cx, 1 + 2);
[01:11:42] 
[01:11:42] 
[01:11:42] warning: unused import: `quote_tokens!(cx, (x, 1 .. 4, *))`
[01:11:42]   --> $DIR/quote-tokens.rs:22:56
[01:11:42]    |
[01:11:42] LL |     let p_toks : Vec<syntax::tokode":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/quote-tokens.rs","byte_start":725,"byte_end":749,"line_start":21,"line_end":21,"column_start":56,"column_end":80,"is_primary":true,"text":[{"text":"    let e_toks : Vec<syntax::tokenstream::TokenTree> = quote_tokens!(cx, 1 + 2);","highlight_start":56,"highlight_end":80}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: unused import: `quote_tokens!(cx, 1 + 2)`\n  --> /checkout/src/test/run-pass-fulldeps/quote-tokens.rs:21:56\n   |\nLL |     let e_toks : Vec<syntax::tokenstream::TokenTree> = quote_tokens!(cx, 1 + 2);\n   |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:11:42] {"message":"unused import: `quote_tokens!(cx, (x, 1 .. 4, *))`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/quote-tokens.rs","byte_start":806,"byte_end":839,"line_start":22,"line_end":22,"column_start":56,"column_end":89,"is_primary":true,"text":[{"text":"    let p_toks : Vec<syntax::tokenstream::TokenTree> = quote_tokens!(cx, (x, 1 .. 4, *));","highlight_start":56,"highlight_end":89}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: unused import: `quote_tokens!(cx, (x, 1 .. 4, *))`\n  --> /checkout/src/test/run-pass-fulldeps/quote-tokens.rs:22:56\n   |\nLL |     let p_toks : Vec<syntax::tokenstream::TokenTree> = quote_tokens!(cx, (x, 1 .. 4, *));\n   | /quote-tokens.rs","byte_start":1925,"byte_end":1929,"line_start":48,"line_end":48,"column_start":9,"column_end":13,"is_primary":true,"text":[{"text":"    let expr: P<syntax::ast::Expr> = quote_expr!(cx, x + y);","highlight_start":9,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `_expr` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/quote-tokens.rs","byte_start":1925,"byte_end":1929,"line_start":48,"line_end":48,"column_start":9,"column_end":13,"is_primary":true,"text":[{"text":"    let expr: P<syntax::ast::Expr> = quote_expr!(cx, x + y);","highlight_start":9,"highlight_end":13}],"label":null,"suggested_replacement":"_expr","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `expr`\n  --> /checkout/src/test/run-pass-fulldeps/quote-tokens.rs:48:9\n   |\nLL |     let expr: P<syntax::ast::Expr> = quote_expr!(cx, x + y);\n   |         ^^^^ help: consider using `_expr` instead\n\n"}
[01:11:42] {"message":"function is never used: `syntax_extension`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/quote-tokens.rs","byte_start":634,"byte_end":667,"line_start":20,"line_end":20,"column_start":1,"column_end":34,"is_primary":true,"text":[{"text":"fn syntax_extension(cx: &ExtCtxt) {","highlight_start":1,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/quote-unused-sp-no-warning.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/quote-unused-sp-no-warning/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/quote-unused-sp-no-warning/auxiliary"
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] ------------------------------------------
[01:11:42] stderr:
[01:11:42] stderr:
[01:11:42] ------------------------------------------
[01:11:42] {"message":"function is never used: `test`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/quote-unused-sp-no-warning.rs","byte_start":608,"byte_end":633,"line_start":19,"line_end":19,"column_start":1,"column_end":26,"is_primary":true,"text":[{"text":"fn test(cx: &mut ExtCtxt) {","highlight_start":1,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: function is never used: `test`\n  --> /checkout/src/test/run-pass-fulldeps/quote-unused-sp-no-warning.rs:19:1\n   |\nLL | fn test(cx: &mut ExtCtxt) {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[01:11:42] 
[01:11:42]ckout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/regions-mock-tcx.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/regions-mock-tcx/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/regions-mock-tcx/auxiliary"
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] ------------------------------------------
[01:11:42] stderr:
[01:11:42] stderr:
[01:11:42] ------------------------------------------
[01:11:42] {"message":"unused import: `std::mem`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/regions-mock-tcx.rs","byte_start":828,"byte_end":836,"line_start":27,"line_end":27,"column_start":5,"column_end":13,"is_primary":true,"text":[{"text":"use std::mem;","highlight_start":5,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_imports)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused import: `std::mem`\n  --> /checkout/src/test/run-pass-fulldeps/regions-mock-tcx.rs:27:5\n   |\nLL | use std::mem;\n   |     ^^^^^^^^\n   |\n   = note: #[warn(unused_imports)] on by default\n\n"}
[01:11:42] {"message":"variant is never constructed: `ExprVar`","code-pass-fulldeps/regions-mock-tcx.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3237:9
[01:11:42] ---- [run-pass] run-pass-fulldeps/rename-directory.rs stdout ----
[01:11:42] normalized stderr:
[01:11:42] warning: unused import: `std::ffi::CString`
[01:11:42]   --> $DIR/rename-directory.rs:17:5
[01:11:42]   --> $DIR/rename-directory.rs:17:5
[01:11:42]    |
[01:11:42] LL | use std::ffi::CString;
[01:11:42]    |
[01:11:42]    = note: #[warn(unused_imports)] on by default
[01:11:42] 
[01:11:42] 
[01:11:42] warning: unused `std::result::Result` which must be used
[01:11:42]    |
[01:11:42]    |
[01:11:42] LL |     fs::rename(&old_path, &new_path.join("newdir"));
[01:11:42]    |
[01:11:42]    = note: #[warn(unused_must_use)] on by default
[01:11:42]    = note: #[warn(unused_must_use)] on by default
[01:11:42]    = note: this `Result` may be an `Err` variant, which should be handled
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] The actual stderr differed from the expected stderr.
[01:11:42] The actual stderr differed from the expected stderr.
[01:11:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/rename-directory/rename-directory.stderr
[01:11:42] To update references, rerun the tests and pass the `--bless` flag
[01:11:42] To only update this specific test, also pass `--test-args rename-directory.rs`
[01:11:42] error: 1 errors occurred comparing output.
[01:11:42] status: exit code: 0
[01:11:42] status: exit code: 0
[01:11:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/rename-directory.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/rename-directory/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/rename-directory/auxiliary"
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] ------------------------------------------
[01:11:42] stderr:
[01:11:42] stderr:
[01:11:42] ------------------------------------------
[01:11:42] {"message":"unused import: `std::ffi::CString`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/rename-directory.rs","byte_start":605,"byte_end":622,"line_start":17,"line_end":17,"column_start":5,"column_end":22,"is_primary":true,"text":[{"text":"use std::ffi::CString;","highlight_start":5,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_imports)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused import: `std::ffi::CString`\n  --> /checkout/src/test/run-pass-fulldeps/rename-directory.rs:17:5\n   |\nLL | use std::ffi::CString;\n   |     ^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(unused_imports)] on by default\n\n"}
[01:11:42] {"message":"unused `std::result::Result` which must be used","code":{"code":"unused_must_use","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/rename-directory.rs","byte_start":1046,"byte_end":1094,"line_start":31,"line_end":31,"column_start":5,"column_end":53,"is_primary":true,"text":[{"text":"    fs::rename(&old_path, &new_path.join(\"newdir\"));","highlight_start":5,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_must_use)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"this `Result` may be an `Err` variant, which should be handled","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused `std::result::Result` which must be used\n  --> /checkout/src/test/run-pass-fulldeps/rename-directory.rs:31:5\n   |\nLL |     fs::rename(&old_path, &new_path.join(\"newdir\"));\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(unused_must_use)] on by default\n   = note: this `Result` may be an `Err` variant, which should be handled\n\n"}
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] thread '[run-pass] run-pass-fulldeps/rename-directory.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3237:9
[01:11:42] 
[01:11:42] 
[01:11:42] ---- [run-pass] run-pass-fulldeps/qquote.rs stdout ----
[01:11:42] normalized stderr:
[01:11:42] warning: unused import: `syntax::symbol::Symbol`
[01:11:42]   --> $DIR/qquote.rs:20:5
[01:11:42]    |
[01:11:42] LL | use syntax::symbol::Symbol;
[01:11:42]    |
[01:11:42]    = note: #[warn(unused_imports)] on by default
[01:11:42] 
[01:11:42] warning: unused import: `syntax_pos::DUMMY_SP`
[01:11:42] warning: unused import: `syntax_pos::DUMMY_SP`
[01:11:42]   --> $DIR/qquote.rs:21:5
[01:11:42]    |
[01:11:42] LL | use syntax_pos::DUMMY_SP;
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] The actual stderr differed from the expected stderr.
[01:11:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/qquote/qquote.stderr
[01:11:42] To update references, rerun the tests and pass the `--bless` flag
[01:11:42] To only update this specific test, also pass `--test-args qquote.rs`
[01:11:42] error: 1 errors occurred comparing output.
[01:11:42] status: exit code: 0
[01:11:42] status: exit code: 0
[01:11:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/qquote.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/qquote/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/qquote/auxiliary"
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] ------------------------------------------
[01:11:42] stderr:
[01:11:42] stderr:
[01:11:42] ------------------------------------------
[01:11:42] {"message":"unused import: `syntax::symbol::Symbol`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/qquote.rs","byte_start":649,"byte_end":671,"line_start":20,"line_end":20,"column_start":5,"column_end":27,"is_primary":true,"text":[{"text":"use syntax::symbol::Symbol;","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_imports)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused import: `syntax::symbol::Symbol`\n  --> /checkout/src/test/run-pass-fulldeps/qquote.rs:20:5\n   |\nLL | use syntax::symbol::Symbol;\n   |     ^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(unused_imports)] on by default\n\n"}
[01:11:42] {"message":"unused import: `syntax_pos::DUMMY_SP`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/qquote.rs","byte_start":677,"byte_end":697,"line_start":21,"line_end":21,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"use syntax_pos::DUMMY_SP;","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: unused import: `syntax_pos::DUMMY_SP`\n  --> /checkout/src/test/run-pass-fulldeps/qquote.rs:21:5\n   |\nLL | use syntax_pos::DUMMY_SP;\n   |     ^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:11:42] ------------------------------------------
[01:11:42] 
[01:11:42] thread '[run-pass] run-pass-fulldeps/qquote.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3237:9
[01:11:42] 
---
[01:11:42] test result: FAILED. 58 passed; 39 failed; 0 ignored; 0 measured; 0 filtered out
[01:11:42] 
[01:11:42] 
[01:11:42] 
[01:11:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:11:42] 
[01:11:42] 
[01:11:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:11:42] Build completed unsuccessfully in 0:22:49
[01:11:42] Build completed unsuccessfully in 0:22:49
[01:11:42] make: *** [check] Error 1
[01:11:42] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:015356ac
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
