plain
test test::test_workspaces_cwd ... ok

failures:

---- doc::doc_message_format stdout ----
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo doc --message-format=json`
thread 'doc::doc_message_format' panicked at '
Expected: execs
    but: exited with Some(0)
--- stdout
{"reason":"compiler-message","package_id":"foo 0.0.1 (path+file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t685/foo)","target":{"kind":["lib"],"crate_types":["lib"],"name":"foo","src_path":"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t685/foo/src/lib.rs","edition":"2015","doc":true,"doctest":true,"test":true},"message":{"rendered":"warning: lint `broken_intra_doc_links` has been renamed to `rustdoc::broken_intra_doc_links`\n --> src/lib.rs:2:9\n  |\n2 | #![deny(broken_intra_doc_links)]\n  |         ^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `rustdoc::broken_intra_doc_links`\n  |\n  = note: `#[warn(renamed_and_removed_lints)]` on by default\n\n","children":[{"children":[],"code":null,"level":"note","message":"`#[warn(renamed_and_removed_lints)]` on by default","rendered":null,"spans":[]},{"children":[],"code":null,"level":"help","message":"use the new name","rendered":null,"spans":[{"byte_end":31,"byte_start":9,"column_end":31,"column_start":9,"expansion":null,"file_name":"src/lib.rs","is_primary":true,"label":null,"line_end":2,"line_start":2,"suggested_replacement":"rustdoc::broken_intra_doc_links","suggestion_applicability":"MachineApplicable","text":[{"highlight_end":31,"highlight_start":9,"text":"#![deny(broken_intra_doc_links)]"}]}]}],"code":{"code":"renamed_and_removed_lints","explanation":null},"level":"warning","message":"lint `broken_intra_doc_links` has been renamed to `rustdoc::broken_intra_doc_links`","spans":[{"byte_end":31,"byte_start":9,"column_end":31,"column_start":9,"expansion":null,"file_name":"src/lib.rs","is_primary":true,"label":null,"line_end":2,"line_start":2,"suggested_replacement":null,"suggestion_applicability":null,"text":[{"highlight_end":31,"highlight_start":9,"text":"#![deny(broken_intra_doc_links)]"}]}]}}
{"reason":"compiler-message","package_id":"foo 0.0.1 (path+file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t685/foo)","target":{"kind":["lib"],"crate_types":["lib"],"name":"foo","src_path":"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t685/foo/src/lib.rs","edition":"2015","doc":true,"doctest":true,"test":true},"message":{"rendered":"warning: unresolved link to `bad_link`\n --> src/lib.rs:4:6\n  |\n4 | /// [bad_link]\n  |      ^^^^^^^^ no item named `bad_link` in scope\n  |\n  = note: `#[warn(rustdoc::broken_intra_doc_links)]` on by default\n  = help: to escape `[` and `]` characters, add '\\' before them like `\\[` or `\\]`\n\n","children":[{"children":[],"code":null,"level":"note","message":"`#[warn(rustdoc::broken_intra_doc_links)]` on by default","rendered":null,"spans":[]},{"children":[],"code":null,"level":"help","message":"to escape `[` and `]` characters, add '\\' before them like `\\[` or `\\]`","rendered":null,"spans":[]}],"code":{"code":"rustdoc::broken_intra_doc_links","explanation":null},"level":"warning","message":"unresolved link to `bad_link`","spans":[{"byte_end":48,"byte_start":40,"column_end":14,"column_start":6,"expansion":null,"file_name":"src/lib.rs","is_primary":true,"label":"no item named `bad_link` in scope","line_end":4,"line_start":4,"suggested_replacement":null,"suggestion_applicability":null,"text":[{"highlight_end":14,"highlight_start":6,"text":"/// [bad_link]"}]}]}}
{"reason":"compiler-message","package_id":"foo 0.0.1 (path+file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t685/foo)","target":{"kind":["lib"],"crate_types":["lib"],"name":"foo","src_path":"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t685/foo/src/lib.rs","edition":"2015","doc":true,"doctest":true,"test":true},"message":{"rendered":"warning: 2 warnings emitted\n\n","children":[],"code":null,"level":"warning","message":"2 warnings emitted","spans":[]}}
{"reason":"compiler-artifact","package_id":"foo 0.0.1 (path+file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t685/foo)","target":{"kind":["lib"],"crate_types":["lib"],"name":"foo","src_path":"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t685/foo/src/lib.rs","edition":"2015","doc":true,"doctest":true,"test":true},"profile":{"opt_level":"0","debuginfo":2,"debug_assertions":true,"overflow_checks":true,"test":false},"features":[],"filenames":["/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t685/foo/target/doc/foo/index.html"],"executable":null,"fresh":false}
{"reason":"build-finished","success":true}
--- stderr
 Documenting foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t685/foo)
    Finished dev [unoptimized + debuginfo] target(s) in 1.12s
', src/tools/cargo/crates/cargo-test-support/src/lib.rs:739:13
', src/tools/cargo/crates/cargo-test-support/src/lib.rs:739:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- doc::short_message_format stdout ----
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo doc --message-format=short`
thread 'doc::short_message_format' panicked at '
Expected: execs
    but: exited with Some(0)

--- stderr
 Documenting foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t715/foo)
 Documenting foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t715/foo)
src/lib.rs:2:9: warning: lint `broken_intra_doc_links` has been renamed to `rustdoc::broken_intra_doc_links`
src/lib.rs:4:6: warning: unresolved link to `bad_link`
    Finished dev [unoptimized + debuginfo] target(s) in 1.17s
', src/tools/cargo/crates/cargo-test-support/src/lib.rs:739:13


---


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/cargo src/tools/cargotest
Build completed unsuccessfully in 0:23:39
make: *** [check-aux] Error 1
Makefile:44: recipe for target 'check-aux' failed
