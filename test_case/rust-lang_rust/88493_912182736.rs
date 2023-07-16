plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 371f3cd3fe523d0b398ed1db1620667c53ba7d02 and 3fdab52bc2fa81ebeaf62e8d3e2d291e59ed6e01
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

---- compile_test stdout ----
diff of stderr:

 error: this `match` has identical arm bodies
    |
    |
 LL |         _ => 0, //~ ERROR match arms have same body
    |
    |
    = note: `-D clippy::match-same-arms` implied by `-D warnings`
 note: same as this
    |
    |
 LL |         Abc::A => 0,
    |                   ^
 note: `Abc::A` has the same arm body as the `_` wildcard, consider removing it
    |
    |
 LL |         Abc::A => 0,
 
 
 error: this `match` has identical arm bodies
    |
    |
 LL |         (.., 3) => 42, //~ ERROR match arms have same body
    |
 note: same as this
   --> $DIR/match_same_arms.rs:17:23
    |
    |
 LL |         (1, .., 3) => 42,
    |                       ^^
 help: consider refactoring into `(1, .., 3) | (.., 3)`
    |
    |
 LL |         (1, .., 3) => 42,
    |         ^^^^^^^^^^
    = help: ...or consider changing the match arm bodies
 
 error: this `match` has identical arm bodies
    |
    |
 LL |         51 => 1, //~ ERROR match arms have same body
    |
 note: same as this
   --> $DIR/match_same_arms.rs:23:15
    |
    |
 LL |         42 => 1,
    |               ^
 help: consider refactoring into `42 | 51`
    |
 LL |         42 => 1,
    |         ^^
    |         ^^
    = help: ...or consider changing the match arm bodies
 
 error: this `match` has identical arm bodies
    |
    |
 LL |         52 => 2, //~ ERROR match arms have same body
    |
 note: same as this
   --> $DIR/match_same_arms.rs:25:15
    |
    |
 LL |         41 => 2,
    |               ^
 help: consider refactoring into `41 | 52`
    |
 LL |         41 => 2,
    |         ^^
    |         ^^
    = help: ...or consider changing the match arm bodies
 
 error: this `match` has identical arm bodies
    |
    |
 LL |         2 => 2, //~ ERROR 2nd matched arms have same body
    |
 note: same as this
   --> $DIR/match_same_arms.rs:31:14
    |
    |
 LL |         1 => 2,
    |              ^
 help: consider refactoring into `1 | 2`
    |
 LL |         1 => 2,
    |         ^
    |         ^
    = help: ...or consider changing the match arm bodies
 
 error: this `match` has identical arm bodies
    |
    |
 LL |         3 => 2, //~ ERROR 3rd matched arms have same body
    |
 note: same as this
   --> $DIR/match_same_arms.rs:31:14
    |
    |
 LL |         1 => 2,
    |              ^
 help: consider refactoring into `1 | 3`
    |
 LL |         1 => 2,
    |         ^
    |         ^
    = help: ...or consider changing the match arm bodies
 
 
 error: this `match` has identical arm bodies
-   |
-   |
-LL |         3 => 2, //~ ERROR 3rd matched arms have same body
-   |              ^
-   |
-note: same as this
-   |
-   |
-LL |         2 => 2, //~ ERROR 2nd matched arms have same body
-   |              ^
-help: consider refactoring into `2 | 3`
-   |
-   |
-LL |         2 => 2, //~ ERROR 2nd matched arms have same body
-   |         ^
-   = help: ...or consider changing the match arm bodies
-
-error: this `match` has identical arm bodies
    |
    |
 LL |                 CommandInfo::External { name, .. } => name.to_string(),
    |
 note: same as this
   --> $DIR/match_same_arms.rs:49:54
    |
    |
 LL |                 CommandInfo::BuiltIn { name, .. } => name.to_string(),
    |                                                      ^^^^^^^^^^^^^^^^
 help: consider refactoring into `CommandInfo::BuiltIn { name, .. } | CommandInfo::External { name, .. }`
    |
    |
 LL |                 CommandInfo::BuiltIn { name, .. } => name.to_string(),
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = help: ...or consider changing the match arm bodies
-error: aborting due to 8 previous errors
+error: aborting due to 7 previous errors
 
 
---
To only update this specific test, also pass `--test-args match_same_arms.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/match_same_arms.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/match_same_arms.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-ff0575c5a2ca7cc8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7064d3f8532fb2a5.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-06539012b522c0cf.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-8e5c09d8dc991112.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-90129c5dba212efa.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-17d0421109b348d4.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-54a7fffacad7d378.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/match_same_arms.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this `match` has identical arm bodies","code":{"code":"clippy::match_same_arms","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":179,"byte_end":180,"line_start":13,"line_end":13,"column_start":14,"column_end":15,"is_primary":true,"text":[{"text":"        _ => 0, //~ ERROR match arms have same body","highlight_start":14,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::match-same-arms` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"same as this","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":142,"byte_end":143,"line_start":11,"line_end":11,"column_start":19,"column_end":20,"is_primary":true,"text":[{"text":"        Abc::A => 0,","highlight_start":19,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"`Abc::A` has the same arm body as the `_` wildcard, consider removing it","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":142,"byte_end":143,"line_start":11,"line_end":11,"column_start":19,"column_end":20,"is_primary":true,"text":[{"text":"        Abc::A => 0,","highlight_start":19,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `match` has identical arm bodies\n  --> tests/ui/match_same_arms.rs:13:14\n   |\nLL |         _ => 0, //~ ERROR match arms have same body\n   |              ^\n   |\n   = note: `-D clippy::match-same-arms` implied by `-D warnings`\nnote: same as this\n  --> tests/ui/match_same_arms.rs:11:19\n   |\nLL |         Abc::A => 0,\n   |                   ^\nnote: `Abc::A` has the same arm body as the `_` wildcard, consider removing it\n  --> tests/ui/match_same_arms.rs:11:19\n   |\nLL |         Abc::A => 0,\n   |                   ^\n\n"}
{"message":"this `match` has identical arm bodies","code":{"code":"clippy::match_same_arms","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":293,"byte_end":295,"line_start":18,"line_end":18,"column_start":20,"column_end":22,"is_primary":true,"text":[{"text":"        (.., 3) => 42, //~ ERROR match arms have same body","highlight_start":20,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"same as this","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":270,"byte_end":272,"line_start":17,"line_end":17,"column_start":23,"column_end":25,"is_primary":true,"text":[{"text":"        (1, .., 3) => 42,","highlight_start":23,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"consider refactoring into `(1, .., 3) | (.., 3)`","code":null,"level":"help","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":256,"byte_end":266,"line_start":17,"line_end":17,"column_start":9,"column_end":19,"is_primary":true,"text":[{"text":"        (1, .., 3) => 42,","highlight_start":9,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...or consider changing the match arm bodies","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this `match` has identical arm bodies\n  --> tests/ui/match_same_arms.rs:18:20\n   |\nLL |         (.., 3) => 42, //~ ERROR match arms have same body\n   |                    ^^\n   |\nnote: same as this\n  --> tests/ui/match_same_arms.rs:17:23\n   |\nLL |         (1, .., 3) => 42,\n   |                       ^^\nhelp: consider refactoring into `(1, .., 3) | (.., 3)`\n  --> tests/ui/match_same_arms.rs:17:9\n   |\nLL |         (1, .., 3) => 42,\n   |         ^^^^^^^^^^\n   = help: ...or consider changing the match arm bodies\n\n"}
{"message":"this `match` has identical arm bodies","code":{"code":"clippy::match_same_arms","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":411,"byte_end":412,"line_start":24,"line_end":24,"column_start":15,"column_end":16,"is_primary":true,"text":[{"text":"        51 => 1, //~ ERROR match arms have same body","highlight_start":15,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"same as this","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":394,"byte_end":395,"line_start":23,"line_end":23,"column_start":15,"column_end":16,"is_primary":true,"text":[{"text":"        42 => 1,","highlight_start":15,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"consider refactoring into `42 | 51`","code":null,"level":"help","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":388,"byte_end":390,"line_start":23,"line_end":23,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"        42 => 1,","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...or consider changing the match arm bodies","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this `match` has identical arm bodies\n  --> tests/ui/match_same_arms.rs:24:15\n   |\nLL |         51 => 1, //~ ERROR match arms have same body\n   |               ^\n   |\nnote: same as this\n  --> tests/ui/match_same_arms.rs:23:15\n   |\nLL |         42 => 1,\n   |               ^\nhelp: consider refactoring into `42 | 51`\n  --> tests/ui/match_same_arms.rs:23:9\n   |\nLL |         42 => 1,\n   |         ^^\n   = help: ...or consider changing the match arm bodies\n\n"}
{"message":"this `match` has identical arm bodies","code":{"code":"clippy::match_same_arms","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":481,"byte_end":482,"line_start":26,"line_end":26,"column_start":15,"column_end":16,"is_primary":true,"text":[{"text":"        52 => 2, //~ ERROR match arms have same body","highlight_start":15,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"same as this","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":464,"byte_end":465,"line_start":25,"line_end":25,"column_start":15,"column_end":16,"is_primary":true,"text":[{"text":"        41 => 2,","highlight_start":15,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"consider refactoring into `41 | 52`","code":null,"level":"help","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":458,"byte_end":460,"line_start":25,"line_end":25,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"        41 => 2,","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...or consider changing the match arm bodies","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this `match` has identical arm bodies\n  --> tests/ui/match_same_arms.rs:26:15\n   |\nLL |         52 => 2, //~ ERROR match arms have same body\n   |               ^\n   |\nnote: same as this\n  --> tests/ui/match_same_arms.rs:25:15\n   |\nLL |         41 => 2,\n   |               ^\nhelp: consider refactoring into `41 | 52`\n  --> tests/ui/match_same_arms.rs:25:9\n   |\nLL |         41 => 2,\n   |         ^^\n   = help: ...or consider changing the match arm bodies\n\n"}
{"message":"this `match` has identical arm bodies","code":{"code":"clippy::match_same_arms","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":596,"byte_end":597,"line_start":32,"line_end":32,"column_start":14,"column_end":15,"is_primary":true,"text":[{"text":"        2 => 2, //~ ERROR 2nd matched arms have same body","highlight_start":14,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"same as this","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":580,"byte_end":581,"line_start":31,"line_end":31,"column_start":14,"column_end":15,"is_primary":true,"text":[{"text":"        1 => 2,","highlight_start":14,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"consider refactoring into `1 | 2`","code":null,"level":"help","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":575,"byte_end":576,"line_start":31,"line_end":31,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        1 => 2,","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...or consider changing the match arm bodies","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this `match` has identical arm bodies\n  --> tests/ui/match_same_arms.rs:32:14\n   |\nLL |         2 => 2, //~ ERROR 2nd matched arms have same body\n   |              ^\n   |\nnote: same as this\n  --> tests/ui/match_same_arms.rs:31:14\n   |\nLL |         1 => 2,\n   |              ^\nhelp: consider refactoring into `1 | 2`\n  --> tests/ui/match_same_arms.rs:31:9\n   |\nLL |         1 => 2,\n   |         ^\n   = help: ...or consider changing the match arm bodies\n\n"}
{"message":"this `match` has identical arm bodies","code":{"code":"clippy::match_same_arms","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":654,"byte_end":655,"line_start":33,"line_end":33,"column_start":14,"column_end":15,"is_primary":true,"text":[{"text":"        3 => 2, //~ ERROR 3rd matched arms have same body","highlight_start":14,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"same as this","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":580,"byte_end":581,"line_start":31,"line_end":31,"column_start":14,"column_end":15,"is_primary":true,"text":[{"text":"        1 => 2,","highlight_start":14,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"consider refactoring into `1 | 3`","code":null,"level":"help","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":575,"byte_end":576,"line_start":31,"line_end":31,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        1 => 2,","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...or consider changing the match arm bodies","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this `match` has identical arm bodies\n  --> tests/ui/match_same_arms.rs:33:14\n   |\nLL |         3 => 2, //~ ERROR 3rd matched arms have same body\n   |              ^\n   |\nnote: same as this\n  --> tests/ui/match_same_arms.rs:31:14\n   |\nLL |         1 => 2,\n   |              ^\nhelp: consider refactoring into `1 | 3`\n  --> tests/ui/match_same_arms.rs:31:9\n   |\nLL |         1 => 2,\n   |         ^\n   = help: ...or consider changing the match arm bodies\n\n"}
{"message":"this `match` has identical arm bodies","code":{"code":"clippy::match_same_arms","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":1167,"byte_end":1183,"line_start":50,"line_end":50,"column_start":55,"column_end":71,"is_primary":true,"text":[{"text":"                CommandInfo::External { name, .. } => name.to_string(),","highlight_start":55,"highlight_end":71}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"same as this","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":1095,"byte_end":1111,"line_start":49,"line_end":49,"column_start":54,"column_end":70,"is_primary":true,"text":[{"text":"                CommandInfo::BuiltIn { name, .. } => name.to_string(),","highlight_start":54,"highlight_end":70}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"consider refactoring into `CommandInfo::BuiltIn { name, .. } | CommandInfo::External { name, .. }`","code":null,"level":"help","spans":[{"file_name":"tests/ui/match_same_arms.rs","byte_start":1058,"byte_end":1091,"line_start":49,"line_end":49,"column_start":17,"column_end":50,"is_primary":true,"text":[{"text":"                CommandInfo::BuiltIn { name, .. } => name.to_string(),","highlight_start":17,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...or consider changing the match arm bodies","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this `match` has identical arm bodies\n  --> tests/ui/match_same_arms.rs:50:55\n   |\nLL |                 CommandInfo::External { name, .. } => name.to_string(),\n   |                                                       ^^^^^^^^^^^^^^^^\n   |\nnote: same as this\n  --> tests/ui/match_same_arms.rs:49:54\n   |\nLL |                 CommandInfo::BuiltIn { name, .. } => name.to_string(),\n   |                                                      ^^^^^^^^^^^^^^^^\nhelp: consider refactoring into `CommandInfo::BuiltIn { name, .. } | CommandInfo::External { name, .. }`\n  --> tests/ui/match_same_arms.rs:49:17\n   |\nLL |                 CommandInfo::BuiltIn { name, .. } => name.to_string(),\n   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   = help: ...or consider changing the match arm bodies\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: this arithmetic operation will overflow
    |
    |
 LL |     i32::MIN % (-1); // also caught by rustc
    |     ^^^^^^^^^^^^^^^ attempt to compute the remainder of `i32::MIN % -1_i32`, which would overflow
    |
    = note: `#[deny(arithmetic_overflow)]` on by default
 
 error: this arithmetic operation will overflow
    |
    |
 LL |     INT_MIN % NEG_ONE; // also caught by rustc
    |     ^^^^^^^^^^^^^^^^^ attempt to compute the remainder of `i64::MIN % -1_i64`, which would overflow
 
 error: this arithmetic operation will overflow
    |
    |
 LL |     INT_MIN % STATIC_NEG_ONE; // ONLY caught by rustc
    |     ^^^^^^^^^^^^^^^^^^^^^^^^ attempt to compute the remainder of `i64::MIN % -1_i64`, which would overflow
 
 error: any number modulo 1 will be 0
    |
 LL |     10 % 1;
    |     ^^^^^^
    |
    |
    = note: `-D clippy::modulo-one` implied by `-D warnings`
 
 error: any number modulo -1 will panic/overflow or result in 0
    |
 LL |     10 % -1;
    |     ^^^^^^^
 
 
 error: any number modulo -1 will panic/overflow or result in 0
    |
    |
 LL |     i32::MIN % (-1); // also caught by rustc
 
 
 error: the operation is ineffective. Consider reducing it to `1`
    |
    |
 LL |     const ONE: u32 = 1 * 1;
    |
    |
    = note: `-D clippy::identity-op` implied by `-D warnings`
 
-error: the operation is ineffective. Consider reducing it to `1`
-   |
-   |
-LL |     const ONE: u32 = 1 * 1;
-   |                      ^^^^^
-
 error: any number modulo 1 will be 0
    |
 LL |     2 % ONE;
    |     ^^^^^^^
 
 
 error: any number modulo -1 will panic/overflow or result in 0
    |
    |
 LL |     2 % NEG_ONE;
 
 
 error: any number modulo -1 will panic/overflow or result in 0
    |
    |
 LL |     INT_MIN % NEG_ONE; // also caught by rustc
 
-error: aborting due to 11 previous errors
+error: aborting due to 10 previous errors
 
---
To only update this specific test, also pass `--test-args modulo_one.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/modulo_one.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/modulo_one.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-ff0575c5a2ca7cc8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7064d3f8532fb2a5.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-06539012b522c0cf.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-8e5c09d8dc991112.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-90129c5dba212efa.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-17d0421109b348d4.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-54a7fffacad7d378.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/modulo_one.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this arithmetic operation will overflow","code":{"code":"arithmetic_overflow","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/modulo_one.rs","byte_start":214,"byte_end":229,"line_start":11,"line_end":11,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    i32::MIN % (-1); // also caught by rustc","highlight_start":5,"highlight_end":20}],"label":"attempt to compute the remainder of `i32::MIN % -1_i32`, which would overflow","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`#[deny(arithmetic_overflow)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: this arithmetic operation will overflow\n  --> tests/ui/modulo_one.rs:11:5\n   |\nLL |     i32::MIN % (-1); // also caught by rustc\n   |     ^^^^^^^^^^^^^^^ attempt to compute the remainder of `i32::MIN % -1_i32`, which would overflow\n   |\n   = note: `#[deny(arithmetic_overflow)]` on by default\n\n"}
{"message":"this arithmetic operation will overflow","code":{"code":"arithmetic_overflow","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/modulo_one.rs","byte_start":474,"byte_end":491,"line_start":21,"line_end":21,"column_start":5,"column_end":22,"is_primary":true,"text":[{"text":"    INT_MIN % NEG_ONE; // also caught by rustc","highlight_start":5,"highlight_end":22}],"label":"attempt to compute the remainder of `i64::MIN % -1_i64`, which would overflow","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this arithmetic operation will overflow\n  --> tests/ui/modulo_one.rs:21:5\n   |\nLL |     INT_MIN % NEG_ONE; // also caught by rustc\n   |     ^^^^^^^^^^^^^^^^^ attempt to compute the remainder of `i64::MIN % -1_i64`, which would overflow\n\n"}
{"message":"this arithmetic operation will overflow","code":{"code":"arithmetic_overflow","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/modulo_one.rs","byte_start":521,"byte_end":545,"line_start":22,"line_end":22,"column_start":5,"column_end":29,"is_primary":true,"text":[{"text":"    INT_MIN % STATIC_NEG_ONE; // ONLY caught by rustc","highlight_start":5,"highlight_end":29}],"label":"attempt to compute the remainder of `i64::MIN % -1_i64`, which would overflow","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this arithmetic operation will overflow\n  --> tests/ui/modulo_one.rs:22:5\n   |\nLL |     INT_MIN % STATIC_NEG_ONE; // ONLY caught by rustc\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^ attempt to compute the remainder of `i64::MIN % -1_i64`, which would overflow\n\n"}
{"message":"any number modulo 1 will be 0","code":{"code":"clippy::modulo_one","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/modulo_one.rs","byte_start":177,"byte_end":183,"line_start":8,"line_end":8,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    10 % 1;","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::modulo-one` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: any number modulo 1 will be 0\n  --> tests/ui/modulo_one.rs:8:5\n   |\nLL |     10 % 1;\n   |     ^^^^^^\n   |\n   = note: `-D clippy::modulo-one` implied by `-D warnings`\n\n"}
{"message":"any number modulo -1 will panic/overflow or result in 0","code":{"code":"clippy::modulo_one","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/modulo_one.rs","byte_start":189,"byte_end":196,"line_start":9,"line_end":9,"column_start":5,"column_end":12,"is_primary":true,"text":[{"text":"    10 % -1;","highlight_start":5,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: any number modulo -1 will panic/overflow or result in 0\n  --> tests/ui/modulo_one.rs:9:5\n   |\nLL |     10 % -1;\n   |     ^^^^^^^\n\n"}
{"message":"any number modulo -1 will panic/overflow or result in 0","code":{"code":"clippy::modulo_one","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/modulo_one.rs","byte_start":214,"byte_end":229,"line_start":11,"line_end":11,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    i32::MIN % (-1); // also caught by rustc","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: any number modulo -1 will panic/overflow or result in 0\n  --> tests/ui/modulo_one.rs:11:5\n   |\nLL |     i32::MIN % (-1); // also caught by rustc\n   |     ^^^^^^^^^^^^^^^\n\n"}
{"message":"the operation is ineffective. Consider reducing it to `1`","code":{"code":"clippy::identity_op","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/modulo_one.rs","byte_start":277,"byte_end":282,"line_start":13,"line_end":13,"column_start":22,"column_end":27,"is_primary":true,"text":[{"text":"    const ONE: u32 = 1 * 1;","highlight_start":22,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::identity-op` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: the operation is ineffective. Consider reducing it to `1`\n  --> tests/ui/modulo_one.rs:13:22\n   |\nLL |     const ONE: u32 = 1 * 1;\n   |                      ^^^^^\n   |\n   = note: `-D clippy::identity-op` implied by `-D warnings`\n\n"}
{"message":"any number modulo 1 will be 0","code":{"code":"clippy::modulo_one","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/modulo_one.rs","byte_start":356,"byte_end":363,"line_start":17,"line_end":17,"column_start":5,"column_end":12,"is_primary":true,"text":[{"text":"    2 % ONE;","highlight_start":5,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: any number modulo 1 will be 0\n  --> tests/ui/modulo_one.rs:17:5\n   |\nLL |     2 % ONE;\n   |     ^^^^^^^\n\n"}
{"message":"any number modulo -1 will panic/overflow or result in 0","code":{"code":"clippy::modulo_one","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/modulo_one.rs","byte_start":411,"byte_end":422,"line_start":19,"line_end":19,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    2 % NEG_ONE;","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: any number modulo -1 will panic/overflow or result in 0\n  --> tests/ui/modulo_one.rs:19:5\n   |\nLL |     2 % NEG_ONE;\n   |     ^^^^^^^^^^^\n\n"}
{"message":"any number modulo -1 will panic/overflow or result in 0","code":{"code":"clippy::modulo_one","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/modulo_one.rs","byte_start":474,"byte_end":491,"line_start":21,"line_end":21,"column_start":5,"column_end":22,"is_primary":true,"text":[{"text":"    INT_MIN % NEG_ONE; // also caught by rustc","highlight_start":5,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: any number modulo -1 will panic/overflow or result in 0\n  --> tests/ui/modulo_one.rs:21:5\n   |\nLL |     INT_MIN % NEG_ONE; // also caught by rustc\n   |     ^^^^^^^^^^^^^^^^^\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: this sequence of operators looks suspiciously like a bug
    |
    |
 LL |         self.x == other.y && self.y == other.y && self.z == other.z
    |         ^^^^^^^^^^^^^^^^^ help: did you mean: `self.x == other.x`
    |
    = note: `-D clippy::suspicious-operation-groupings` implied by `-D warnings`
 
 error: this sequence of operators looks suspiciously like a bug
-   |
-   |
-LL |         self.x == other.y && self.y == other.y && self.z == other.z
-   |         ^^^^^^^^^^^^^^^^^ help: did you mean: `self.x == other.x`
-
-error: this sequence of operators looks suspiciously like a bug
    |
    |
 LL |     s1.a < s2.a && s1.a < s2.b
    |                    ^^^^^^^^^^^ help: did you mean: `s1.b < s2.b`
 
 error: this sequence of operators looks suspiciously like a bug
    |
    |
 LL |     s1.a * s2.a + s1.b * s2.b + s1.c * s2.b + s1.d * s2.d
    |                                 ^^^^^^^^^^^ help: did you mean: `s1.c * s2.c`
 
 error: this sequence of operators looks suspiciously like a bug
    |
    |
 LL |     s1.a * s2.a + s1.b * s2.c + s1.c * s2.c
    |                   ^^^^^^^^^^^ help: did you mean: `s1.b * s2.b`
 
 error: this sequence of operators looks suspiciously like a bug
    |
    |
 LL |     s1.a * s2.a + s1.b * s2.c + s1.c * s2.c
    |                   ^^^^^^^^^^^ help: did you mean: `s1.b * s2.b`
 
 error: this sequence of operators looks suspiciously like a bug
    |
    |
 LL |     s1.a * s2.a + s2.b * s2.b + s1.c * s2.c
    |                   ^^^^^^^^^^^ help: did you mean: `s1.b * s2.b`
 
 error: this sequence of operators looks suspiciously like a bug
    |
    |
 LL |     s1.a * s2.a + s1.b * s1.b + s1.c * s2.c
    |                   ^^^^^^^^^^^ help: did you mean: `s1.b * s2.b`
 
 error: this sequence of operators looks suspiciously like a bug
    |
    |
 LL |     s1.a * s1.a + s1.b * s2.b + s1.c * s2.c
    |     ^^^^^^^^^^^ help: did you mean: `s1.a * s2.a`
 
 error: this sequence of operators looks suspiciously like a bug
    |
    |
 LL |     s1.a * s2.a + s1.b * s2.b + s1.c * s1.c
    |                                 ^^^^^^^^^^^ help: did you mean: `s1.c * s2.c`
 
 error: this sequence of operators looks suspiciously like a bug
    |
    |
 LL |     (s1.a * s2.a + s1.b * s1.b)
    |                    ^^^^^^^^^^^ help: did you mean: `s1.b * s2.b`
 
 error: this sequence of operators looks suspiciously like a bug
    |
    |
 LL |     (s1.a * s2.a + s1.b * s2.b + s1.c * s2.b + s1.d * s2.d)
    |                                  ^^^^^^^^^^^ help: did you mean: `s1.c * s2.c`
 
 error: this sequence of operators looks suspiciously like a bug
    |
    |
 LL |     (s1.a * s2.a) + (s1.b * s2.b) + (s1.c * s2.b) + (s1.d * s2.d)
    |                                      ^^^^^^^^^^^ help: did you mean: `s1.c * s2.c`
 
 error: this sequence of operators looks suspiciously like a bug
    |
    |
 LL |     ((s1.a * s2.a) + (s1.b * s2.b) + (s1.c * s2.b) + (s1.d * s2.d))
    |                                       ^^^^^^^^^^^ help: did you mean: `s1.c * s2.c`
 
 error: this sequence of operators looks suspiciously like a bug
    |
    |
 LL |     (((s1.a * s2.a) + (s1.b * s2.b)) + ((s1.c * s2.b) + (s1.d * s2.d)))
    |                                          ^^^^^^^^^^^ help: did you mean: `s1.c * s2.c`
 
 error: this sequence of operators looks suspiciously like a bug
    |
    |
 LL |     (((s1.a * s2.a) + (s1.b * s2.b)) + ((s1.c * s2.b) + (s1.d * s2.d)))
    |                                          ^^^^^^^^^^^ help: did you mean: `s1.c * s2.c`
 
 error: this sequence of operators looks suspiciously like a bug
    |
    |
 LL |     (((s1.a * s2.a) + (s1.b * s2.b) + (s1.c * s2.b)) + (s1.d * s2.d))
    |                                        ^^^^^^^^^^^ help: did you mean: `s1.c * s2.c`
 
 error: this sequence of operators looks suspiciously like a bug
    |
    |
 LL |     ((s1.a * s2.a) + ((s1.b * s2.b) + (s1.c * s2.b) + (s1.d * s2.d)))
    |                                        ^^^^^^^^^^^ help: did you mean: `s1.c * s2.c`
 
 error: this sequence of operators looks suspiciously like a bug
    |
    |
 LL |     (s1.a * s2.a + s2.b * s2.b) / 2
    |                    ^^^^^^^^^^^ help: did you mean: `s1.b * s2.b`
 
 error: this sequence of operators looks suspiciously like a bug
    |
    |
 LL |     i32::swap_bytes(s1.a * s2.a + s2.b * s2.b)
    |                                   ^^^^^^^^^^^ help: did you mean: `s1.b * s2.b`
 
 error: this sequence of operators looks suspiciously like a bug
    |
    |
 LL |     s1.a > 0 && s1.b > 0 && s1.d == s2.c && s1.d == s2.d
    |                             ^^^^^^^^^^^^ help: did you mean: `s1.c == s2.c`
 
 error: this sequence of operators looks suspiciously like a bug
    |
    |
 LL |     s1.a > 0 && s1.d == s2.c && s1.b > 0 && s1.d == s2.d
    |                 ^^^^^^^^^^^^ help: did you mean: `s1.c == s2.c`
 
 error: this sequence of operators looks suspiciously like a bug
    |
    |
 LL |     (n1.inner.0).0 == (n2.inner.0).0 && (n1.inner.1).0 == (n2.inner.1).0 && (n1.inner.2).0 == (n2.inner.1).0
    |                                                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: did you mean: `(n1.inner.2).0 == (n2.inner.2).0`
 
 error: this sequence of operators looks suspiciously like a bug
    |
    |
 LL |         s1.a <= s2.a && s1.a <= s2.b
    |                         ^^^^^^^^^^^^ help: did you mean: `s1.b <= s2.b`
 
 error: this sequence of operators looks suspiciously like a bug
    |
    |
 LL |     if s1.a < s2.a && s1.a < s2.b {
    |                       ^^^^^^^^^^^ help: did you mean: `s1.b < s2.b`
 
 error: this sequence of operators looks suspiciously like a bug
    |
    |
 LL |     -(-(-s1.a * -s2.a) + (-(-s1.b * -s2.b) + -(-s1.c * -s2.b) + -(-s1.d * -s2.d)))
    |                                                ^^^^^^^^^^^^^ help: did you mean: `-s1.c * -s2.c`
 
 error: this sequence of operators looks suspiciously like a bug
    |
    |
 LL |     -(if -s1.a < -s2.a && -s1.a < -s2.b { s1.c } else { s2.a })
    |                           ^^^^^^^^^^^^^ help: did you mean: `-s1.b < -s2.b`
-error: aborting due to 27 previous errors
+error: aborting due to 26 previous errors
 
 
---
To only update this specific test, also pass `--test-args suspicious_operation_groupings.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/suspicious_operation_groupings.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/suspicious_operation_groupings.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-ff0575c5a2ca7cc8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7064d3f8532fb2a5.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-06539012b522c0cf.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-8e5c09d8dc991112.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-90129c5dba212efa.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-17d0421109b348d4.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-54a7fffacad7d378.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/suspicious_operation_groupings.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this sequence of operators looks suspiciously like a bug","code":{"code":"clippy::suspicious_operation_groupings","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":281,"byte_end":298,"line_start":14,"line_end":14,"column_start":9,"column_end":26,"is_primary":true,"text":[{"text":"        self.x == other.y && self.y == other.y && self.z == other.z","highlight_start":9,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::suspicious-operation-groupings` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"did you mean","code":null,"level":"help","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":281,"byte_end":298,"line_start":14,"line_end":14,"column_start":9,"column_end":26,"is_primary":true,"text":[{"text":"        self.x == other.y && self.y == other.y && self.z == other.z","highlight_start":9,"highlight_end":26}],"label":null,"suggested_replacement":"self.x == other.x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this sequence of operators looks suspiciously like a bug\n  --> tests/ui/suspicious_operation_groupings.rs:14:9\n   |\nLL |         self.x == other.y && self.y == other.y && self.z == other.z\n   |         ^^^^^^^^^^^^^^^^^ help: did you mean: `self.x == other.x`\n   |\n   = note: `-D clippy::suspicious-operation-groupings` implied by `-D warnings`\n\n"}
{"message":"this sequence of operators looks suspiciously like a bug","code":{"code":"clippy::suspicious_operation_groupings","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":498,"byte_end":509,"line_start":27,"line_end":27,"column_start":20,"column_end":31,"is_primary":true,"text":[{"text":"    s1.a < s2.a && s1.a < s2.b","highlight_start":20,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean","code":null,"level":"help","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":498,"byte_end":509,"line_start":27,"line_end":27,"column_start":20,"column_end":31,"is_primary":true,"text":[{"text":"    s1.a < s2.a && s1.a < s2.b","highlight_start":20,"highlight_end":31}],"label":null,"suggested_replacement":"s1.b < s2.b","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this sequence of operators looks suspiciously like a bug\n  --> tests/ui/suspicious_operation_groupings.rs:27:20\n   |\nLL |     s1.a < s2.a && s1.a < s2.b\n   |                    ^^^^^^^^^^^ help: did you mean: `s1.b < s2.b`\n\n"}
{"message":"this sequence of operators looks suspiciously like a bug","code":{"code":"clippy::suspicious_operation_groupings","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":1694,"byte_end":1705,"line_start":75,"line_end":75,"column_start":33,"column_end":44,"is_primary":true,"text":[{"text":"    s1.a * s2.a + s1.b * s2.b + s1.c * s2.b + s1.d * s2.d","highlight_start":33,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean","code":null,"level":"help","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":1694,"byte_end":1705,"line_start":75,"line_end":75,"column_start":33,"column_end":44,"is_primary":true,"text":[{"text":"    s1.a * s2.a + s1.b * s2.b + s1.c * s2.b + s1.d * s2.d","highlight_start":33,"highlight_end":44}],"label":null,"suggested_replacement":"s1.c * s2.c","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this sequence of operators looks suspiciously like a bug\n  --> tests/ui/suspicious_operation_groupings.rs:75:33\n   |\nLL |     s1.a * s2.a + s1.b * s2.b + s1.c * s2.b + s1.d * s2.d\n   |                                 ^^^^^^^^^^^ help: did you mean: `s1.c * s2.c`\n\n"}
{"message":"this sequence of operators looks suspiciously like a bug","code":{"code":"clippy::suspicious_operation_groupings","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":1814,"byte_end":1825,"line_start":80,"line_end":80,"column_start":19,"column_end":30,"is_primary":true,"text":[{"text":"    s1.a * s2.a + s1.b * s2.c + s1.c * s2.c","highlight_start":19,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean","code":null,"level":"help","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":1814,"byte_end":1825,"line_start":80,"line_end":80,"column_start":19,"column_end":30,"is_primary":true,"text":[{"text":"    s1.a * s2.a + s1.b * s2.c + s1.c * s2.c","highlight_start":19,"highlight_end":30}],"label":null,"suggested_replacement":"s1.b * s2.b","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this sequence of operators looks suspiciously like a bug\n  --> tests/ui/suspicious_operation_groupings.rs:80:19\n   |\nLL |     s1.a * s2.a + s1.b * s2.c + s1.c * s2.c\n   |                   ^^^^^^^^^^^ help: did you mean: `s1.b * s2.b`\n\n"}
{"message":"this sequence of operators looks suspiciously like a bug","code":{"code":"clippy::suspicious_operation_groupings","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":1814,"byte_end":1825,"line_start":80,"line_end":80,"column_start":19,"column_end":30,"is_primary":true,"text":[{"text":"    s1.a * s2.a + s1.b * s2.c + s1.c * s2.c","highlight_start":19,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean","code":null,"level":"help","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":1814,"byte_end":1825,"line_start":80,"line_end":80,"column_start":19,"column_end":30,"is_primary":true,"text":[{"text":"    s1.a * s2.a + s1.b * s2.c + s1.c * s2.c","highlight_start":19,"highlight_end":30}],"label":null,"suggested_replacement":"s1.b * s2.b","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this sequence of operators looks suspiciously like a bug\n  --> tests/ui/suspicious_operation_groupings.rs:80:19\n   |\nLL |     s1.a * s2.a + s1.b * s2.c + s1.c * s2.c\n   |                   ^^^^^^^^^^^ help: did you mean: `s1.b * s2.b`\n\n"}
{"message":"this sequence of operators looks suspiciously like a bug","code":{"code":"clippy::suspicious_operation_groupings","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":1953,"byte_end":1964,"line_start":85,"line_end":85,"column_start":19,"column_end":30,"is_primary":true,"text":[{"text":"    s1.a * s2.a + s2.b * s2.b + s1.c * s2.c","highlight_start":19,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean","code":null,"level":"help","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":1953,"byte_end":1964,"line_start":85,"line_end":85,"column_start":19,"column_end":30,"is_primary":true,"text":[{"text":"    s1.a * s2.a + s2.b * s2.b + s1.c * s2.c","highlight_start":19,"highlight_end":30}],"label":null,"suggested_replacement":"s1.b * s2.b","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this sequence of operators looks suspiciously like a bug\n  --> tests/ui/suspicious_operation_groupings.rs:85:19\n   |\nLL |     s1.a * s2.a + s2.b * s2.b + s1.c * s2.c\n   |                   ^^^^^^^^^^^ help: did you mean: `s1.b * s2.b`\n\n"}
{"message":"this sequence of operators looks suspiciously like a bug","code":{"code":"clippy::suspicious_operation_groupings","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":2093,"byte_end":2104,"line_start":90,"line_end":90,"column_start":19,"column_end":30,"is_primary":true,"text":[{"text":"    s1.a * s2.a + s1.b * s1.b + s1.c * s2.c","highlight_start":19,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean","code":null,"level":"help","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":2093,"byte_end":2104,"line_start":90,"line_end":90,"column_start":19,"column_end":30,"is_primary":true,"text":[{"text":"    s1.a * s2.a + s1.b * s1.b + s1.c * s2.c","highlight_start":19,"highlight_end":30}],"label":null,"suggested_replacement":"s1.b * s2.b","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this sequence of operators looks suspiciously like a bug\n  --> tests/ui/suspicious_operation_groupings.rs:90:19\n   |\nLL |     s1.a * s2.a + s1.b * s1.b + s1.c * s2.c\n   |                   ^^^^^^^^^^^ help: did you mean: `s1.b * s2.b`\n\n"}
{"message":"this sequence of operators looks suspiciously like a bug","code":{"code":"clippy::suspicious_operation_groupings","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":2205,"byte_end":2216,"line_start":95,"line_end":95,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    s1.a * s1.a + s1.b * s2.b + s1.c * s2.c","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean","code":null,"level":"help","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":2205,"byte_end":2216,"line_start":95,"line_end":95,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    s1.a * s1.a + s1.b * s2.b + s1.c * s2.c","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":"s1.a * s2.a","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this sequence of operators looks suspiciously like a bug\n  --> tests/ui/suspicious_operation_groupings.rs:95:5\n   |\nLL |     s1.a * s1.a + s1.b * s2.b + s1.c * s2.c\n   |     ^^^^^^^^^^^ help: did you mean: `s1.a * s2.a`\n\n"}
{"message":"this sequence of operators looks suspiciously like a bug","code":{"code":"clippy::suspicious_operation_groupings","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":2357,"byte_end":2368,"line_start":100,"line_end":100,"column_start":33,"column_end":44,"is_primary":true,"text":[{"text":"    s1.a * s2.a + s1.b * s2.b + s1.c * s1.c","highlight_start":33,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean","code":null,"level":"help","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":2357,"byte_end":2368,"line_start":100,"line_end":100,"column_start":33,"column_end":44,"is_primary":true,"text":[{"text":"    s1.a * s2.a + s1.b * s2.b + s1.c * s1.c","highlight_start":33,"highlight_end":44}],"label":null,"suggested_replacement":"s1.c * s2.c","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this sequence of operators looks suspiciously like a bug\n  --> tests/ui/suspicious_operation_groupings.rs:100:33\n   |\nLL |     s1.a * s2.a + s1.b * s2.b + s1.c * s1.c\n   |                                 ^^^^^^^^^^^ help: did you mean: `s1.c * s2.c`\n\n"}
{"message":"this sequence of operators looks suspiciously like a bug","code":{"code":"clippy::suspicious_operation_groupings","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":2658,"byte_end":2669,"line_start":113,"line_end":113,"column_start":20,"column_end":31,"is_primary":true,"text":[{"text":"    (s1.a * s2.a + s1.b * s1.b)","highlight_start":20,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean","code":null,"level":"help","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":2658,"byte_end":2669,"line_start":113,"line_end":113,"column_start":20,"column_end":31,"is_primary":true,"text":[{"text":"    (s1.a * s2.a + s1.b * s1.b)","highlight_start":20,"highlight_end":31}],"label":null,"suggested_replacement":"s1.b * s2.b","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this sequence of operators looks suspiciously like a bug\n  --> tests/ui/suspicious_operation_groupings.rs:113:20\n   |\nLL |     (s1.a * s2.a + s1.b * s1.b)\n   |                    ^^^^^^^^^^^ help: did you mean: `s1.b * s2.b`\n\n"}
{"message":"this sequence of operators looks suspiciously like a bug","code":{"code":"clippy::suspicious_operation_groupings","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":2773,"byte_end":2784,"line_start":118,"line_end":118,"column_start":34,"column_end":45,"is_primary":true,"text":[{"text":"    (s1.a * s2.a + s1.b * s2.b + s1.c * s2.b + s1.d * s2.d)","highlight_start":34,"highlight_end":45}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean","code":null,"level":"help","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":2773,"byte_end":2784,"line_start":118,"line_end":118,"column_start":34,"column_end":45,"is_primary":true,"text":[{"text":"    (s1.a * s2.a + s1.b * s2.b + s1.c * s2.b + s1.d * s2.d)","highlight_start":34,"highlight_end":45}],"label":null,"suggested_replacement":"s1.c * s2.c","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this sequence of operators looks suspiciously like a bug\n  --> tests/ui/suspicious_operation_groupings.rs:118:34\n   |\nLL |     (s1.a * s2.a + s1.b * s2.b + s1.c * s2.b + s1.d * s2.d)\n   |                                  ^^^^^^^^^^^ help: did you mean: `s1.c * s2.c`\n\n"}
{"message":"this sequence of operators looks suspiciously like a bug","code":{"code":"clippy::suspicious_operation_groupings","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":2906,"byte_end":2917,"line_start":123,"line_end":123,"column_start":38,"column_end":49,"is_primary":true,"text":[{"text":"    (s1.a * s2.a) + (s1.b * s2.b) + (s1.c * s2.b) + (s1.d * s2.d)","highlight_start":38,"highlight_end":49}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean","code":null,"level":"help","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":2906,"byte_end":2917,"line_start":123,"line_end":123,"column_start":38,"column_end":49,"is_primary":true,"text":[{"text":"    (s1.a * s2.a) + (s1.b * s2.b) + (s1.c * s2.b) + (s1.d * s2.d)","highlight_start":38,"highlight_end":49}],"label":null,"suggested_replacement":"s1.c * s2.c","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this sequence of operators looks suspiciously like a bug\n  --> tests/ui/suspicious_operation_groupings.rs:123:38\n   |\nLL |     (s1.a * s2.a) + (s1.b * s2.b) + (s1.c * s2.b) + (s1.d * s2.d)\n   |                                      ^^^^^^^^^^^ help: did you mean: `s1.c * s2.c`\n\n"}
{"message":"this sequence of operators looks suspiciously like a bug","code":{"code":"clippy::suspicious_operation_groupings","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":3057,"byte_end":3068,"line_start":128,"line_end":128,"column_start":39,"column_end":50,"is_primary":true,"text":[{"text":"    ((s1.a * s2.a) + (s1.b * s2.b) + (s1.c * s2.b) + (s1.d * s2.d))","highlight_start":39,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean","code":null,"level":"help","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":3057,"byte_end":3068,"line_start":128,"line_end":128,"column_start":39,"column_end":50,"is_primary":true,"text":[{"text":"    ((s1.a * s2.a) + (s1.b * s2.b) + (s1.c * s2.b) + (s1.d * s2.d))","highlight_start":39,"highlight_end":50}],"label":null,"suggested_replacement":"s1.c * s2.c","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this sequence of operators looks suspiciously like a bug\n  --> tests/ui/suspicious_operation_groupings.rs:128:39\n   |\nLL |     ((s1.a * s2.a) + (s1.b * s2.b) + (s1.c * s2.b) + (s1.d * s2.d))\n   |                                       ^^^^^^^^^^^ help: did you mean: `s1.c * s2.c`\n\n"}
{"message":"this sequence of operators looks suspiciously like a bug","code":{"code":"clippy::suspicious_operation_groupings","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":3209,"byte_end":3220,"line_start":133,"line_end":133,"column_start":42,"column_end":53,"is_primary":true,"text":[{"text":"    (((s1.a * s2.a) + (s1.b * s2.b)) + ((s1.c * s2.b) + (s1.d * s2.d)))","highlight_start":42,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean","code":null,"level":"help","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":3209,"byte_end":3220,"line_start":133,"line_end":133,"column_start":42,"column_end":53,"is_primary":true,"text":[{"text":"    (((s1.a * s2.a) + (s1.b * s2.b)) + ((s1.c * s2.b) + (s1.d * s2.d)))","highlight_start":42,"highlight_end":53}],"label":null,"suggested_replacement":"s1.c * s2.c","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this sequence of operators looks suspiciously like a bug\n  --> tests/ui/suspicious_operation_groupings.rs:133:42\n   |\nLL |     (((s1.a * s2.a) + (s1.b * s2.b)) + ((s1.c * s2.b) + (s1.d * s2.d)))\n   |                                          ^^^^^^^^^^^ help: did you mean: `s1.c * s2.c`\n\n"}
{"message":"this sequence of operators looks suspiciously like a bug","code":{"code":"clippy::suspicious_operation_groupings","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":3209,"byte_end":3220,"line_start":133,"line_end":133,"column_start":42,"column_end":53,"is_primary":true,"text":[{"text":"    (((s1.a * s2.a) + (s1.b * s2.b)) + ((s1.c * s2.b) + (s1.d * s2.d)))","highlight_start":42,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean","code":null,"level":"help","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":3209,"byte_end":3220,"line_start":133,"line_end":133,"column_start":42,"column_end":53,"is_primary":true,"text":[{"text":"    (((s1.a * s2.a) + (s1.b * s2.b)) + ((s1.c * s2.b) + (s1.d * s2.d)))","highlight_start":42,"highlight_end":53}],"label":null,"suggested_replacement":"s1.c * s2.c","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this sequence of operators looks suspiciously like a bug\n  --> tests/ui/suspicious_operation_groupings.rs:133:42\n   |\nLL |     (((s1.a * s2.a) + (s1.b * s2.b)) + ((s1.c * s2.b) + (s1.d * s2.d)))\n   |                                          ^^^^^^^^^^^ help: did you mean: `s1.c * s2.c`\n\n"}
{"message":"this sequence of operators looks suspiciously like a bug","code":{"code":"clippy::suspicious_operation_groupings","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":3356,"byte_end":3367,"line_start":138,"line_end":138,"column_start":40,"column_end":51,"is_primary":true,"text":[{"text":"    (((s1.a * s2.a) + (s1.b * s2.b) + (s1.c * s2.b)) + (s1.d * s2.d))","highlight_start":40,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean","code":null,"level":"help","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":3356,"byte_end":3367,"line_start":138,"line_end":138,"column_start":40,"column_end":51,"is_primary":true,"text":[{"text":"    (((s1.a * s2.a) + (s1.b * s2.b) + (s1.c * s2.b)) + (s1.d * s2.d))","highlight_start":40,"highlight_end":51}],"label":null,"suggested_replacement":"s1.c * s2.c","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this sequence of operators looks suspiciously like a bug\n  --> tests/ui/suspicious_operation_groupings.rs:138:40\n   |\nLL |     (((s1.a * s2.a) + (s1.b * s2.b) + (s1.c * s2.b)) + (s1.d * s2.d))\n   |                                        ^^^^^^^^^^^ help: did you mean: `s1.c * s2.c`\n\n"}
{"message":"this sequence of operators looks suspiciously like a bug","code":{"code":"clippy::suspicious_operation_groupings","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":3504,"byte_end":3515,"line_start":143,"line_end":143,"column_start":40,"column_end":51,"is_primary":true,"text":[{"text":"    ((s1.a * s2.a) + ((s1.b * s2.b) + (s1.c * s2.b) + (s1.d * s2.d)))","highlight_start":40,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean","code":null,"level":"help","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":3504,"byte_end":3515,"line_start":143,"line_end":143,"column_start":40,"column_end":51,"is_primary":true,"text":[{"text":"    ((s1.a * s2.a) + ((s1.b * s2.b) + (s1.c * s2.b) + (s1.d * s2.d)))","highlight_start":40,"highlight_end":51}],"label":null,"suggested_replacement":"s1.c * s2.c","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this sequence of operators looks suspiciously like a bug\n  --> tests/ui/suspicious_operation_groupings.rs:143:40\n   |\nLL |     ((s1.a * s2.a) + ((s1.b * s2.b) + (s1.c * s2.b) + (s1.d * s2.d)))\n   |                                        ^^^^^^^^^^^ help: did you mean: `s1.c * s2.c`\n\n"}
{"message":"this sequence of operators looks suspiciously like a bug","code":{"code":"clippy::suspicious_operation_groupings","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":3640,"byte_end":3651,"line_start":148,"line_end":148,"column_start":20,"column_end":31,"is_primary":true,"text":[{"text":"    (s1.a * s2.a + s2.b * s2.b) / 2","highlight_start":20,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean","code":null,"level":"help","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":3640,"byte_end":3651,"line_start":148,"line_end":148,"column_start":20,"column_end":31,"is_primary":true,"text":[{"text":"    (s1.a * s2.a + s2.b * s2.b) / 2","highlight_start":20,"highlight_end":31}],"label":null,"suggested_replacement":"s1.b * s2.b","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this sequence of operators looks suspiciously like a bug\n  --> tests/ui/suspicious_operation_groupings.rs:148:20\n   |\nLL |     (s1.a * s2.a + s2.b * s2.b) / 2\n   |                    ^^^^^^^^^^^ help: did you mean: `s1.b * s2.b`\n\n"}
{"message":"this sequence of operators looks suspiciously like a bug","code":{"code":"clippy::suspicious_operation_groupings","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":3768,"byte_end":3779,"line_start":153,"line_end":153,"column_start":35,"column_end":46,"is_primary":true,"text":[{"text":"    i32::swap_bytes(s1.a * s2.a + s2.b * s2.b)","highlight_start":35,"highlight_end":46}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean","code":null,"level":"help","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":3768,"byte_end":3779,"line_start":153,"line_end":153,"column_start":35,"column_end":46,"is_primary":true,"text":[{"text":"    i32::swap_bytes(s1.a * s2.a + s2.b * s2.b)","highlight_start":35,"highlight_end":46}],"label":null,"suggested_replacement":"s1.b * s2.b","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this sequence of operators looks suspiciously like a bug\n  --> tests/ui/suspicious_operation_groupings.rs:153:35\n   |\nLL |     i32::swap_bytes(s1.a * s2.a + s2.b * s2.b)\n   |                                   ^^^^^^^^^^^ help: did you mean: `s1.b * s2.b`\n\n"}
{"message":"this sequence of operators looks suspiciously like a bug","code":{"code":"clippy::suspicious_operation_groupings","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":3899,"byte_end":3911,"line_start":158,"line_end":158,"column_start":29,"column_end":41,"is_primary":true,"text":[{"text":"    s1.a > 0 && s1.b > 0 && s1.d == s2.c && s1.d == s2.d","highlight_start":29,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean","code":null,"level":"help","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":3899,"byte_end":3911,"line_start":158,"line_end":158,"column_start":29,"column_end":41,"is_primary":true,"text":[{"text":"    s1.a > 0 && s1.b > 0 && s1.d == s2.c && s1.d == s2.d","highlight_start":29,"highlight_end":41}],"label":null,"suggested_replacement":"s1.c == s2.c","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this sequence of operators looks suspiciously like a bug\n  --> tests/ui/suspicious_operation_groupings.rs:158:29\n   |\nLL |     s1.a > 0 && s1.b > 0 && s1.d == s2.c && s1.d == s2.d\n   |                             ^^^^^^^^^^^^ help: did you mean: `s1.c == s2.c`\n\n"}
{"message":"this sequence of operators looks suspiciously like a bug","code":{"code":"clippy::suspicious_operation_groupings","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":4052,"byte_end":4064,"line_start":163,"line_end":163,"column_start":17,"column_end":29,"is_primary":true,"text":[{"text":"    s1.a > 0 && s1.d == s2.c && s1.b > 0 && s1.d == s2.d","highlight_start":17,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean","code":null,"level":"help","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":4052,"byte_end":4064,"line_start":163,"line_end":163,"column_start":17,"column_end":29,"is_primary":true,"text":[{"text":"    s1.a > 0 && s1.d == s2.c && s1.b > 0 && s1.d == s2.d","highlight_start":17,"highlight_end":29}],"label":null,"suggested_replacement":"s1.c == s2.c","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this sequence of operators looks suspiciously like a bug\n  --> tests/ui/suspicious_operation_groupings.rs:163:17\n   |\nLL |     s1.a > 0 && s1.d == s2.c && s1.b > 0 && s1.d == s2.d\n   |                 ^^^^^^^^^^^^ help: did you mean: `s1.c == s2.c`\n\n"}
{"message":"this sequence of operators looks suspiciously like a bug","code":{"code":"clippy::suspicious_operation_groupings","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":4321,"byte_end":4353,"line_start":172,"line_end":172,"column_start":77,"column_end":109,"is_primary":true,"text":[{"text":"    (n1.inner.0).0 == (n2.inner.0).0 && (n1.inner.1).0 == (n2.inner.1).0 && (n1.inner.2).0 == (n2.inner.1).0","highlight_start":77,"highlight_end":109}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean","code":null,"level":"help","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":4321,"byte_end":4353,"line_start":172,"line_end":172,"column_start":77,"column_end":109,"is_primary":true,"text":[{"text":"    (n1.inner.0).0 == (n2.inner.0).0 && (n1.inner.1).0 == (n2.inner.1).0 && (n1.inner.2).0 == (n2.inner.1).0","highlight_start":77,"highlight_end":109}],"label":null,"suggested_replacement":"(n1.inner.2).0 == (n2.inner.2).0","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this sequence of operators looks suspiciously like a bug\n  --> tests/ui/suspicious_operation_groupings.rs:172:77\n   |\nLL |     (n1.inner.0).0 == (n2.inner.0).0 && (n1.inner.1).0 == (n2.inner.1).0 && (n1.inner.2).0 == (n2.inner.1).0\n   |                                                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: did you mean: `(n1.inner.2).0 == (n2.inner.2).0`\n\n"}
{"message":"this sequence of operators looks suspiciously like a bug","code":{"code":"clippy::suspicious_operation_groupings","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":4813,"byte_end":4825,"line_start":186,"line_end":186,"column_start":25,"column_end":37,"is_primary":true,"text":[{"text":"        s1.a <= s2.a && s1.a <= s2.b","highlight_start":25,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean","code":null,"level":"help","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":4813,"byte_end":4825,"line_start":186,"line_end":186,"column_start":25,"column_end":37,"is_primary":true,"text":[{"text":"        s1.a <= s2.a && s1.a <= s2.b","highlight_start":25,"highlight_end":37}],"label":null,"suggested_replacement":"s1.b <= s2.b","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this sequence of operators looks suspiciously like a bug\n  --> tests/ui/suspicious_operation_groupings.rs:186:25\n   |\nLL |         s1.a <= s2.a && s1.a <= s2.b\n   |                         ^^^^^^^^^^^^ help: did you mean: `s1.b <= s2.b`\n\n"}
{"message":"this sequence of operators looks suspiciously like a bug","code":{"code":"clippy::suspicious_operation_groupings","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":4926,"byte_end":4937,"line_start":192,"line_end":192,"column_start":23,"column_end":34,"is_primary":true,"text":[{"text":"    if s1.a < s2.a && s1.a < s2.b {","highlight_start":23,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean","code":null,"level":"help","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":4926,"byte_end":4937,"line_start":192,"line_end":192,"column_start":23,"column_end":34,"is_primary":true,"text":[{"text":"    if s1.a < s2.a && s1.a < s2.b {","highlight_start":23,"highlight_end":34}],"label":null,"suggested_replacement":"s1.b < s2.b","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this sequence of operators looks suspiciously like a bug\n  --> tests/ui/suspicious_operation_groupings.rs:192:23\n   |\nLL |     if s1.a < s2.a && s1.a < s2.b {\n   |                       ^^^^^^^^^^^ help: did you mean: `s1.b < s2.b`\n\n"}
{"message":"this sequence of operators looks suspiciously like a bug","code":{"code":"clippy::suspicious_operation_groupings","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":5101,"byte_end":5114,"line_start":199,"line_end":199,"column_start":48,"column_end":61,"is_primary":true,"text":[{"text":"    -(-(-s1.a * -s2.a) + (-(-s1.b * -s2.b) + -(-s1.c * -s2.b) + -(-s1.d * -s2.d)))","highlight_start":48,"highlight_end":61}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean","code":null,"level":"help","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":5101,"byte_end":5114,"line_start":199,"line_end":199,"column_start":48,"column_end":61,"is_primary":true,"text":[{"text":"    -(-(-s1.a * -s2.a) + (-(-s1.b * -s2.b) + -(-s1.c * -s2.b) + -(-s1.d * -s2.d)))","highlight_start":48,"highlight_end":61}],"label":null,"suggested_replacement":"-s1.c * -s2.c","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this sequence of operators looks suspiciously like a bug\n  --> tests/ui/suspicious_operation_groupings.rs:199:48\n   |\nLL |     -(-(-s1.a * -s2.a) + (-(-s1.b * -s2.b) + -(-s1.c * -s2.b) + -(-s1.d * -s2.d)))\n   |                                                ^^^^^^^^^^^^^ help: did you mean: `-s1.c * -s2.c`\n\n"}
{"message":"this sequence of operators looks suspiciously like a bug","code":{"code":"clippy::suspicious_operation_groupings","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":5252,"byte_end":5265,"line_start":204,"line_end":204,"column_start":27,"column_end":40,"is_primary":true,"text":[{"text":"    -(if -s1.a < -s2.a && -s1.a < -s2.b { s1.c } else { s2.a })","highlight_start":27,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean","code":null,"level":"help","spans":[{"file_name":"tests/ui/suspicious_operation_groupings.rs","byte_start":5252,"byte_end":5265,"line_start":204,"line_end":204,"column_start":27,"column_end":40,"is_primary":true,"text":[{"text":"    -(if -s1.a < -s2.a && -s1.a < -s2.b { s1.c } else { s2.a })","highlight_start":27,"highlight_end":40}],"label":null,"suggested_replacement":"-s1.b < -s2.b","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this sequence of operators looks suspiciously like a bug\n  --> tests/ui/suspicious_operation_groupings.rs:204:27\n   |\nLL |     -(if -s1.a < -s2.a && -s1.a < -s2.b { s1.c } else { s2.a })\n   |                           ^^^^^^^^^^^^^ help: did you mean: `-s1.b < -s2.b`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.6.0/src/lib.rs:105:22
