
src/bootstrap/compile.rs
692:        if cfg!(test) { // codegen backends -- we expect at least one to have been built, but we build none as we haven't run cargo
743:        if cfg!(test) { return; } // parsing filesystem path
1003:    if cfg!(test) { return Vec::new(); } // cargo is run via `Command::spawn` instead of traditional APIs

src/bootstrap/lib.rs
447:        if !cfg!(test) { // parsing command output, will panic if unexpected output
675:        if cfg!(test) { return; } // running command
682:        if cfg!(test) { return; } // running command
691:        if cfg!(test) { return true; } // running command
700:        if cfg!(test) { return true; } // running command
998:        if cfg!(test) { return String::from("0.1.2"); } // parsing file system

src/bootstrap/native.rs # these are all because we can't easily stub out the cc crate's run
62:        if cfg!(test) {
339:        if cfg!(test) {
395:        if cfg!(test) {
450:        if cfg!(test) {

src/bootstrap/util.rs
213:    if cfg!(test) { return Ok(()); } # the windows code uses low-level APIs that we can't really stub out

