plain
[00:37:34]    Compiling minifier v0.0.19
[00:37:36]    Compiling rand v0.4.2
[00:37:39]    Compiling tempfile v3.0.2
[00:37:40]    Compiling rustdoc v0.0.0 (file:///checkout/src/librustdoc)
[00:37:44] error[E0599]: no function or associated item named `with_capacity` found for type `std::collections::HashSet<_, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>` in the current scope
[00:37:44]     --> librustdoc/html/render.rs:1559:22
[00:37:44]      |
[00:37:44] 1559 |             structs: FxHashSet::with_capacity(100),
[00:37:44]      |                      ^^^^^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `std::collections::HashSet<_, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:37:44] 
[00:37:44] error[E0599]: no function or associated item named `with_capacity` found for type `std::collections::HashSet<_, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>` in the current scope
[00:37:44]     --> librustdoc/html/render.rs:1560:20
[00:37:44]      |
[00:37:44] 1560 |             enums: FxHashSet::with_capacity(100),
[00:37:44]      |                    ^^^^^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `std::collections::HashSet<_, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:37:44] 
[00:37:44] error[E0599]: no function or associated item named `with_capacity` found for type `std::collections::HashSet<_, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>` in the current scope
[00:37:44]     --> librustdoc/html/render.rs:1561:21
[00:37:44]      |
[00:37:44] 1561 |             unions: FxHashSet::with_capacity(100),
[00:37:44]      |                     ^^^^^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `std::collections::HashSet<_, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:37:44] 
[00:37:44] error[E0599]: no function or associated item named `with_capacity` found for type `std::collections::HashSet<_, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>` in the current scope
[00:37:44]     --> librustdoc/html/render.rs:1562:25
[00:37:44]      |
[00:37:44] 1562 |             primitives: FxHashSet::with_capacity(26),
[00:37:44]      |                         ^^^^^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `std::collections::HashSet<_, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:37:44] 
[00:37:44] error[E0599]: no function or associated item named `with_capacity` found for type `std::collections::HashSet<_, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>` in the current scope
[00:37:44]     --> librustdoc/html/render.rs:1563:21
[00:37:44]      |
[00:37:44] 1563 |             traits: FxHashSet::with_capacity(100),
[00:37:44]      |                     ^^^^^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `std::collections::HashSet<_, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:37:44] 
[00:37:44] error[E0599]: no function or associated item named `with_capacity` found for type `std::collections::HashSet<_, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>` in the current scope
[00:37:44]     --> librustdoc/html/render.rs:1564:21
[00:37:44]      |
[00:37:44] 1564 |             macros: FxHashSet::with_capacity(100),
[00:37:44]      |                     ^^^^^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `std::collections::HashSet<_, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:37:44] 
[00:37:44] error[E0599]: no function or associated item named `with_capacity` found for type `std::collections::HashSet<_, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>` in the current scope
[00:37:44]     --> librustdoc/html/render.rs:1567:27
[00:37:44]      |
[00:37:44] 1567 |             existentials: FxHashSet::with_capacity(100),
[00:37:44]      |                           ^^^^^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `std::collections::HashSet<_, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:37:44] 
[00:37:44] error[E0599]: no function or associated item named `with_capacity` found for type `std::collections::HashSet<_, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>` in the current scope
[00:37:44]     --> librustdoc/html/render.rs:1568:22
[00:37:44]      |
[00:37:44] 1568 |             statics: FxHashSet::with_capacity(100),
[00:37:44]      |                      ^^^^^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `std::collections::HashSet<_, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:ompiler_builtins/compiler-rt/test/builtins/Unit
22656 ./src/llvm-emscripten/test/MC
travis_time:end:0a865fa3:start=1534599293241779132,finish=1534599293769256863,duration=527477731
travis_fold:end:after_failure.1
---
travis_time:end:234c66d6:start=1534599293799337567,finish=1534599293808607568,duration=9270001
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b5a3254
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
