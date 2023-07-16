\n\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/privacy/pub-priv-dep/std-pub.rs","byte_start":130,"byte_end":157,"line_start":6,"line_end":6,"column_start":12,"column_end":39,"is_primary":true,"text":[{"text":"#![feature(public_private_dependencies)]","highlight_start":12,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0635]: unknown feature `public_private_dependencies`\n  --> /checkout/src/test/ui/privacy/pub-priv-dep/std-pub.rs:6:12\n   |\nLL | #![feature(public_private_dependencies)]\n   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:04:15] {"message":"For more information about this error, try `rustc --explain E0635`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0635`.\n"}
[01:04:15] 
[01:04:15] ------------------------------------------
[01:04:15] 
[01:04:15] 
[01:04:15] thread '[ui] ui/privacy/pub-priv-dep/std-pub.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3291:9
[01:04:15] 
[01:0t code: 101
[01:04:15] 
[01:04:15] 
[01:04:15] 
[01:04:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:15] Build completed unsuccessfully in 0:04:07
[01:04:15] make: *** [check] Error 1
[01:04:15] Makefile:48: recipe for target 'check' failed
