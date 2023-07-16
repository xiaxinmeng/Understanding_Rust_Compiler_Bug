plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between addacb5878b9970ebc1665768a05cb601e7aea15 and 573ea37dc0028676ffa9053dd89e9232c2bd9340
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

---- compile_test stdout ----
diff of stderr:

 error: you are iterating over `Iterator::next()` which is an Option; this will compile but is probably not what you want
    |
    |
 LL |     for _v in vec.iter().next() {}
    |
    |
    = note: `-D clippy::iter-next-loop` implied by `-D warnings`
-error: aborting due to previous error
-error: aborting due to previous error
+error: for loop over an `Option`. This is more readably written as an `if let` statement
+   |
+   |
+LL |     for _v in vec.iter().next() {}
+   |
+   |
+   = note: `-D for-loop-over-fallibles` implied by `-D warnings`
+help: to iterate over `vec.iter()` remove the call to `next`
+   |
+LL |     for _v in vec.iter().by_ref() {}
+   |                         ~~~~~~~~~
+help: consider using `if let` to clear intent
+   |
+LL |     if let Some(_v) = vec.iter().next() {}
+   |     ~~~~~~~~~~~~  ~~~
+error: aborting due to 2 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/for_loop_unfixable.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args for_loop_unfixable.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/for_loop_unfixable.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/for_loop_unfixable.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-11c942eb60796e9d.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-021aec868151835c.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-04f014bd62aa87c5.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-b0e96f2e9d30bd37.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-507b29393c1a728f.so" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-7dc368fb32eb8aae.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-36709515b9cb16b6.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-0c795f7a8756f15a.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-3e103f3c7cb1e342.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-56bce9bcc023120a.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-cdd893c121eb00e4.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-71205fa4273edf27.so" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/for_loop_unfixable.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"you are iterating over `Iterator::next()` which is an Option; this will compile but is probably not what you want","code":{"code":"clippy::iter_next_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loop_unfixable.rs","byte_start":362,"byte_end":379,"line_start":14,"line_end":14,"column_start":15,"column_end":32,"is_primary":true,"text":[{"text":"    for _v in vec.iter().next() {}","highlight_start":15,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::iter-next-loop` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: you are iterating over `Iterator::next()` which is an Option; this will compile but is probably not what you want\n  --> tests/ui/for_loop_unfixable.rs:14:15\n   |\nLL |     for _v in vec.iter().next() {}\n   |               ^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::iter-next-loop` implied by `-D warnings`\n\n"}
{"message":"for loop over an `Option`. This is more readably written as an `if let` statement","code":{"code":"for_loop_over_fallibles","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loop_unfixable.rs","byte_start":362,"byte_end":379,"line_start":14,"line_end":14,"column_start":15,"column_end":32,"is_primary":true,"text":[{"text":"    for _v in vec.iter().next() {}","highlight_start":15,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D for-loop-over-fallibles` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"to iterate over `vec.iter()` remove the call to `next`","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loop_unfixable.rs","byte_start":372,"byte_end":379,"line_start":14,"line_end":14,"column_start":25,"column_end":32,"is_primary":true,"text":[{"text":"    for _v in vec.iter().next() {}","highlight_start":25,"highlight_end":32}],"label":null,"suggested_replacement":".by_ref()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null},{"message":"consider using `if let` to clear intent","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loop_unfixable.rs","byte_start":352,"byte_end":356,"line_start":14,"line_end":14,"column_start":5,"column_end":9,"is_primary":true,"text":[{"text":"    for _v in vec.iter().next() {}","highlight_start":5,"highlight_end":9}],"label":null,"suggested_replacement":"if let Some(","suggestion_applicability":"MaybeIncorrect","expansion":{"span":{"file_name":"tests/ui/for_loop_unfixable.rs","byte_start":352,"byte_end":382,"line_start":14,"line_end":14,"column_start":5,"column_end":35,"is_primary":false,"text":[{"text":"    for _v in vec.iter().next() {}","highlight_start":5,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/for_loop_unfixable.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"tests/ui/for_loop_unfixable.rs","byte_start":358,"byte_end":362,"line_start":14,"line_end":14,"column_start":11,"column_end":15,"is_primary":true,"text":[{"text":"    for _v in vec.iter().next() {}","highlight_start":11,"highlight_end":15}],"label":null,"suggested_replacement":") = ","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: for loop over an `Option`. This is more readably written as an `if let` statement\n  --> tests/ui/for_loop_unfixable.rs:14:15\n   |\nLL |     for _v in vec.iter().next() {}\n   |               ^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D for-loop-over-fallibles` implied by `-D warnings`\nhelp: to iterate over `vec.iter()` remove the call to `next`\n   |\nLL |     for _v in vec.iter().by_ref() {}\n   |                         ~~~~~~~~~\nhelp: consider using `if let` to clear intent\n   |\nLL |     if let Some(_v) = vec.iter().next() {}\n   |     ~~~~~~~~~~~~  ~~~\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: for loop over `option`, which is an `Option`. This is more readably written as an `if let` statement
    |
 LL |     for x in option {
    |              ^^^^^^
    |
    |
    = note: `-D clippy::for-loops-over-fallibles` implied by `-D warnings`
    = help: consider replacing `for x in option` with `if let Some(x) = option`
 
 error: for loop over `option`, which is an `Option`. This is more readably written as an `if let` statement
    |
 LL |     for x in option.iter() {
    |              ^^^^^^
    |
    |
    = help: consider replacing `for x in option.iter()` with `if let Some(x) = option`
 
 error: for loop over `result`, which is a `Result`. This is more readably written as an `if let` statement
    |
 LL |     for x in result {
    |              ^^^^^^
    |
    |
    = help: consider replacing `for x in result` with `if let Ok(x) = result`
 
 error: for loop over `result`, which is a `Result`. This is more readably written as an `if let` statement
    |
 LL |     for x in result.iter_mut() {
    |              ^^^^^^
    |
    |
    = help: consider replacing `for x in result.iter_mut()` with `if let Ok(x) = result`
 
 error: for loop over `result`, which is a `Result`. This is more readably written as an `if let` statement
    |
 LL |     for x in result.into_iter() {
    |              ^^^^^^
    |
    |
    = help: consider replacing `for x in result.into_iter()` with `if let Ok(x) = result`
 
 error: for loop over `option.ok_or("x not found")`, which is a `Result`. This is more readably written as an `if let` statement
    |
    |
 LL |     for x in option.ok_or("x not found") {
    |
    |
    = help: consider replacing `for x in option.ok_or("x not found")` with `if let Ok(x) = option.ok_or("x not found")`
 
 error: you are iterating over `Iterator::next()` which is an Option; this will compile but is probably not what you want
    |
    |
 LL |     for x in v.iter().next() {
    |
    |
    = note: `#[deny(clippy::iter_next_loop)]` on by default
 
 error: for loop over `v.iter().next().and(Some(0))`, which is an `Option`. This is more readably written as an `if let` statement
    |
    |
 LL |     for x in v.iter().next().and(Some(0)) {
    |
    |
    = help: consider replacing `for x in v.iter().next().and(Some(0))` with `if let Some(x) = v.iter().next().and(Some(0))`
 
 error: for loop over `v.iter().next().ok_or("x not found")`, which is a `Result`. This is more readably written as an `if let` statement
    |
    |
 LL |     for x in v.iter().next().ok_or("x not found") {
    |
    |
    = help: consider replacing `for x in v.iter().next().ok_or("x not found")` with `if let Ok(x) = v.iter().next().ok_or("x not found")`
 error: this loop never actually loops
   --> $DIR/for_loops_over_fallibles.rs:60:5
    |
    |
 LL | /     while let Some(x) = option {
 LL | |         println!("{}", x);
 LL | |         break;
 LL | |     }
    |
    |
    = note: `#[deny(clippy::never_loop)]` on by default
 error: this loop never actually loops
   --> $DIR/for_loops_over_fallibles.rs:66:5
    |
    |
 LL | /     while let Ok(x) = result {
 LL | |         println!("{}", x);
 LL | |         break;
 LL | |     }
 
-error: aborting due to 11 previous errors
-error: aborting due to 11 previous errors
+error: for loop over an `Option`. This is more readably written as an `if let` statement
+   |
+LL |     for x in option {
+   |              ^^^^^^
+   |
+   |
+   = note: `-D for-loop-over-fallibles` implied by `-D warnings`
+help: to check pattern in a loop use `while let`
+   |
+LL |     while let Some(x) = option {
+   |     ~~~~~~~~~~~~~~~ ~~~
+help: consider using `if let` to clear intent
+   |
+LL |     if let Some(x) = option {
+
+
+error: for loop over a `Result`. This is more readably written as an `if let` statement
+   |
+LL |     for x in result {
+   |              ^^^^^^
+   |
+   |
+help: to check pattern in a loop use `while let`
+   |
+LL |     while let Ok(x) = result {
+   |     ~~~~~~~~~~~~~ ~~~
+help: consider using `if let` to clear intent
+   |
+LL |     if let Ok(x) = result {
+
+
+error: for loop over a `Result`. This is more readably written as an `if let` statement
+   |
+   |
+LL |     for x in option.ok_or("x not found") {
+   |
+   |
+help: to check pattern in a loop use `while let`
+   |
+LL |     while let Ok(x) = option.ok_or("x not found") {
+   |     ~~~~~~~~~~~~~ ~~~
+help: consider using `if let` to clear intent
+   |
+LL |     if let Ok(x) = option.ok_or("x not found") {
+
+
+error: for loop over an `Option`. This is more readably written as an `if let` statement
+   |
+   |
+LL |     for x in v.iter().next() {
+   |
+   |
+help: to iterate over `v.iter()` remove the call to `next`
+   |
+LL |     for x in v.iter().by_ref() {
+   |                      ~~~~~~~~~
+help: consider using `if let` to clear intent
+   |
+LL |     if let Some(x) = v.iter().next() {
+
+
+error: for loop over an `Option`. This is more readably written as an `if let` statement
+   |
+   |
+LL |     for x in v.iter().next().and(Some(0)) {
+   |
+   |
+help: to check pattern in a loop use `while let`
+   |
+LL |     while let Some(x) = v.iter().next().and(Some(0)) {
+   |     ~~~~~~~~~~~~~~~ ~~~
+help: consider using `if let` to clear intent
+   |
+LL |     if let Some(x) = v.iter().next().and(Some(0)) {
+
+
+error: for loop over a `Result`. This is more readably written as an `if let` statement
+   |
+   |
+LL |     for x in v.iter().next().ok_or("x not found") {
+   |
+   |
+help: to check pattern in a loop use `while let`
+   |
+LL |     while let Ok(x) = v.iter().next().ok_or("x not found") {
+   |     ~~~~~~~~~~~~~ ~~~
+help: consider using `if let` to clear intent
+   |
+LL |     if let Ok(x) = v.iter().next().ok_or("x not found") {
+
+error: aborting due to 17 previous errors
 
 
---
To only update this specific test, also pass `--test-args for_loops_over_fallibles.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/for_loops_over_fallibles.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/for_loops_over_fallibles.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-11c942eb60796e9d.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-021aec868151835c.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-04f014bd62aa87c5.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-b0e96f2e9d30bd37.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-507b29393c1a728f.so" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-7dc368fb32eb8aae.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-36709515b9cb16b6.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-0c795f7a8756f15a.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-3e103f3c7cb1e342.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-56bce9bcc023120a.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-cdd893c121eb00e4.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-71205fa4273edf27.so" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/for_loops_over_fallibles.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"for loop over `option`, which is an `Option`. This is more readably written as an `if let` statement","code":{"code":"clippy::for_loops_over_fallibles","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":223,"byte_end":229,"line_start":9,"line_end":9,"column_start":14,"column_end":20,"is_primary":true,"text":[{"text":"    for x in option {","highlight_start":14,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::for-loops-over-fallibles` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider replacing `for x in option` with `if let Some(x) = option`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: for loop over `option`, which is an `Option`. This is more readably written as an `if let` statement\n  --> tests/ui/for_loops_over_fallibles.rs:9:14\n   |\nLL |     for x in option {\n   |              ^^^^^^\n   |\n   = note: `-D clippy::for-loops-over-fallibles` implied by `-D warnings`\n   = help: consider replacing `for x in option` with `if let Some(x) = option`\n\n"}
{"message":"for loop over `option`, which is an `Option`. This is more readably written as an `if let` statement","code":{"code":"clippy::for_loops_over_fallibles","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":309,"byte_end":315,"line_start":14,"line_end":14,"column_start":14,"column_end":20,"is_primary":true,"text":[{"text":"    for x in option.iter() {","highlight_start":14,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider replacing `for x in option.iter()` with `if let Some(x) = option`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: for loop over `option`, which is an `Option`. This is more readably written as an `if let` statement\n  --> tests/ui/for_loops_over_fallibles.rs:14:14\n   |\nLL |     for x in option.iter() {\n   |              ^^^^^^\n   |\n   = help: consider replacing `for x in option.iter()` with `if let Some(x) = option`\n\n"}
{"message":"for loop over `result`, which is a `Result`. This is more readably written as an `if let` statement","code":{"code":"clippy::for_loops_over_fallibles","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":401,"byte_end":407,"line_start":19,"line_end":19,"column_start":14,"column_end":20,"is_primary":true,"text":[{"text":"    for x in result {","highlight_start":14,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider replacing `for x in result` with `if let Ok(x) = result`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: for loop over `result`, which is a `Result`. This is more readably written as an `if let` statement\n  --> tests/ui/for_loops_over_fallibles.rs:19:14\n   |\nLL |     for x in result {\n   |              ^^^^^^\n   |\n   = help: consider replacing `for x in result` with `if let Ok(x) = result`\n\n"}
{"message":"for loop over `result`, which is a `Result`. This is more readably written as an `if let` statement","code":{"code":"clippy::for_loops_over_fallibles","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":486,"byte_end":492,"line_start":24,"line_end":24,"column_start":14,"column_end":20,"is_primary":true,"text":[{"text":"    for x in result.iter_mut() {","highlight_start":14,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider replacing `for x in result.iter_mut()` with `if let Ok(x) = result`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: for loop over `result`, which is a `Result`. This is more readably written as an `if let` statement\n  --> tests/ui/for_loops_over_fallibles.rs:24:14\n   |\nLL |     for x in result.iter_mut() {\n   |              ^^^^^^\n   |\n   = help: consider replacing `for x in result.iter_mut()` with `if let Ok(x) = result`\n\n"}
{"message":"for loop over `result`, which is a `Result`. This is more readably written as an `if let` statement","code":{"code":"clippy::for_loops_over_fallibles","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":582,"byte_end":588,"line_start":29,"line_end":29,"column_start":14,"column_end":20,"is_primary":true,"text":[{"text":"    for x in result.into_iter() {","highlight_start":14,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider replacing `for x in result.into_iter()` with `if let Ok(x) = result`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: for loop over `result`, which is a `Result`. This is more readably written as an `if let` statement\n  --> tests/ui/for_loops_over_fallibles.rs:29:14\n   |\nLL |     for x in result.into_iter() {\n   |              ^^^^^^\n   |\n   = help: consider replacing `for x in result.into_iter()` with `if let Ok(x) = result`\n\n"}
{"message":"for loop over `option.ok_or(\"x not found\")`, which is a `Result`. This is more readably written as an `if let` statement","code":{"code":"clippy::for_loops_over_fallibles","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":650,"byte_end":677,"line_start":33,"line_end":33,"column_start":14,"column_end":41,"is_primary":true,"text":[{"text":"    for x in option.ok_or(\"x not found\") {","highlight_start":14,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider replacing `for x in option.ok_or(\"x not found\")` with `if let Ok(x) = option.ok_or(\"x not found\")`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: for loop over `option.ok_or(\"x not found\")`, which is a `Result`. This is more readably written as an `if let` statement\n  --> tests/ui/for_loops_over_fallibles.rs:33:14\n   |\nLL |     for x in option.ok_or(\"x not found\") {\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider replacing `for x in option.ok_or(\"x not found\")` with `if let Ok(x) = option.ok_or(\"x not found\")`\n\n"}
{"message":"you are iterating over `Iterator::next()` which is an Option; this will compile but is probably not what you want","code":{"code":"clippy::iter_next_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":838,"byte_end":853,"line_start":39,"line_end":39,"column_start":14,"column_end":29,"is_primary":true,"text":[{"text":"    for x in v.iter().next() {","highlight_start":14,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`#[deny(clippy::iter_next_loop)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: you are iterating over `Iterator::next()` which is an Option; this will compile but is probably not what you want\n  --> tests/ui/for_loops_over_fallibles.rs:39:14\n   |\nLL |     for x in v.iter().next() {\n   |              ^^^^^^^^^^^^^^^\n   |\n   = note: `#[deny(clippy::iter_next_loop)]` on by default\n\n"}
{"message":"for loop over `v.iter().next().and(Some(0))`, which is an `Option`. This is more readably written as an `if let` statement","code":{"code":"clippy::for_loops_over_fallibles","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":974,"byte_end":1002,"line_start":44,"line_end":44,"column_start":14,"column_end":42,"is_primary":true,"text":[{"text":"    for x in v.iter().next().and(Some(0)) {","highlight_start":14,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider replacing `for x in v.iter().next().and(Some(0))` with `if let Some(x) = v.iter().next().and(Some(0))`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: for loop over `v.iter().next().and(Some(0))`, which is an `Option`. This is more readably written as an `if let` statement\n  --> tests/ui/for_loops_over_fallibles.rs:44:14\n   |\nLL |     for x in v.iter().next().and(Some(0)) {\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider replacing `for x in v.iter().next().and(Some(0))` with `if let Some(x) = v.iter().next().and(Some(0))`\n\n"}
{"message":"for loop over `v.iter().next().ok_or(\"x not found\")`, which is a `Result`. This is more readably written as an `if let` statement","code":{"code":"clippy::for_loops_over_fallibles","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":1052,"byte_end":1088,"line_start":48,"line_end":48,"column_start":14,"column_end":50,"is_primary":true,"text":[{"text":"    for x in v.iter().next().ok_or(\"x not found\") {","highlight_start":14,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider replacing `for x in v.iter().next().ok_or(\"x not found\")` with `if let Ok(x) = v.iter().next().ok_or(\"x not found\")`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: for loop over `v.iter().next().ok_or(\"x not found\")`, which is a `Result`. This is more readably written as an `if let` statement\n  --> tests/ui/for_loops_over_fallibles.rs:48:14\n   |\nLL |     for x in v.iter().next().ok_or(\"x not found\") {\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider replacing `for x in v.iter().next().ok_or(\"x not found\")` with `if let Ok(x) = v.iter().next().ok_or(\"x not found\")`\n\n"}
{"message":"this loop never actually loops","code":{"code":"clippy::never_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":1288,"byte_end":1364,"line_start":60,"line_end":63,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    while let Some(x) = option {","highlight_start":5,"highlight_end":33},{"text":"        println!(\"{}\", x);","highlight_start":1,"highlight_end":27},{"text":"        break;","highlight_start":1,"highlight_end":15},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`#[deny(clippy::never_loop)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: this loop never actually loops\n  --> tests/ui/for_loops_over_fallibles.rs:60:5\n   |\nLL | /     while let Some(x) = option {\nLL | |         println!(\"{}\", x);\nLL | |         break;\nLL | |     }\n   | |_____^\n   |\n   = note: `#[deny(clippy::never_loop)]` on by default\n\n"}
{"message":"this loop never actually loops","code":{"code":"clippy::never_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":1413,"byte_end":1487,"line_start":66,"line_end":69,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    while let Ok(x) = result {","highlight_start":5,"highlight_end":31},{"text":"        println!(\"{}\", x);","highlight_start":1,"highlight_end":27},{"text":"        break;","highlight_start":1,"highlight_end":15},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this loop never actually loops\n  --> tests/ui/for_loops_over_fallibles.rs:66:5\n   |\nLL | /     while let Ok(x) = result {\nLL | |         println!(\"{}\", x);\nLL | |         break;\nLL | |     }\n   | |_____^\n\n"}
{"message":"for loop over an `Option`. This is more readably written as an `if let` statement","code":{"code":"for_loop_over_fallibles","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":223,"byte_end":229,"line_start":9,"line_end":9,"column_start":14,"column_end":20,"is_primary":true,"text":[{"text":"    for x in option {","highlight_start":14,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D for-loop-over-fallibles` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"to check pattern in a loop use `while let`","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":214,"byte_end":218,"line_start":9,"line_end":9,"column_start":5,"column_end":9,"is_primary":true,"text":[{"text":"    for x in option {","highlight_start":5,"highlight_end":9}],"label":null,"suggested_replacement":"while let Some(","suggestion_applicability":"MaybeIncorrect","expansion":{"span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":214,"byte_end":264,"line_start":9,"line_end":11,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for x in option {","highlight_start":5,"highlight_end":22},{"text":"        println!(\"{}\", x);","highlight_start":1,"highlight_end":27},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":219,"byte_end":223,"line_start":9,"line_end":9,"column_start":10,"column_end":14,"is_primary":true,"text":[{"text":"    for x in option {","highlight_start":10,"highlight_end":14}],"label":null,"suggested_replacement":") = ","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null},{"message":"consider using `if let` to clear intent","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":214,"byte_end":218,"line_start":9,"line_end":9,"column_start":5,"column_end":9,"is_primary":true,"text":[{"text":"    for x in option {","highlight_start":5,"highlight_end":9}],"label":null,"suggested_replacement":"if let Some(","suggestion_applicability":"MaybeIncorrect","expansion":{"span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":214,"byte_end":264,"line_start":9,"line_end":11,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for x in option {","highlight_start":5,"highlight_end":22},{"text":"        println!(\"{}\", x);","highlight_start":1,"highlight_end":27},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":219,"byte_end":223,"line_start":9,"line_end":9,"column_start":10,"column_end":14,"is_primary":true,"text":[{"text":"    for x in option {","highlight_start":10,"highlight_end":14}],"label":null,"suggested_replacement":") = ","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: for loop over an `Option`. This is more readably written as an `if let` statement\n  --> tests/ui/for_loops_over_fallibles.rs:9:14\n   |\nLL |     for x in option {\n   |              ^^^^^^\n   |\n   = note: `-D for-loop-over-fallibles` implied by `-D warnings`\nhelp: to check pattern in a loop use `while let`\n   |\nLL |     while let Some(x) = option {\n   |     ~~~~~~~~~~~~~~~ ~~~\nhelp: consider using `if let` to clear intent\n   |\nLL |     if let Some(x) = option {\n   |     ~~~~~~~~~~~~ ~~~\n\n"}
{"message":"for loop over a `Result`. This is more readably written as an `if let` statement","code":{"code":"for_loop_over_fallibles","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":401,"byte_end":407,"line_start":19,"line_end":19,"column_start":14,"column_end":20,"is_primary":true,"text":[{"text":"    for x in result {","highlight_start":14,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"to check pattern in a loop use `while let`","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":392,"byte_end":396,"line_start":19,"line_end":19,"column_start":5,"column_end":9,"is_primary":true,"text":[{"text":"    for x in result {","highlight_start":5,"highlight_end":9}],"label":null,"suggested_replacement":"while let Ok(","suggestion_applicability":"MaybeIncorrect","expansion":{"span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":392,"byte_end":442,"line_start":19,"line_end":21,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for x in result {","highlight_start":5,"highlight_end":22},{"text":"        println!(\"{}\", x);","highlight_start":1,"highlight_end":27},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":397,"byte_end":401,"line_start":19,"line_end":19,"column_start":10,"column_end":14,"is_primary":true,"text":[{"text":"    for x in result {","highlight_start":10,"highlight_end":14}],"label":null,"suggested_replacement":") = ","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null},{"message":"consider using `if let` to clear intent","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":392,"byte_end":396,"line_start":19,"line_end":19,"column_start":5,"column_end":9,"is_primary":true,"text":[{"text":"    for x in result {","highlight_start":5,"highlight_end":9}],"label":null,"suggested_replacement":"if let Ok(","suggestion_applicability":"MaybeIncorrect","expansion":{"span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":392,"byte_end":442,"line_start":19,"line_end":21,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for x in result {","highlight_start":5,"highlight_end":22},{"text":"        println!(\"{}\", x);","highlight_start":1,"highlight_end":27},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":397,"byte_end":401,"line_start":19,"line_end":19,"column_start":10,"column_end":14,"is_primary":true,"text":[{"text":"    for x in result {","highlight_start":10,"highlight_end":14}],"label":null,"suggested_replacement":") = ","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: for loop over a `Result`. This is more readably written as an `if let` statement\n  --> tests/ui/for_loops_over_fallibles.rs:19:14\n   |\nLL |     for x in result {\n   |              ^^^^^^\n   |\nhelp: to check pattern in a loop use `while let`\n   |\nLL |     while let Ok(x) = result {\n   |     ~~~~~~~~~~~~~ ~~~\nhelp: consider using `if let` to clear intent\n   |\nLL |     if let Ok(x) = result {\n   |     ~~~~~~~~~~ ~~~\n\n"}
{"message":"for loop over a `Result`. This is more readably written as an `if let` statement","code":{"code":"for_loop_over_fallibles","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":650,"byte_end":677,"line_start":33,"line_end":33,"column_start":14,"column_end":41,"is_primary":true,"text":[{"text":"    for x in option.ok_or(\"x not found\") {","highlight_start":14,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"to check pattern in a loop use `while let`","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":641,"byte_end":645,"line_start":33,"line_end":33,"column_start":5,"column_end":9,"is_primary":true,"text":[{"text":"    for x in option.ok_or(\"x not found\") {","highlight_start":5,"highlight_end":9}],"label":null,"suggested_replacement":"while let Ok(","suggestion_applicability":"MaybeIncorrect","expansion":{"span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":641,"byte_end":712,"line_start":33,"line_end":35,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for x in option.ok_or(\"x not found\") {","highlight_start":5,"highlight_end":43},{"text":"        println!(\"{}\", x);","highlight_start":1,"highlight_end":27},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":646,"byte_end":650,"line_start":33,"line_end":33,"column_start":10,"column_end":14,"is_primary":true,"text":[{"text":"    for x in option.ok_or(\"x not found\") {","highlight_start":10,"highlight_end":14}],"label":null,"suggested_replacement":") = ","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null},{"message":"consider using `if let` to clear intent","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":641,"byte_end":645,"line_start":33,"line_end":33,"column_start":5,"column_end":9,"is_primary":true,"text":[{"text":"    for x in option.ok_or(\"x not found\") {","highlight_start":5,"highlight_end":9}],"label":null,"suggested_replacement":"if let Ok(","suggestion_applicability":"MaybeIncorrect","expansion":{"span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":641,"byte_end":712,"line_start":33,"line_end":35,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for x in option.ok_or(\"x not found\") {","highlight_start":5,"highlight_end":43},{"text":"        println!(\"{}\", x);","highlight_start":1,"highlight_end":27},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":646,"byte_end":650,"line_start":33,"line_end":33,"column_start":10,"column_end":14,"is_primary":true,"text":[{"text":"    for x in option.ok_or(\"x not found\") {","highlight_start":10,"highlight_end":14}],"label":null,"suggested_replacement":") = ","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: for loop over a `Result`. This is more readably written as an `if let` statement\n  --> tests/ui/for_loops_over_fallibles.rs:33:14\n   |\nLL |     for x in option.ok_or(\"x not found\") {\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nhelp: to check pattern in a loop use `while let`\n   |\nLL |     while let Ok(x) = option.ok_or(\"x not found\") {\n   |     ~~~~~~~~~~~~~ ~~~\nhelp: consider using `if let` to clear intent\n   |\nLL |     if let Ok(x) = option.ok_or(\"x not found\") {\n   |     ~~~~~~~~~~ ~~~\n\n"}
{"message":"for loop over an `Option`. This is more readably written as an `if let` statement","code":{"code":"for_loop_over_fallibles","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":838,"byte_end":853,"line_start":39,"line_end":39,"column_start":14,"column_end":29,"is_primary":true,"text":[{"text":"    for x in v.iter().next() {","highlight_start":14,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"to iterate over `v.iter()` remove the call to `next`","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":846,"byte_end":853,"line_start":39,"line_end":39,"column_start":22,"column_end":29,"is_primary":true,"text":[{"text":"    for x in v.iter().next() {","highlight_start":22,"highlight_end":29}],"label":null,"suggested_replacement":".by_ref()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null},{"message":"consider using `if let` to clear intent","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":829,"byte_end":833,"line_start":39,"line_end":39,"column_start":5,"column_end":9,"is_primary":true,"text":[{"text":"    for x in v.iter().next() {","highlight_start":5,"highlight_end":9}],"label":null,"suggested_replacement":"if let Some(","suggestion_applicability":"MaybeIncorrect","expansion":{"span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":829,"byte_end":888,"line_start":39,"line_end":41,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for x in v.iter().next() {","highlight_start":5,"highlight_end":31},{"text":"        println!(\"{}\", x);","highlight_start":1,"highlight_end":27},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":834,"byte_end":838,"line_start":39,"line_end":39,"column_start":10,"column_end":14,"is_primary":true,"text":[{"text":"    for x in v.iter().next() {","highlight_start":10,"highlight_end":14}],"label":null,"suggested_replacement":") = ","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: for loop over an `Option`. This is more readably written as an `if let` statement\n  --> tests/ui/for_loops_over_fallibles.rs:39:14\n   |\nLL |     for x in v.iter().next() {\n   |              ^^^^^^^^^^^^^^^\n   |\nhelp: to iterate over `v.iter()` remove the call to `next`\n   |\nLL |     for x in v.iter().by_ref() {\n   |                      ~~~~~~~~~\nhelp: consider using `if let` to clear intent\n   |\nLL |     if let Some(x) = v.iter().next() {\n   |     ~~~~~~~~~~~~ ~~~\n\n"}
{"message":"for loop over an `Option`. This is more readably written as an `if let` statement","code":{"code":"for_loop_over_fallibles","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":974,"byte_end":1002,"line_start":44,"line_end":44,"column_start":14,"column_end":42,"is_primary":true,"text":[{"text":"    for x in v.iter().next().and(Some(0)) {","highlight_start":14,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"to check pattern in a loop use `while let`","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":965,"byte_end":969,"line_start":44,"line_end":44,"column_start":5,"column_end":9,"is_primary":true,"text":[{"text":"    for x in v.iter().next().and(Some(0)) {","highlight_start":5,"highlight_end":9}],"label":null,"suggested_replacement":"while let Some(","suggestion_applicability":"MaybeIncorrect","expansion":{"span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":965,"byte_end":1037,"line_start":44,"line_end":46,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for x in v.iter().next().and(Some(0)) {","highlight_start":5,"highlight_end":44},{"text":"        println!(\"{}\", x);","highlight_start":1,"highlight_end":27},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":970,"byte_end":974,"line_start":44,"line_end":44,"column_start":10,"column_end":14,"is_primary":true,"text":[{"text":"    for x in v.iter().next().and(Some(0)) {","highlight_start":10,"highlight_end":14}],"label":null,"suggested_replacement":") = ","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null},{"message":"consider using `if let` to clear intent","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":965,"byte_end":969,"line_start":44,"line_end":44,"column_start":5,"column_end":9,"is_primary":true,"text":[{"text":"    for x in v.iter().next().and(Some(0)) {","highlight_start":5,"highlight_end":9}],"label":null,"suggested_replacement":"if let Some(","suggestion_applicability":"MaybeIncorrect","expansion":{"span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":965,"byte_end":1037,"line_start":44,"line_end":46,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for x in v.iter().next().and(Some(0)) {","highlight_start":5,"highlight_end":44},{"text":"        println!(\"{}\", x);","highlight_start":1,"highlight_end":27},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":970,"byte_end":974,"line_start":44,"line_end":44,"column_start":10,"column_end":14,"is_primary":true,"text":[{"text":"    for x in v.iter().next().and(Some(0)) {","highlight_start":10,"highlight_end":14}],"label":null,"suggested_replacement":") = ","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: for loop over an `Option`. This is more readably written as an `if let` statement\n  --> tests/ui/for_loops_over_fallibles.rs:44:14\n   |\nLL |     for x in v.iter().next().and(Some(0)) {\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nhelp: to check pattern in a loop use `while let`\n   |\nLL |     while let Some(x) = v.iter().next().and(Some(0)) {\n   |     ~~~~~~~~~~~~~~~ ~~~\nhelp: consider using `if let` to clear intent\n   |\nLL |     if let Some(x) = v.iter().next().and(Some(0)) {\n   |     ~~~~~~~~~~~~ ~~~\n\n"}
{"message":"for loop over a `Result`. This is more readably written as an `if let` statement","code":{"code":"for_loop_over_fallibles","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":1052,"byte_end":1088,"line_start":48,"line_end":48,"column_start":14,"column_end":50,"is_primary":true,"text":[{"text":"    for x in v.iter().next().ok_or(\"x not found\") {","highlight_start":14,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"to check pattern in a loop use `while let`","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":1043,"byte_end":1047,"line_start":48,"line_end":48,"column_start":5,"column_end":9,"is_primary":true,"text":[{"text":"    for x in v.iter().next().ok_or(\"x not found\") {","highlight_start":5,"highlight_end":9}],"label":null,"suggested_replacement":"while let Ok(","suggestion_applicability":"MaybeIncorrect","expansion":{"span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":1043,"byte_end":1123,"line_start":48,"line_end":50,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for x in v.iter().next().ok_or(\"x not found\") {","highlight_start":5,"highlight_end":52},{"text":"        println!(\"{}\", x);","highlight_start":1,"highlight_end":27},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":1048,"byte_end":1052,"line_start":48,"line_end":48,"column_start":10,"column_end":14,"is_primary":true,"text":[{"text":"    for x in v.iter().next().ok_or(\"x not found\") {","highlight_start":10,"highlight_end":14}],"label":null,"suggested_replacement":") = ","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null},{"message":"consider using `if let` to clear intent","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":1043,"byte_end":1047,"line_start":48,"line_end":48,"column_start":5,"column_end":9,"is_primary":true,"text":[{"text":"    for x in v.iter().next().ok_or(\"x not found\") {","highlight_start":5,"highlight_end":9}],"label":null,"suggested_replacement":"if let Ok(","suggestion_applicability":"MaybeIncorrect","expansion":{"span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":1043,"byte_end":1123,"line_start":48,"line_end":50,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for x in v.iter().next().ok_or(\"x not found\") {","highlight_start":5,"highlight_end":52},{"text":"        println!(\"{}\", x);","highlight_start":1,"highlight_end":27},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":1048,"byte_end":1052,"line_start":48,"line_end":48,"column_start":10,"column_end":14,"is_primary":true,"text":[{"text":"    for x in v.iter().next().ok_or(\"x not found\") {","highlight_start":10,"highlight_end":14}],"label":null,"suggested_replacement":") = ","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: for loop over a `Result`. This is more readably written as an `if let` statement\n  --> tests/ui/for_loops_over_fallibles.rs:48:14\n   |\nLL |     for x in v.iter().next().ok_or(\"x not found\") {\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nhelp: to check pattern in a loop use `while let`\n   |\nLL |     while let Ok(x) = v.iter().next().ok_or(\"x not found\") {\n   |     ~~~~~~~~~~~~~ ~~~\nhelp: consider using `if let` to clear intent\n   |\nLL |     if let Ok(x) = v.iter().next().ok_or(\"x not found\") {\n   |     ~~~~~~~~~~ ~~~\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.8.0/src/lib.rs:111:22
