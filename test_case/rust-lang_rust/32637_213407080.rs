
% git remote -v
origin  git@github.com:paholg/typenum.git (fetch)
origin  git@github.com:paholg/typenum.git (push)
% git status
HEAD detached at 10c15e3
nothing to commit, working directory clean
% multirust default nightly-2016-04-07
multirust: using existing install for 'nightly-2016-04-07'
multirust: default toolchain set to 'nightly-2016-04-07'
% rustc --version
rustc 1.9.0-nightly (bf5da36f1 2016-04-06)
% OUT_DIR=/home/fklock/Dev/Rust/typenum/target/debug      rustc src/lib.rs --cra\
te-name typenum --crate-type lib -g --out-dir /home/fklock/Dev/Rust/typenum/targ\
et/debug --emit=dep-info,link -L dependency=/home/fklock/Dev/Rust/typenum/target\
/debug -L dependency=/home/fklock/Dev/Rust/typenum/target/debug/deps
error: overflow evaluating the requirement `uint::UInt<_, _> : private::PrivateC\
mp<uint::UInt<_, _>, Equal>` [E0275]
src/lib.rs:1:1: 1:1 note: consider adding a `#![recursion_limit="128"]` attribut\
e to your crate
error: aborting due to previous error
% multirust default nightly-2016-04-08
multirust: using existing install for 'nightly-2016-04-08'
multirust: default toolchain set to 'nightly-2016-04-08'
% rustc --version
rustc 1.9.0-nightly (7979dd608 2016-04-07)
% OUT_DIR=/home/fklock/Dev/Rust/typenum/target/debug      rustc src/lib.rs --cra\
te-name typenum --crate-type lib -g --out-dir /home/fklock/Dev/Rust/typenum/targ\
et/debug --emit=dep-info,link -L dependency=/home/fklock/Dev/Rust/typenum/target\
/debug -L dependency=/home/fklock/Dev/Rust/typenum/target/debug/deps
%
