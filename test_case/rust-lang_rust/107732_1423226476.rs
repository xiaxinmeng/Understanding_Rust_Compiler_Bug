plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:cb2b9920774a63bd54b3676f2b669ea1e777a91e)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
   Compiling rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
   Compiling rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
   Compiling rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
   Compiling rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
error[E0502]: cannot borrow `*cx` as immutable because it is also borrowed as mutable
  --> compiler/rustc_expand/src/mbe/diagnostics.rs:52:19
   |
31 |     let mut tracker = CollectTrackerAndEmitter::new(cx, sp);
   |                                                     -- mutable borrow occurs here
...
52 |     let mut err = cx.struct_span_err(span, &parse_failure_msg(&token));
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ immutable borrow occurs here
91 | }
   | - mutable borrow later used here


error[E0502]: cannot borrow `*cx` as immutable because it is also borrowed as mutable
  --> compiler/rustc_expand/src/mbe/diagnostics.rs:54:33
   |
31 |     let mut tracker = CollectTrackerAndEmitter::new(cx, sp);
   |                                                     -- mutable borrow occurs here
...
54 |     if !def_span.is_dummy() && !cx.source_map().is_imported(def_span) {
   |                                 ^^^^^^^^^^^^^^^ immutable borrow occurs here
91 | }
   | - mutable borrow later used here


error[E0502]: cannot borrow `*cx` as immutable because it is also borrowed as mutable
  --> compiler/rustc_expand/src/mbe/diagnostics.rs:55:24
   |
31 |     let mut tracker = CollectTrackerAndEmitter::new(cx, sp);
   |                                                     -- mutable borrow occurs here
...
55 |         err.span_label(cx.source_map().guess_head_span(def_span), "when calling this macro");
   |                        ^^^^^^^^^^^^^^^ immutable borrow occurs here
91 | }
   | - mutable borrow later used here


error[E0499]: cannot borrow `*cx` as mutable more than once at a time
  --> compiler/rustc_expand/src/mbe/diagnostics.rs:89:5
   |
31 |     let mut tracker = CollectTrackerAndEmitter::new(cx, sp);
   |                                                     -- first mutable borrow occurs here
...
89 |     cx.trace_macros_diag();
   |     ^^^^^^^^^^^^^^^^^^^^^^ second mutable borrow occurs here
90 |     DummyResult::any(sp)
   | - first borrow later used here

Some errors have detailed explanations: E0499, E0502.
For more information about an error, try `rustc --explain E0499`.
