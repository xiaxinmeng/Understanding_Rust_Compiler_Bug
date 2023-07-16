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
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
error[E0308]: mismatched types
   --> compiler/rustc_session/src/filesearch.rs:117:9
    |
86  | fn current_dll_path() -> Result<PathBuf, String> {
    |                          ----------------------- expected `Result<std::path::PathBuf, std::string::String>` because of return type
...
117 |         Some(PathBuf::from(os))
    |
    = note: expected enum `Result<std::path::PathBuf, std::string::String>`
               found enum `Option<std::path::PathBuf>`

