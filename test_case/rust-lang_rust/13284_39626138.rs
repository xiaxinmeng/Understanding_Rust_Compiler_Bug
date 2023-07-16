
% /Users/fklock/Dev/Mozilla/rust.git/objdir-dbgopt/x86_64-apple-darwin/stage1/bin/rustc --out-dir /Users/fklock/Dev/Mozilla/rust.git/objdir-dbgopt/x86_64-apple-darwin/test/run-make/many-crates-but-no-match -L /Users/fklock/Dev/Mozilla/rust.git/objdir-dbgopt/x86_64-apple-darwin/test/run-make/many-crates-but-no-match -L/Users/fklock/Dev/Mozilla/rust.git/objdir-dbgopt/x86_64-apple-darwin/test/run-make/many-crates-but-no-match/a2 -L/Users/fklock/Dev/Mozilla/rust.git/objdir-dbgopt/x86_64-apple-darwin/test/run-make/many-crates-but-no-match/a3 ../src/test/run-make/many-crates-but-no-match/crateC.rs
../src/test/run-make/many-crates-but-no-match/crateC.rs:11:1: 11:21 error: found possibly newer version of crate `crateA` which `crateB` depends on
../src/test/run-make/many-crates-but-no-match/crateC.rs:11 extern crate crateB;
                                                           ^~~~~~~~~~~~~~~~~~~~
../src/test/run-make/many-crates-but-no-match/crateC.rs:11:1: 11:21 note: perhaps this crate needs to be recompiled?
../src/test/run-make/many-crates-but-no-match/crateC.rs:11 extern crate crateB;
                                                           ^~~~~~~~~~~~~~~~~~~~
../src/test/run-make/many-crates-but-no-match/crateC.rs:11:21: 11:21 note: crate `crateA` path #1: /Users/fklock/Dev/Mozilla/rust.git/objdir-dbgopt/x86_64-apple-darwin/test/run-make/many-crates-but-no-match/a3/libcrateA-2f87ccf5-0.0.rlib
../src/test/run-make/many-crates-but-no-match/crateC.rs:11:21: 11:21 note: crate `crateA` path #2: /Users/fklock/Dev/Mozilla/rust.git/objdir-dbgopt/x86_64-apple-darwin/test/run-make/many-crates-but-no-match/a2/libcrateA-2f87ccf5-0.0.rlib
../src/test/run-make/many-crates-but-no-match/crateC.rs:11:21: 11:21 note: crate `crateB` path #1: /Users/fklock/Dev/Mozilla/rust.git/objdir-dbgopt/x86_64-apple-darwin/test/run-make/many-crates-but-no-match/libcrateB-38a3e48a-0.0.rlib
error: aborting due to previous error
