plain
    |
281 |         match self._unimplemented {}
    |               ----^^^^^^^^^^^^^^^
    |               |
    |               unreachable expression
    |               this expression has type `deconstruct_pat::Slice`, which is uninhabited
    |
    = note: `-D unreachable-code` implied by `-D warnings`
error: unreachable expression
   --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:286:15
    |
286 |         match self._unimplemented {}
286 |         match self._unimplemented {}
    |               ----^^^^^^^^^^^^^^^
    |               |
    |               unreachable expression
    |               this expression has type `deconstruct_pat::Slice`, which is uninhabited
error: unreachable call
   --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:348:29
    |
    |
348 |             Slice(slice) => Some(*slice),
    |                             ^^^^ ------ this expression has type `deconstruct_pat::Slice`, which is uninhabited
    |                             unreachable call

error: unreachable call
   --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:470:67
   --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:470:67
    |
470 |             (Slice(self_slice), Slice(other_slice)) => self_slice.is_covered_by(*other_slice),
    |                                                                   ^^^^^^^^^^^^^ ------------ this expression has type `deconstruct_pat::Slice`, which is uninhabited
    |                                                                   unreachable call

error: unreachable call
   --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:505:36
   --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:505:36
    |
505 |                 .any(|other| slice.is_covered_by(other)),
    |                                    ^^^^^^^^^^^^^ ----- this expression has type `deconstruct_pat::Slice`, which is uninhabited
    |                                    unreachable call

error: unreachable expression
    --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:1020:36
    --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:1020:36
     |
1020 |             &Slice(slice) => match slice._unimplemented {},
     |                                    |
     |                                    unreachable expression
     |                                    unreachable expression
     |                                    this expression has type `deconstruct_pat::Slice`, which is uninhabited

error: could not compile `hir-ty` due to 6 previous errors
Build completed unsuccessfully in 0:28:03
