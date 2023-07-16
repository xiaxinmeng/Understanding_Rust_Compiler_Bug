plain
[00:52:07] 
[00:52:07] ---- [ui] ui/transmute/main.rs stdout ----
[00:52:07] diff of stderr:
[00:52:07] 
[00:52:07] 22 LL |     let x: u8 = transmute("test"); //~ ERROR cannot transmute between types of different sizes
[00:52:07] 24    |
[00:52:07] 24    |
[00:52:07] -    = note: source type: `&str` (128 bits)
[00:52:07] +    = note: source type: `&str` (64 bits)
[00:52:07] 26    = note: target type: `u8` (8 bits)
[00:52:07] 27 
[00:52:07] 28 error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
[00:52:07] 
[00:52:07] The actual stderr differed from the expected stderr.
[00:52:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmute/main/main.stderr
[00:52:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmute/main/main.stderr
[00:52:07] To update references, rerun the tests and pass the `--bless` flag
[00:52:07] To only update this specific test, also pass `--test-args transmute/main.rs`
[00:52:07] error: 1 errors occurred comparing output.
[00:52:07] status: exit code: 1
[00:52:07] status: exit code: 1
[00:52:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/transmute/main.rs" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmute/main/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmute/main/auxiliary" "-A" "unused"
[00:52:07] ------------------------------------------
[00:52:07] 
[00:52:07] ------------------------------------------
[00:52:07] stderr:
[00:52:07] stderr:
[00:52:07] ------------------------------------------
[00:52:07] {"message":"cannot transmute between types of different sizes, or dependently-sized types","code":{"code":"E0512","explanation":"\nTransmute with two differently sized types was attempted. Erroneous code\nexample:\n\n