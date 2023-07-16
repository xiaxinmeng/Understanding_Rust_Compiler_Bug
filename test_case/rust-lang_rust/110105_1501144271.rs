plain
   Compiling proc-macro-srv-cli v0.0.0 (/checkout/src/tools/rust-analyzer/crates/proc-macro-srv-cli)
error: unreachable pattern
   --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:351:13
    |
351 |             Slice(slice) => Some(*slice),
    |
    |
    = note: `-D unreachable-patterns` implied by `-D warnings`
error: unreachable pattern
   --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:401:13
    |
    |
401 |             Slice(slice) => slice.arity(),

error: unreachable pattern
   --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:402:13
    |
    |
402 |             Str(..)
    |             ^^^^^^^

error: unreachable pattern
   --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:403:15
    |
403 |             | FloatRange(..)

error: unreachable pattern
   --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:448:13
    |
    |
448 |             Slice(slice) => match slice._unimplemented {},

error: unreachable pattern
   --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:471:13
    |
    |
471 |             (FloatRange(void), FloatRange(..)) => match *void {},

error: unreachable pattern
   --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:472:13
    |
    |
472 |             (Str(void), Str(..)) => match *void {},

error: unreachable pattern
   --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:473:13
    |
    |
473 |             (Slice(self_slice), Slice(other_slice)) => self_slice.is_covered_by(*other_slice),

error: unreachable pattern
   --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:505:13
    |
    |
505 |             Slice(slice) => used_ctors

error: unreachable pattern
   --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:511:13
    |
    |
511 |             Str(..) | FloatRange(..) | Opaque | Missing { .. } | Wildcard | Or => {

error: unreachable pattern
   --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:511:23
    |
    |
511 |             Str(..) | FloatRange(..) | Opaque | Missing { .. } | Wildcard | Or => {

error: unreachable pattern
   --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:820:13
    |
    |
820 |             Slice(slice) => match slice._unimplemented {},

error: unreachable pattern
   --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:821:13
    |
    |
821 |             Str(..)
    |             ^^^^^^^

error: unreachable pattern
   --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:822:15
    |
822 |             | FloatRange(..)

error: unreachable pattern
    --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:1024:13
     |
     |
1024 |             &Slice(slice) => match slice._unimplemented {},

error: unreachable pattern
    --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:1025:13
     |
     |
1025 |             &Str(void) => match void {},

error: unreachable pattern
    --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:1026:13
     |
     |
1026 |             &FloatRange(void) => match void {},

error: unreachable pattern
    --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:1072:13
     |
     |
1072 |             (Slice(self_slice), Slice(other_slice))

error: unreachable pattern
    --> crates/hir-ty/src/display.rs:1351:13
     |
     |
1351 |             LifetimeData::Phantom(_, _) => Ok(()),


error: could not compile `hir-ty` due to 19 previous errors
Build completed unsuccessfully in 0:24:21
