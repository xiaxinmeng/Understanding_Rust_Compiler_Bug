plain
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error[E0061]: this function takes 1 argument but 0 arguments were supplied
    --> compiler/rustc_codegen_ssa/src/back/linker.rs:1074:22
     |
1074 |                 self.build_dynamic_exe();
     |                      ^^^^^^^^^^^^^^^^^-- an argument of type `&std::path::Path` is missing
note: associated function defined here
    --> compiler/rustc_codegen_ssa/src/back/linker.rs:1040:8
     |
     |
1040 |     fn build_dynamic_exe(&mut self, _out_filename: &Path) {
help: provide the argument
     |
     |
1074 |                 self.build_dynamic_exe({&std::path::Path});

error[E0061]: this function takes 1 argument but 0 arguments were supplied
    --> compiler/rustc_codegen_ssa/src/back/linker.rs:1077:22
     |
     |
1077 |                 self.build_dynamic_exe();
     |                      ^^^^^^^^^^^^^^^^^-- an argument of type `&std::path::Path` is missing
note: associated function defined here
    --> compiler/rustc_codegen_ssa/src/back/linker.rs:1040:8
     |
     |
1040 |     fn build_dynamic_exe(&mut self, _out_filename: &Path) {
help: provide the argument
     |
     |
1077 |                 self.build_dynamic_exe({&std::path::Path});

error[E0061]: this function takes 1 argument but 0 arguments were supplied
    --> compiler/rustc_codegen_ssa/src/back/linker.rs:1084:22
     |
     |
1084 |                 self.build_dylib();
     |                      ^^^^^^^^^^^-- an argument of type `&std::path::Path` is missing
note: associated function defined here
    --> compiler/rustc_codegen_ssa/src/back/linker.rs:1052:8
     |
     |
1052 |     fn build_dylib(&mut self, _out_filename: &Path) {
help: provide the argument
     |
     |
1084 |                 self.build_dylib({&std::path::Path});

error[E0061]: this function takes 1 argument but 0 arguments were supplied
    --> compiler/rustc_codegen_ssa/src/back/linker.rs:1087:22
     |
     |
1087 |                 self.build_dylib();
     |                      ^^^^^^^^^^^-- an argument of type `&std::path::Path` is missing
note: associated function defined here
    --> compiler/rustc_codegen_ssa/src/back/linker.rs:1052:8
     |
     |
1052 |     fn build_dylib(&mut self, _out_filename: &Path) {
help: provide the argument
     |
     |
1087 |                 self.build_dylib({&std::path::Path});


error[E0599]: no method named `linker_args` found for mutable reference `&mut EmLinker<'a>` in the current scope
    --> compiler/rustc_codegen_ssa/src/back/linker.rs:1090:22
     |
1090 |                 self.linker_args(&["--entry", "_initialize", "-sSTANDALONE_WASM"]);
     |                      ^^^^^^^^^^^ method not found in `&mut EmLinker<'a>`
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
Some errors have detailed explanations: E0061, E0599.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `rustc_codegen_ssa` due to 5 previous errors
