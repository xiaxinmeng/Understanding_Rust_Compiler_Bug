
[00:02:35] error: no method named `supports_reset` found for type `&mut std::boxed::Box<term::Terminal<Output=std::io::Stderr> + std::marker::Send + 'static>` in the current scope
[00:02:35]     --> /checkout/src/librustc_errors/emitter.rs:1379:22
[00:02:35]      |
[00:02:35] 1379 |                 if t.supports_reset() {
[00:02:35]      |                      ^^^^^^^^^^^^^^
[00:02:35] 
[00:02:35] error: no method named `supports_reset` found for type `&mut std::boxed::Box<term::Terminal<Output=emitter::BufferedWriter> + std::marker::Send + 'static>` in the current scope
[00:02:35]     --> /checkout/src/librustc_errors/emitter.rs:1384:22
[00:02:35]      |
[00:02:35] 1384 |                 if t.supports_reset() {
[00:02:35]      |                      ^^^^^^^^^^^^^^
[00:02:35] 
[00:02:35] error: aborting due to 2 previous errors
