plain
[00:02:36] Successfully tagged rust-ci:latest
[00:02:36] Built container sha256:fffbc36b59de7c436a6645f9bfe5ed3437aeaa48cf01b04f86be33019a1f39ee
[00:02:36] Uploading finished image to s3://rust-lang-ci-sccache2/docker/6e066733b0b8bc03640758eb363f3a99027aa4f79b7978ff798327e892ff0aa1e2aeadb5cc8358d1817cc604e5021ce84d8e6dbf513140f592fbc78995b78ff4
[00:02:37] 
[00:02:37] Partial credentials found in env, missing: AWS_SECRET_ACCESS_KEY
[00:02:43] xargs: docker: terminated by signal 13

[00:02:44] travis_time:end:1d6e0de0:start=1539916409686970126,finish=1539916512837461301,duration=103150491175
[CI_JOB_NAME=x86_64-gnu-llvm-5.0]
[00:02:44] [CI_JOB_NAME=x86_64-gnu-llvm-5.0]
---
[00:49:14] ......................................i............................................................. 3900/4626
[00:49:19] .................................................................................................... 4000/4626
[00:49:22] .................................................................................................... 4100/4626
[00:49:26] .................................................................................................... 4200/4626
[00:49:29] ..........i.........................F..F............................................................ 4300/4626
[00:49:36] .................................................................................................... 4500/4626
[00:49:39] ..........................
[00:49:39] failures:
[00:49:39] 
[00:49:39] 
[00:49:39] ---- [ui] ui/trait-alias-fail.rs stdout ----
[00:49:39] diff of stderr:
[00:49:39] 
[00:49:39] 22 LL | impl Alias1 for () { //~ERROR expected trait, found trait alias
[00:49:39] 24 
[00:49:39] 24 
[00:49:39] - error[E0658]: trait aliases are not yet fully implemented (see issue #41517)
[00:49:39] + error[E0658]: trait aliases are experimental (see issue #41517)
[00:49:39] 27    |
[00:49:39] 27    |
[00:49:39] 28 LL | trait Alias1<T> = Default where T: Clone; // ok
[00:49:39] 30    |
[00:49:39] 31    = help: add #![feature(trait_alias)] to the crate attributes to enable
[00:49:39] 32 
[00:49:39] 32 
[00:49:39] - error[E0658]: trait aliases are not yet fully implemented (see issue #41517)
[00:49:39] + error[E0658]: trait aliases are experimental (see issue #41517)
[00:49:39] 35    |
[00:49:39] 35    |
[00:49:39] 36 LL | trait Alias2<T: Clone = ()> = Default;
[00:49:39] 
[00:49:39] The actual stderr differed from the expected stderr.
[00:49:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-alias-fail/trait-alias-fail.stderr
[00:49:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-alias-fail/trait-alias-fail.stderr
[00:49:39] To update references, rerun the tests and pass the `--bless` flag
[00:49:300:49:39] {"message":"type parameters on the left side of a trait alias cannot have defaults","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/trait-alias-fail.rs","byte_start":612,"byte_end":613,"line_start":15,"line_end":15,"column_start":14,"column_end":15,"is_primary":true,"text":[{"text":"trait Alias2<T: Clone = ()> = Default;","highlight_start":14,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: type parameters on the left side of a trait alias cannot have defaults\n  --> /checkout/src/test/ui/trait-alias-fail.rs:15:14\n   |\nLL | trait Alias2<T: Clone = ()> = Default;\n   |              ^\n\n"}
[00:49:39] {"message":"expected type, found trait alias `Alias1`","code":{"code":"E0573","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/trait-alias-fail.rs","byte_start":872,"byte_end":878,"line_start":20,"line_end":20,"column_start":6,"column_end":12,"is_primary":true,"text":[{"text":"impl Alias1 { //~ERROR expected type, found trait alias","highlight_start":6,"highlight_end":12}],"label":"not a type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0573]: expected type, found trait alias `Alias1`\n  --> /checkout/src/test/ui/trait-alias-fail.rs:20:6\n   |\nLL | impl Alias1 { //~ERROR expected type, found trait alias\n   |      ^^^^^^ not a type\n\n"}
[00:49:39] {"message":"expected trait, found trait alias `Alias1`","code":{"code":"E0404","explanation":"\nYou tried to use something which is not a trait in a trait position, such as\na bound or `impl`.\n\nErroneous code example:\n\n