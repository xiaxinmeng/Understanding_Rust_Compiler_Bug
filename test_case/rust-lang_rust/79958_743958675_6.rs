text

===================================================================================================

Failure due to:
    The program compiles successfully, but it is supposed to fail to compile,
    with a compile-time error because the compiler recognizes an overflow error.
With `-Z instrument-coverage`, the compiler fails to recognize the error at compile time,
but overflows at runtime.

Prior to this PR, -Zinstrument-coverage disables MIR pass `Inline`:

       if tcx.sess.opts.debugging_opts.instrument_coverage {
           // The current implementation of source code coverage injects code region counters
           // into the MIR, and assumes a 1-to-1 correspondence between MIR and source-code-
           // based function.
           debug!("function inlining is disabled when compiling with `instrument_coverage`");
           return;
       }

But this test was attempting to enable it, with `-Z mir-opt-level=2`:

src/test/ui/const_prop/inline_spans.rs

