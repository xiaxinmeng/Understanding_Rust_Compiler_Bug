plain
[00:51:29] ....................................................................................................
[00:51:35] ....................................................................................................
[00:51:42] ....................................................................................................
[00:51:49] ....................................................................................................
[00:51:56] ...........................................................F........................................
[00:52:11] ..i.................................................................................................
[00:52:18] ................ii..................................................................................
[00:52:35] ..................................................................
[00:52:35] failures:
[00:52:35] failures:
[00:52:35] 
[00:52:35] ---- [ui (nll)] ui/lint/suggestions.rs stdout ----
[00:52:35]  diff of stderr:
[00:52:35] 
[00:52:35] 44    |  |____________|
[00:52:35] 45    |               help: remove this `mut`
[00:52:35] 46 
[00:52:35] + warning: variable does not need to be mutable
[00:52:35] +   --> $DIR/suggestions.rs:48:13
[00:52:35] +    |
[00:52:35] + LL |         let mut a = (1); // should suggest no `mut`, no parens
[00:52:35] +    |             ----^
[00:52:35] +    |             |
[00:52:35] +    |             help: remove this `mut`
[00:52:35] + 
[00:52:35] 47 warning: static is marked #[no_mangle], but not exported
[00:52:35] 48   --> $DIR/suggestions.rs:16:14
[00:52:35] 
[00:52:35] 
[00:52:35] The actual stderr differed from the expected stderr.
[00:52:35] The actual stderr differed from the expected stderr.
[00:52:35] Actual stdt","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/suggestions.rs","byte_start":1677,"byte_end":1682,"line_start":48,"line_end":48,"column_start":13,"column_end":18,"is_primary":true,"text":[{"text":"        let mut a = (1); // should suggest no `mut`, no parens","highlight_start":13,"highlight_end":18}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"remove this `mut`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/suggestions.rs","byte_start":1677,"byte_end":1681,"line_start":48,"line_end":48,"column_start":13,"column_end":17,"is_primary":true,"text":[{"text":"        let mut a = (1); // should suggest no `mut`, no parens","highlight_start":13,"highlight_end":17}],"label":null,"suggested_replacement":"","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: variable does not need to be mutable\n  --> /checkout/src/test/ui/lint/suggestions.rs:48:13\n   |\nLL |         let mut a = (1); // should suggest no `mut`, no parens\n   |             ----^\n   |             |\n   |             help: remove this `mut`\n\n"}
[00:52:35] {"message":"static is marked #[no_mangle], but not exported","code":{"code":"private_no_mangle_statics","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/suggestions.rs","byte_start":607,"byte_end":634,"line_start":16,"line_end":16,"column_start":14,"column_end":41,"is_primary":true,"text":[{"text":"#[no_mangle] static SHENZHOU: usize = 1; // should suggest `pub`","highlight_start":14,"highlight_end":41}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"#[warn(private_no_mangle_statics)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try making it public","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/suggestions.rs","byte_start":607,"byte_end":607,"line_start":16,"line_end":16,"column_start":14,"column_end":14,"is_primary":true,"text":[{"text":"#[no_mangle] static SHENZHOU: usize = 1; // should suggest `pub`","highlight_start":14,"highlight_end":14}],"label":null,"suggested_replacement":"pub ","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: static is marked #[no_mangle], but not exported\n  --> /checkout/src/test/ui/lint/suggestions.rs:16:14\n   |\nLL | #[no_mangle] static SHENZHOU: usize = 1; // should suggest `pub`\n   |              -^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |              |\n   |              help: try making it public: `pub`\n   |\n   = note: #[warn(private_no_mangle_statics)] on by default\n\n"}
[00:52:35] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[00:52:35] {"message":"const items should never be #[no_mangle]","code":{"code":"no_mangle_const_items","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/suggestions.rs","byte_start":712,"byte_end":739,"line_start":18,"line_end":18,"column_start":14,"column_end":41,"is_primary":true,"text":[{"text":"#[no_mangle] const DISCOVERY: usize = 1; // should suggest `pub static` rather than `const`","highlight_start":14,"highlight_end":41}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"#[deny(no_mangle_const_items)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try a static value","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/suggestions.rs","byte_start":712,"byte_end":717,"line_start":18,"line_end":18,"column_start":14,"column_end":19,"is_primary":true,"text":[{"text":"#[no_mangle] const DISCOVERY: usize = 1; // should suggest `pub static` rather than `const`","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":"pub static","expansion":null}],"children":[],"rendered":null}],"rendered":"error: const items should never be #[no_mangle]\n  --> /checkout/src/test/ui/lint/suggestions.rs:18:14\n   |\nLL | #[no_mangle] const DISCOVERY: usize = 1; // should suggest `pub static` rather than `const`\n   |              -----^^^^^^^^^^^^^^^^^^^^^^\n   |              |\n   |              help: try a static value: `pub static`\n   |\n   = note: #[deny(no_mangle_const_items)] on by default\n\n"}
[00:52:35] {"message":"functions generic over types must be mangled","code":{"code":"no_mangle_generic_items","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/suggestions.rs","byte_start":913,"byte_end":940,"line_start":22,"line_end":22,"column_start":1,"column_end":28,"is_primary":true,"text":[{"text":"pub fn defiant<T>(_t: T) {}","highlight_start":1,"highlight_end":28}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"#[warn(no_mangle_generic_items)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":n3982380 .
2716940 ./obj/build
1957676 ./obj/build/x86_64-unknown-linux-gnu
722080 ./src
569140 ./obj/build/bootstrap
---
149124 ./src/llvm-emscripten/test
148956 ./.git/modules
148952 ./.git/modules/src
122700 ./obj/build/bootstrap/debug/incremental/bootstrap-1sil8jgb030ka
122696 ./obj/build/bootstrap/debug/incremental/bootstrap-1sil8jgb030ka/s-f0cidcixkh-2xgqfi-2ud7sffvl82ye
78376 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental
70976 ./obj/build/x86_64-unknown-linux-gnu/native
70568 ./.git/modules/src/tools
70512 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
---
56088 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/bin
55348 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release
53568 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/build
52004 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-33oa6nnkk1g08
52000 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-33oa6nnkk1g08/s-f0cjmm96s8-vzzwy5-2ky3dmoclz0i2
47832 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
46828 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps
46696 ./src/test
46660 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/release
