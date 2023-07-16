\n\nFor more information on the rust ownership system, take a look at\nhttps://doc.rust-lang.org/stable/book/references-and-borrowing.html.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/get_default.rs","byte_start":1367,"byte_end":1370,"line_start":45,"line_end":45,"column_start":17,"column_end":20,"is_primary":true,"text":[{"text":"                map.set(String::new()); // Both AST and MIR error here","highlight_start":17,"highlight_end":20}],"label":"mutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/get_default.rs","byte_start":1314,"byte_end":1317,"line_start":43,"line_end":43,"column_start":15,"column_end":18,"is_primary":false,"text":[{"text":"        match map.get() {","highlight_start":15,"highlight_end":18}],"label":"immutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/get_default.rs","byte_start":1812,"byte_end":1813,"line_start":57,"line_end":57,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"immutable borrow ends here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0502]: cannot borrow `*map` as mutable because it is also borrowed as immutable (Ast)\n  --> /checkout/src/test/ui/nll/get_default.rs:45:3469736 .
2578316 ./obj/build
1821044 ./obj/build/x86_64-unknown-linux-gnu
727164 ./src
567100 ./obj/build/bootstrap
---
149128 ./src/llvm-emscripten/test
141984 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
141980 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
122540 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj
122536 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj/s-f1f9tgmmr8-zgya24-33jenrrxh1gwm
107128 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release
104172 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/xfinish=1527420210869139965,duration=8966128
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
