plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
 Documenting core v0.0.0 (/checkout/library/core)
error: could not parse code block as Rust code
   --> library/core/src/num/nonzero.rs:912:17
    |
912 | /                 #[doc = concat!("The minimum value for a `", stringify!($Ty), " `.")]
913 | |                 /// # Examples
915 | |                 /// 