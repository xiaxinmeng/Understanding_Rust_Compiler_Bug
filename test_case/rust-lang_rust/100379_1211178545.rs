plain
   Compiling toml v0.5.7
   Compiling toml_edit v0.14.3
   Compiling clap v3.2.5
   Compiling racer v2.2.2 (/checkout/src/tools/rls/racer)
error[E0407]: method `fluent_bundle` is not a member of trait `Emitter`
   |
33 | /     fn fluent_bundle(&self) -> Option<&Lrc<rustc_errors::FluentBundle>> {
34 | |         None
35 | |     }
35 | |     }
   | |_____^ not a member of trait `Emitter`

error[E0407]: method `fallback_fluent_bundle` is not a member of trait `Emitter`
  --> src/tools/rls/racer/src/racer/ast.rs:36:5
   |
36 | /     fn fallback_fluent_bundle(&self) -> &rustc_errors::FluentBundle {
37 | |         unimplemented!("diagnostic translations are unimplemented in racer");
   | |_____^ not a member of trait `Emitter`

warning: unexpected `cfg` condition value
 --> src/tools/rls/racer/src/racer/lib.rs:1:13
---
error[E0277]: the trait bound `DummyEmitter: Translate` is not satisfied
   --> src/tools/rls/racer/src/racer/ast.rs:25:6
    |
25  | impl Emitter for DummyEmitter {
    |      ^^^^^^^ the trait `Translate` is not implemented for `DummyEmitter`
    |
    = help: the following other types implement trait `Translate`:
              AnnotateSnippetEmitterWriter
              EmitterWriter
              JsonEmitter
              SilentEmitter
note: required by a bound in `Emitter`
    |
203 | pub trait Emitter: Translate {
    |                    ^^^^^^^^^ required by this bound in `Emitter`

---

Mismatch at src/parse/session.rs:2:
 use std::sync::atomic::{AtomicBool, Ordering};
 
 use rustc_data_structures::sync::{Lrc, Send};
-use rustc_errors::translation::Translate;
 use rustc_errors::emitter::{Emitter, EmitterWriter};
+use rustc_errors::translation::Translate;
 use rustc_errors::{ColorConfig, Diagnostic, Handler, Level as DiagnosticLevel};
 use rustc_session::parse::ParseSess as RawParseSess;
 use rustc_span::{
test test::system_tests ... ok
test test::idempotence_tests ... ok

failures:
failures:

---- test::self_tests stdout ----
Ran 5 self tests.
thread 'test::self_tests' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`: 1 self tests failed', src/tools/rustfmt/src/test/mod.rs:400:5


failures:
    test::self_tests
