\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/uniform-paths/ambiguity-nested.rs","byte_start":574,"byte_end":577,"line_start":16,"line_end":16,"column_start":13,"column_end":16,"is_primary":true,"text":[{"text":"    pub use std::io;","highlight_start":13,"highlight_end":16}],"label":"ambiguous name","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`std` could refer to a built-in extern crate","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/uniform-paths/ambiguity-nested.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2018 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"use `::std` to refer to this extern crate unambiguously","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"`std` could also refer to the module defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/uniform-paths/ambiguity-nested.rs","byte_start":622,"byte_end":660,"line_start":19,"line_end":21,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    mod std {","highlight_start":5,"highlight_end":14},{"text":"        pub struct io;","highlight_n extern crate\n   = help: use `::std` to refer to this extern crate unambiguously\nnote: `std` could also refer to the module defined here\n  --> /checkout/src/test/ui/rust-2018/uniform-paths/ambiguity.rs:16:1\n   |\nLL | / mod std {\nLL | |     pub struct io;\nLL | | }\n   | |_^\n   = help: use `self::std` to refer to this module unambiguously\n\n"}
[00:53:40] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:53:40] {"message":"For more information about this error, try `rustc --explain E0659`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0659`.\n"}
[00:53:40] ------------------------------------------
[00:53:40] 
[00:53:40] thread '[ui] ui/rust-2018/uniform-paths/ambiguity.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:53:40] 
---
[00:53:40] 
[00:53:40] 
[00:53:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:53:40] Build completed unsuccessfully in 0:04:00
[00:53:40] Makefile:58: recipe for target 'check' failed
[00:53:40] make: *** [check] Error 1
127444 ./.git/modules
127440 ./.git/modules/src
123692 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
115736 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release
