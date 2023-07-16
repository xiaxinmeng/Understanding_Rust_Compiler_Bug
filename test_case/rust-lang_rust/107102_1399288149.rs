plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error: unused import: `MaybeCause`
 --> compiler/rustc_trait_selection/src/solve/assembly.rs:4:59
  |
4 | use super::{CanonicalResponse, Certainty, EvalCtxt, Goal, MaybeCause, QueryResult};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `MaybeCause`
 --> compiler/rustc_trait_selection/src/solve/project_goals.rs:6:40
  |
  |
6 | use super::{Certainty, EvalCtxt, Goal, MaybeCause, QueryResult};

error: unused import: `MaybeCause`
 --> compiler/rustc_trait_selection/src/solve/trait_goals.rs:7:40
  |
  |
7 | use super::{Certainty, EvalCtxt, Goal, MaybeCause, QueryResult};

error: could not compile `rustc_trait_selection` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_trait_selection` due to 3 previous errors
