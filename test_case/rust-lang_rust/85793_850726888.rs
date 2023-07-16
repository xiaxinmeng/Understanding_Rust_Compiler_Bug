
A: rustc --emit metadata -Zmetadata-link --crate-type rlib foo.rs & # generate libfoo.rmeta
B: rustc --emit metadata -Zmetadata-link --crate-type rlib bar.rs --extern foo=libfoo.rmeta & # generate libbar.rmeta for dependencies (not needed here)
C: wait A; rustc --emit link --crate-type rlib bar.rs --extern foo=libfoo.rmeta & # generate libbar.rlib for linking
D: rustc --emit link --crate-type rlib foo.rs & # libfoo.rlib (could be started at the same time as A)
E: wait C D; rustc --emit link --crate-type bin main.rs --extern bar=libbar.rlib -Ldependency=. # final executable
