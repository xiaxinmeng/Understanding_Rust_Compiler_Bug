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
    |
19  | pub type Priority = ();
    | ----------------------- previous definition of the type `Priority` here
...
109 | pub type Priority = ();
    | ^^^^^^^^^^^^^^^^^^^^^^^ `Priority` redefined here
    |
    = note: `Priority` must be defined only once in the type namespace of this module

error[E0428]: the name `Affinity` is defined multiple times
    |
20  | pub type Affinity = ();
20  | pub type Affinity = ();
    | ----------------------- previous definition of the type `Affinity` here
110 | pub type Affinity = ();
110 | pub type Affinity = ();
    | ^^^^^^^^^^^^^^^^^^^^^^^ `Affinity` redefined here
    |
    = note: `Affinity` must be defined only once in the type namespace of this module
For more information about this error, try `rustc --explain E0428`.
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:00:25
