plain
2020-02-12T07:03:08.3651061Z failures:
2020-02-12T07:03:08.3651128Z 
2020-02-12T07:03:08.3651970Z ---- check::check_fail stdout ----
2020-02-12T07:03:08.3652454Z running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo check`
2020-02-12T07:03:08.3653159Z thread 'check::check_fail' panicked at '
2020-02-12T07:03:08.3653274Z Expected: execs
2020-02-12T07:03:08.3653369Z     but: expected to find:
2020-02-12T07:03:08.3653791Z [..]this function takes 0 parameters but 1 parameter was supplied
2020-02-12T07:03:08.3653946Z did not find in output:
2020-02-12T07:03:08.3654454Z     Checking bar v0.1.0 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t449/bar)
2020-02-12T07:03:08.3654454Z     Checking bar v0.1.0 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t449/bar)
2020-02-12T07:03:08.3654941Z     Checking foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t449/foo)
2020-02-12T07:03:08.3655068Z error[E0061]: this function takes 0 arguments but 1 argument was supplied
2020-02-12T07:03:08.3655504Z   |
2020-02-12T07:03:08.3655504Z   |
2020-02-12T07:03:08.3655586Z 1 | extern crate bar; fn main() { ::bar::baz(42); }
2020-02-12T07:03:08.3655956Z   |                               ^^^^^^^^^^ -- supplied 1 argument
2020-02-12T07:03:08.3656397Z   |                               expected 0 arguments
2020-02-12T07:03:08.3656475Z 
2020-02-12T07:03:08.3656567Z error: aborting due to previous error
2020-02-12T07:03:08.3656619Z 
---
2020-02-12T07:03:08.3694536Z 
2020-02-12T07:03:08.3694600Z 
2020-02-12T07:03:08.3700902Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/tools/cargo src/tools/cargotest
2020-02-12T07:03:08.3701082Z Build completed unsuccessfully in 1:55:45
2020-02-12T07:03:08.3756889Z Makefile:50: recipe for target 'check-aux' failed
2020-02-12T07:03:08.3757067Z == clock drift check ==
2020-02-12T07:03:08.3758320Z   local time: make: *** [check-aux] Error 1
2020-02-12T07:03:08.6529361Z   network time: Wed, 12 Feb 2020 07:03:08 GMT
2020-02-12T07:03:08.6529523Z == end clock drift check ==
2020-02-12T07:03:11.3740577Z 
2020-02-12T07:03:11.3740577Z 
2020-02-12T07:03:11.3868968Z ##[error]Bash exited with code '2'.
2020-02-12T07:03:11.3927942Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-02-12T07:03:11.3930742Z ==============================================================================
2020-02-12T07:03:11.3930850Z Task         : Get sources
2020-02-12T07:03:11.3930972Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
