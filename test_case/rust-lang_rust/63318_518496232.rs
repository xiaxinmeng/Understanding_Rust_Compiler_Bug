plain
2019-08-06T02:41:15.9798396Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-06T02:41:15.9798456Z 
2019-08-06T02:41:15.9798660Z   git checkout -b <new-branch-name>
2019-08-06T02:41:15.9798696Z 
2019-08-06T02:41:15.9798952Z HEAD is now at 92de8936b Auto merge of #63318 - Centril:rollup-r599iq6, r=Centril
2019-08-06T02:41:15.9946542Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-06T02:41:15.9949424Z ==============================================================================
2019-08-06T02:41:15.9949497Z Task         : Bash
2019-08-06T02:41:15.9949587Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-06T04:41:43.7142997Z test workspaces::ws_warn_unused ... ok
2019-08-06T04:41:43.7143150Z 
2019-08-06T04:41:43.7143219Z failures:
2019-08-06T04:41:43.7143281Z 
2019-08-06T04:41:43.7147281Z ---- check::rustc_check_err stdout ----
2019-08-06T04:41:43.7148159Z running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo rustc --profile check -- --emit=metadata`
2019-08-06T04:41:43.7148883Z thread 'check::rustc_check_err' panicked at '
2019-08-06T04:41:43.7149007Z Expected: execs
2019-08-06T04:41:43.7149102Z     but: expected to find:
2019-08-06T04:41:43.7149174Z [..]cannot find function `qux` in module `bar`
2019-08-06T04:41:43.7149298Z did not find in output:
2019-08-06T04:41:43.7149298Z did not find in output:
2019-08-06T04:41:43.7150238Z     Checking bar v0.1.0 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t458/bar)
2019-08-06T04:41:43.7150659Z     Checking foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t458/foo)
2019-08-06T04:41:43.7150780Z error[E0425]: cannot find function `qux` in crate `bar`
2019-08-06T04:41:43.7151095Z   |
2019-08-06T04:41:43.7151095Z   |
2019-08-06T04:41:43.7151160Z 1 | extern crate bar; fn main() { ::bar::qux(); }
2019-08-06T04:41:43.7151257Z   |                                      ^^^ not found in `bar`
2019-08-06T04:41:43.7151369Z error: aborting due to previous error
2019-08-06T04:41:43.7151446Z 
2019-08-06T04:41:43.7151724Z For more information about this error, try `rustc --explain E0425`.
2019-08-06T04:41:43.7151823Z error: Could not compile `foo`.
2019-08-06T04:41:43.7151823Z error: Could not compile `foo`.
2019-08-06T04:41:43.7151866Z 
2019-08-06T04:41:43.7152256Z To learn more, run the command again with --verbose.
2019-08-06T04:41:43.7152511Z ', src/tools/cargo/tests/testsuite/support/mod.rs:843:13
2019-08-06T04:41:43.7152605Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-06T04:41:43.7152656Z 
2019-08-06T04:41:43.7152876Z ---- metabuild::metabuild_failed_build_json stdout ----
2019-08-06T04:41:43.7153435Z running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build --message-format=json`
2019-08-06T04:41:43.7153688Z thread 'metabuild::metabuild_failed_build_json' panicked at '
2019-08-06T04:41:43.7153771Z Expected: execs
2019-08-06T04:41:43.7153828Z     but: Did not find expected JSON:
2019-08-06T04:41:43.7154774Z "\n{\n  \"message\": {\n    \"children\": \"{...}\",\n    \"code\": \"{...}\",\n    \"level\": \"error\",\n    \"message\": \"cannot find function `metabuild` in module `mb`\",\n    \"rendered\": \"[..]\",\n    \"spans\": \"{...}\"\n  },\n  \"package_id\": \"foo [..]\",\n  \"reason\": \"compiler-message\",\n  \"target\": {\n    \"crate_types\": [\n      \"bin\"\n    ],\n    \"doctest\": false,\n    \"edition\": \"2018\",\n    \"kind\": [\n      \"custom-build\"\n    ],\n    \"name\": \"metabuild-foo\",\n    \"src_path\": null\n  }\n}\n"
2019-08-06T04:41:43.7155173Z Remaining available output:
2019-08-06T04:41:43.7156618Z {"reason":"compiler-artifact","package_id":"mb 0.5.0 (path+file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t932/foo/mb)","target":{"kind":["lib"],"crate_types":["lib"],"name":"mb","src_path":"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t932/foo/mb/src/lib.rs","edition":"2015","doctest":true},"profile":{"opt_level":"0","debuginfo":2,"debug_assertions":true,"overflow_checks":true,"test":false},"features":[],"filenames":["/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t932/foo/target/debug/deps/libmb-e9507ea5788b89c0.rlib","/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t932/foo/target/debug/deps/libmb-e9507ea5788b89c0.rmeta"],"executable":null,"fresh":false}
2019-08-06T04:41:43.7158180Z {"reason":"compiler-artifact","package_id":"mb-other 0.0.1 (path+file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t932/foo/mb-other)","target":{"kind":["lib"],"crate_types":["lib"],"name":"mb-other","src_path":"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t932/foo/mb-other/src/lib.rs","edition":"2015","doctest":true},"profile":{"opt_level":"0","debuginfo":2,"debug_assertions":true,"overflow_checks":true,"test":false},"features":[],"filenames":["/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t932/foo/target/debug/deps/libmb_other-cbe3510fdcdf0ea7.rlib","/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t932/foo/target/debug/deps/libmb_other-cbe3510fdcdf0ea7.rmeta"],"executable":null,"fresh":false}
2019-08-06T04:41:43.7163264Z {"reason":"compiler-message","package_id":"foo 0.0.1 (path+file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t932/foo)","target":{"kind":["custom-build"],"crate_types":["bin"],"name":"metabuild-foo","src_path":null,"edition":"2018","doctest":false},"message":{"rendered":"error[E0425]: cannot find function `metabuild` in crate `mb`\n --> target/.metabuild/metabuild-foo-11ea4648a08a0492.rs:4:9\n  |\n4 |     mb::metabuild();\n  |         ^^^^^^^^^ not found in `mb`\nhelp: possible candidate is found in another module, you can import it into scope\n  |\n1 | use mb_other::metabuild;\n  |\n\n","children":[{"children":[],"code":null,"level":"help","message":"possible candidate is found in another module, you can import it into scope","rendered":null,"spans":[{"byte_end":0,"byte_start":0,"column_end":1,"column_start":1,"expansion":null,"file_name":"target/.metabuild/metabuild-foo-11ea4648a08a0492.rs","is_primary":true,"label":null,"line_end":1,"line_start":1,"suggested_replacement":"use mb_other::metabuild;\n","suggestion_applicability":"Unspecified","text":[{"highlight_end":1,"highlight_start":1,"text":"use mb;"}]}]}],"code":{"code":"E0425","explanation":"\nAn unresolved name was used.\n\nErroneous code examples:\n\n