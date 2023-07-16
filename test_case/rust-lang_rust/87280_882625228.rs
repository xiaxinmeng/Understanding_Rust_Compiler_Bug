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
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> compiler/rustc_mir/src/dataflow/framework/tests.rs:60:5
    |
60  |     mir::Body::new_cfg_only(blocks)
    |     |
    |     expected 2 arguments
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/compiler/rustc_middle/src/mir/mod.rs:297:12
    |
297 |     pub fn new_cfg_only(

error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> compiler/rustc_mir/src/transform/coverage/tests.rs:179:9
    |
    |
179 |         Body::new_cfg_only(self.blocks)
    |         |
    |         expected 2 arguments
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/compiler/rustc_middle/src/mir/mod.rs:297:12
    |
297 |     pub fn new_cfg_only(

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0061`.
