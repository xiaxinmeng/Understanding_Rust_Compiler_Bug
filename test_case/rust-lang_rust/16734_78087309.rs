
davidrhyswhite - Don't suppose anyone could help me understand why I get a 'linking with cc failed exit code 1' when I have a struct nested inside another struct and attempt to call a method on the child struct?
davidrhyswhite - Have a git repo with an example here: https://github.com/davidrhyswhite/rust-structs-example
krdln          - davidrhyswhite: Changing mod child to pub mod child works. You should file a bug, but I'm not shure what should be expected behaviour
mbrubeck       - davidrhyswhite: `cargo build` succeeds for me with only a dead_code warning
davidrhyswhite - Yeah cargo build was fine it was running 'cargo run --example example' that fails for me
mbrubeck       - ah, got it
krdln          - davidrhyswhite: Changing `use child` into `pub use child` works too
davidrhyswhite - Awesome thanks krdln been stuck on this for a while
mbrubeck       - davidrhyswhite: an example is compiled as a separate crate, so it can only call exported (`pub`) code from the main crate.
mbrubeck       - It'd be nice to have better diagnostics instead of just a linker error in cases like this...
krdln          - davidrhyswhite: Rust should either forbid returning private things from public funtions, or it should forbid calling method in example. Either way, file a bug
davidrhyswhite - mbrubeck: Okay, but I though since the child member on the Parent struct was pub and the method was pub too it would allow me to call it
krdln          - davidrhyswhite: Notice that you get warned about Child::method when building a lib
mbrubeck       - since `child::Child::method` is not a publicly-accessible path, it's not exported by the library, so other crates can't link to it.
aawe           - so the rust linker could use a patch for better error messages then
davidrhyswhite - mbrubeck: Thanks, that makes a bit more sense now.
mbrubeck       - aawe: Yeah, it'd be nice if rustc could thrown an error while compiling example.rs, instead of waiting for linking to fail.
mbrubeck       - The error is actually ends up coming from the system linker, gcc in my case.
