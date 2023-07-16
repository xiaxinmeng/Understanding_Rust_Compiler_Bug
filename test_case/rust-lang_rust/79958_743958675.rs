text
Many errors seem to be in tests that declare `no_core`, and I think adding `-Zinstrument-coverage`
may force loading `core` (or `std`) anyway. See `creader.rs`:

   fn inject_profiler_runtime(&mut self) {
       if (self.sess.opts.debugging_opts.instrument_coverage
           || self.sess.opts.debugging_opts.profile
           || self.sess.opts.cg.profile_generate.enabled())
           && !self.sess.opts.debugging_opts.no_profiler_runtime
       {
           info!("loading profiler");

           let name = sym::profiler_builtins;
           let cnum = self.resolve_crate(name, DUMMY_SP, CrateDepKind::Implicit, None);
           let data = self.cstore.get_crate_data(cnum);

           // Sanity check the loaded crate to ensure it is indeed a profiler runtime
           if !data.is_profiler_runtime() {
               self.sess.err("the crate `profiler_builtins` is not a profiler runtime");
           }
       }
   }

===================================================================================================

Failures due to:
    error[E0152]: found duplicate lang item `sized`

All of the 'duplicate lang item' errors include `#![no_core]` and then define at least one lang item
already defined in core.

As shown above, `-Zinstrument-coverage` force-loads `profiler_builtins`, and that
may force-load `std` or `core` anyway?

src/test/ui/associated-types/associated-types-ICE-when-projecting-out-of-err.rs
src/test/ui/feature-gates/feature-gate-cfg-target-has-atomic.rs
src/test/ui/issues/issue-19660.rs
src/test/ui/issues/issue-31076.rs
src/test/ui/lang-item-missing-generator.rs
src/test/ui/panic-handler/panic-handler-requires-panic-info.rs
src/test/ui/privacy/privacy1.rs
src/test/ui/privacy/privacy4.rs
src/test/ui/required-lang-item.rs
src/test/ui/static_sized_requirement.rs

Note that src/test/ui/asm/bad-arch.rs also redefines lang item `sized`, but fails earlier (no "profiler_builtins")

===================================================================================================

Failure due to:
   error: `#[panic_handler]` function required, but not found ...
   error: language item required, but not found: `eh_personality`
This test actually expects a missing lang item, because "no_core", but the test expects a specific missing lang item.
With -Zinstrument-coverage, it finds different missing lang items

src/test/ui/lang-item-missing.rs

