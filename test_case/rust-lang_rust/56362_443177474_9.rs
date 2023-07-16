\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-pattern-irrefutable.rs","byte_start":675,"byte_end":676,"line_start":23,"line_end":23,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let c = 4; //~ ERROR refutable pattern in local binding: `_` not covered","highlight_start":9,"highlight_end":10}],"label":"interpreted as a consta^
[00:51:08] 12 
[00:51:08] 12 
[00:51:08] 13 error[E0004]: non-exhaustive patterns: `128u8..=255u8` not covered
[00:51:08] -   --> $DIR/exhaustive_integer_patterns.rs:37:11
[00:51:08] +   --> $DIR/exhaustive_integer_patterns.rs:38:11
[00:51:08] 15    |
[00:51:08] 16 LL |     match x { //~ ERROR non-exhaustive patterns
[00:51:08] 17    |           ^ pattern `128u8..=255u8` not covered
[00:51:08] 18 
[00:51:08] 18 
[00:51:08] 19 error[E0004]: non-exhaustive patterns: `11u8..=19u8`, `31u8..=34u8`, `36u8..=69u8` and 1 more not covered
[00:51:08] -   --> $DIR/exhaustive_integer_patterns.rs:42:11
[00:51:08] +   --> $DIR/exhaustive_integer_patterns.rs:43:11
[00:51:08] 21    |
[00:51:08] 22 LL |     match x { //~ ERROR non-exhaustive patterns
[00:51:08] 23    |           ^ patterns `11u8..=19u8`, `31u8..=34u8`, `36u8..=69u8` and 1 more not covered
[00:51:08] 24 
[00:51:08] 25 error: unreachable pattern
[00:51:08] -   --> $DIR/exhaustive_integer_patterns.rs:53:9
[00:51:08] +   --> $DIR/exhaustive_integer_patterns.rs:54:9
[00:51:08] +   --> $DIR/exhaustive_integer_patterns.rs:54:9
[00:51:08] 27    |
[00:51:08] 28 LL |         -2..=20 => {} //~ ERROR unreachable pattern
[00:51:08] 
[00:51:08] 30 
[00:51:08] 30 
[00:51:08] 31 error[E0004]: non-exhaustive patterns: `-128i8..=-8i8`, `-6i8`, `121i8..=124i8` and 1 more not covered
[00:51:08] -   --> $DIR/exhaustive_integer_patterns.rs:50:11
[00:51:08] +   --> $DIR/exhaustive_integer_patterns.rs:51:11
[00:51:08] 33    |
[00:51:08] 34 LL |     match x { //~ ERROR non-exhaustive patterns
[00:51:08] 35    |           ^ patterns `-128i8..=-8i8`, `-6i8`, `121i8..=124i8` and 1 more not covered
[00:51:08] 36 
[00:51:08] 36 
[00:51:08] 37 error[E0004]: non-exhaustive patterns: `-128i8` not covered
[00:51:08] -   --> $DIR/exhaustive_integer_patterns.rs:99:11
[00:51:08] +   --> $DIR/exhaustive_integer_patterns.rs:100:11
[00:51:08] 39    |
[00:51:08] 40 LL |     match 0i8 { //~ ERROR non-exhaustive patterns
[00:51:08] 41    |           ^^^ pattern `-128i8` not covered
[00:51:08] 42 
[00:51:08] 43 error[E0004]: non-exhaustive patterns: `0i16` not covered
[00:51:08] -   --> $DIR/exhaustive_integer_patterns.rs:107:11
[00:51:08] +   --> $DIR/exhaustive_integer_patterns.rs:108:11
[00:51:08] +   --> $DIR/exhaustive_integer_patterns.rs:108:11
[00:51:08] 45    |
[00:51:08] 46 LL |     match 0i16 { //~ ERROR non-exhaustive patterns
[00:51:08] 47    |           ^^^^ pattern `0i16` not covered
[00:51:08] 48 
[00:51:08] 48 
[00:51:08] 49 error[E0004]: non-exhaustive patterns: `128u8..=255u8` not covered
[00:51:08] -   --> $DIR/exhaustive_integer_patterns.rs:125:11
[00:51:08] +   --> $DIR/exhaustive_integer_patterns.rs:126:11
[00:51:08] 51    |
[00:51:08] 52 LL |     match 0u8 { //~ ERROR non-exhaustive patterns
[00:51:08] 53    |           ^^^ pattern `128u8..=255u8` not covered
[00:51:08] 54 
[00:51:08] 54 
[00:51:08] 55 error[E0004]: non-exhaustive patterns: `(0u8, Some(_))` and `(2u8..=255u8, Some(_))` not covered
[00:51:08] -   --> $DIR/exhaustive_integer_patterns.rs:137:11
[00:51:08] +   --> $DIR/exhaustive_integer_patterns.rs:138:11
[00:51:08] 57    |
[00:51:08] 58 LL |     match (0u8, Some(())) { //~ ERROR non-exhaustive patterns
[00:51:08] 59    |           ^^^^^^^^^^^^^^^ patterns `(0u8, Some(_))` and `(2u8..=255u8, Some(_))` not covered
[00:51:08] 60 
[00:51:08] 60 
[00:51:08] 61 error[E0004]: non-exhaustive patterns: `(126u8..=127u8, false)` not covered
[00:51:08] -   --> $DIR/exhaustive_integer_patterns.rs:142:11
[00:51:08] +   --> $DIR/exhaustive_integer_patterns.rs:143:11
[00:51:08] 63    |
[00:51:08] 64 LL |     match (0u8, true) { //~ ERROR non-exhaustive patterns
[00:51:08] 65    |           ^^^^^^^^^^^ pattern `(126u8..=127u8, false)` not covered
[00:51:08] 66 
[00:51:08] 67 error[E0004]: non-exhaustive patterns: `340282366920938463463374607431768211455u128` not covered
[00:51:08] -   --> $DIR/exhaustive_integer_patterns.rs:162:11
[00:51:08] +   --> $DIR/exhaustive_integer_patterns.rs:163:11
[00:51:08] +   --> $DIR/exhaustive_integer_patterns.rs:163:11
[00:51:08] 69    |
[00:51:08] 70 LL |     match 0u128 { //~ ERROR non-exhaustive patterns
[00:51:08] 71    |           ^^^^^ pattern `340282366920938463463374607431768211455u128` not covered
[00:51:08] 72 
[00:51:08] 72 
[00:51:08] 73 error[E0004]: non-exhaustive patterns: `5u128..=340282366920938463463374607431768211455u128` not covered
[00:51:08] -   --> $DIR/exhaustive_integer_patterns.rs:166:11
[00:51:08] +   --> $DIR/exhaustive_integer_patterns.rs:167:11
[00:51:08] 75    |
[00:51:08] 76 LL |     match 0u128 { //~ ERROR non-exhaustive patterns
[00:51:08] 77    |           ^^^^^ pattern `5u128..=340282366920938463463374607431768211455u128` not covered
[00:51:08] 78 
[00:51:08] 78 
[00:51:08] 79 error[E0004]: non-exhaustive patterns: `0u128..=3u128` not covered
[00:51:08] -   --> $DIR/exhaustive_integer_patterns.rs:170:11
[00:51:08] +   --> $DIR/exhaustive_integer_patterns.rs:171:11
[00:51:08] 81    |
[00:51:08] 82 LL |     match 0u128 { //~ ERROR non-exhaustive patterns
[00:51:08] 83    |           ^^^^^ pattern `0u128..=3u128` not covered
[00:51:08] 
[00:51:08] The actual stderr differed from the expected stderr.
[00:51:08] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/exhaustive_integer_patterns/exhaustive_integer_patterns.stderr
[00:51:08] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/exhaustive_integer_patterns/exhaustive_integer_patterns.stderr
[00:51:08] To update references, rerun the tests and pass the `--bless` flag
[00:51:08] To only update this specific test, also pass `--test-args exhaustive_integer_patterns.rs`
[00:51:08] error: 1 errors occurred comparing output.
[00:51:08] status: exit code: 1
[00:51:08] status: exit code: 1
[00:51:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/exhaustive_integer_patterns.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/exhaustive_integer_patterns/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/exhaustive_integer_patterns/auxiliary" "-A" "unused"
[00:51:08] ------------------------------------------
[00:51:08] 
[00:51:08] ------------------------------------------
[00:51:08] stderr:
[00:51:08] stderr:
[00:51:08] ------------------------------------------
[00:51:08] {"message":"unreachable pattern","code":{"code":"unreachable_patterns","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/exhaustive_integer_patterns.rs","byte_start":1004,"byte_end":1007,"line_start":33,"line_end // error: non-exhaustive patterns: `HastaLaVistaBaby` not covered\n    Terminator::TalkToMyHand => {}\n}\n