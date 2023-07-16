plain
[00:47:52] 
[00:47:52] ---- [ui] ui/transmute/main.rs stdout ----
[00:47:52] diff of stderr:
[00:47:52] 
[00:47:52] 22 LL |     let x: u8 = transmute("test"); //~ ERROR cannot transmute between types of different sizes
[00:47:52] 24    |
[00:47:52] 24    |
[00:47:52] -    = note: source type: `&str` ($STR bits)
[00:47:52] +    = note: source type: `&str` (64 bits)
[00:47:52] 26    = note: target type: `u8` (8 bits)
[00:47:52] 27 
[00:47:52] 28 error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
[00:47:52] 
[00:47:52] The actual stderr differed from the expected stderr.
[00:47:52] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmute/main/main.stderr
[00:47:52] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmute/main/main.stderr
[00:47:52] To update references, rerun the tests and pass the `--bless` flag
[00:47:52] To only update this specific test, also pass `--test-args transmute/main.rs`
[00:47:52] error: 1 errors occurred comparing output.
[00:47:52] status: exit code: 1
[00:47:52] status: exit code: 1
[00:47:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/transmute/main.rs" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmute/main/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmute/main/auxiliary" "-A" "unused"
[00:47:52] ------------------------------------------
[00:47:52] 
[00:47:52] ------------------------------------------
[00:47:52] stderr:
[00:47:52] stderr:
[00:47:52] ------------------------------------------
[00:47:52] {"message":"cannot transmute between types of different sizes, or dependently-sized types","code":{"code":"E0512","explanation":"\nTransmute with two differently sized types was attempted. Erroneous code\nexample:\n\n