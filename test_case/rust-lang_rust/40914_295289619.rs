

6:39 pm Luqman
TimNN: which was the branch you tested gh40914 with? I'd like to see figure out the cause of the assertion
6:39 pm [o__o]
Update LLVM to pull in patch that removes extraneous null check.: https://github.com/rust-lang/rust/pull/40914
6:40 pm TimNN
llvm40 in my rust-lang fork
6:40 pm
although I just backported a few llvm patches, which haven't been tested with
6:41 pm
so it would probably be best if you used llvm40 without the most recent commit
