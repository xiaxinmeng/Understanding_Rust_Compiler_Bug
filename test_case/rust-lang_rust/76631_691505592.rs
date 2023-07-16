
$ ./x.py check
Updating only changed submodules
Submodules updated in 0.01 seconds
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
warning: you have not made a `config.toml`
help: consider running `x.py setup` or copying `config.toml.example`
...
$ ./x.py setup
Updating only changed submodules
Submodules updated in 0.01 seconds
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
Welcome to the Rust project! What do you want to do with x.py?
a) Contribute to the standard library
b) Contribute to the compiler
c) Contribute to the compiler, and also modify LLVM or codegen
d) Install Rust from source
Please choose one (a/b/c/d): a
`x.py` will now use the configuration at /home/joshua/rustc2/src/bootstrap/defaults/config.toml.library
To get started, try one of the following commands:
- `x.py check`
- `x.py build`
- `x.py test library/std`
- `x.py doc`
For more suggestions, see https://rustc-dev-guide.rust-lang.org/building/suggested.html
Build completed successfully in 0:00:05
$ ./x.py setup
Welcome to the Rust project! What do you want to do with x.py?
a) Contribute to the standard library
b) Contribute to the compiler
c) Contribute to the compiler, and also modify LLVM or codegen
d) Install Rust from source
Please choose one (a/b/c/d): a
error: you asked `x.py` to setup a new config file, but one already exists at `config.toml`
help: try adding `profile = "library"` at the top of config.toml
note: this will use the configuration in /home/joshua/rustc2/src/bootstrap/defaults/config.toml.library
failed to run: /home/joshua/rustc2/build/bootstrap/debug/bootstrap setup
Build completed unsuccessfully in 0:00:01
