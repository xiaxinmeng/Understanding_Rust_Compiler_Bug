text

===================================================================================================

Side note regarding one more difference when compiling with `-Zinstrument-coverage`:

These test failures don't necessarily indicate an issue linking the LLVM runtime, but note,
ONLY the `GccLinker` does something specific for coverage and profiling. This may or may not be
relevant to other issues related to -Zinstrument-coverage.

impl<'a> Linker for GccLinker {
   ...
   fn pgo_gen(&mut self) {
       if !self.sess.target.linker_is_gnu {
           return;
       }

       // If we're doing PGO generation stuff and on a GNU-like linker, use the
       // "-u" flag to properly pull in the profiler runtime bits.
       //
       // This is because LLVM otherwise won't add the needed initialization
       // for us on Linux (though the extra flag should be harmless if it
       // does).
       //
       // See https://reviews.llvm.org/D14033 and https://reviews.llvm.org/D14030.
       //
       // Though it may be worth to try to revert those changes upstream, since
       // the overhead of the initialization should be minor.
       self.cmd.arg("-u");
       self.cmd.arg("__llvm_profile_runtime");
   }
