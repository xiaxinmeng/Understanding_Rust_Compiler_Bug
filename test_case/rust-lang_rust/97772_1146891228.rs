plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking askama_shared v0.12.0
   Compiling askama_derive v0.11.0
    Checking askama v0.11.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0310]: the parameter type `F` may not live long enough
   --> src/librustdoc/docfs.rs:89:13
    |
89  | /             rayon::spawn(move || {
90  | |                 let file = match File::create(&path) {
91  | |                     Ok(f) => f,
92  | |                     Err(e) => {
103 | |                 });
104 | |             });
104 | |             });
    | |______________^ ...so that the type `F` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
    |
81  |         F: FnOnce(fs::File) -> io::Result<()> + Send + 'static,
    |                                                      +++++++++
