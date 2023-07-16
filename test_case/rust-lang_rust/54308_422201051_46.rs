\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/xcrate/xcrate-unit-struct.rs","byte_start":700,"byte_end":736,"line_start":19,"line_end":19,"column_start":13,"column_end":49,"is_primary":true,"text":[{"text":"    let _ = xcrate_unit_struct::StructWithFields;","highlight_start":13,"highlight_end":49}],"label":"not a value","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0423]: expected value, found struct `xcrate_unit_struct::StructWithFields`\n  --> /checkout/src/test/ui/xcrate/xcrate-unit-struct.rs:19:13\n   |\nLL |     let _ = xcrate_unit_struct::StructWithFields;\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not a value\n\n"}
[01:05:24] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:05:24] {"message":"For more information about this error, try `rustc --explain E0423`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0423`.\n"}
[01:05:24] ------------------------------------------
[01:05:24] 
[01:05:24] thread '[ui] ui/xcrate/xcrate-unit-struct.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[01:05:24] 
---
151412 ./src/tools/clang
149112 ./src/llvm-emscripten/test
149064 ./obj/build/bootstrap/debug/incremental
134624 ./obj/build/bootstrap/debug/incremental/bootstrap-14ucxxomeo8gg
134620 ./obj/build/bootstrap/debug/incremental/bootstrap-14ucxxomeo8gg/s-f4wef9juxj-10uy6ip-2f6sgdh2xj16g
131200 ./obj/build/x86_64-unknown-linux-gn ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps
35108 ./obj/build/x86_64-unknown-linux-gnu/doc/core/arch
35036 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt
34916 ./obj/build/x86_64-unknown-linux-gnu/native/jemalloc/src
34588 ./obj/build/x86_64-unknown-linux-gnu/native/jemalloc/lib
---
travis_time:end:0826a6ee:start=1537225908388353926,finish=1537225908393644551,duration=5290625
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ff8e58c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!
