plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
error[E0277]: `Cell<*mut u8>` cannot be shared between threads safely
    |
    |
176 |     thread::spawn(move || {
    |     ^^^^^^^^^^^^^ `Cell<*mut u8>` cannot be shared between threads safely
   ::: /checkout/library/std/src/thread/mod.rs:619:8
    |
619 |     F: Send + 'static,
619 |     F: Send + 'static,
    |        ---- required by this bound in `std::thread::spawn`
    |
    = help: within `SessionGlobals`, the trait `std::marker::Sync` is not implemented for `Cell<*mut u8>`
    = note: required because it appears within the type `rustc_arena::DroplessArena`
    = note: required because it appears within the type `Interner`
    = note: required because it appears within the type `SessionGlobals`
    = note: required because of the requirements on the impl of `Send` for `&SessionGlobals`
    = note: required because it appears within the type `[closure@compiler/rustc_interface/src/util.rs:176:19: 182:6]`

error[E0277]: `std::cell::RefCell<Vec<rustc_arena::TypedArenaChunk<u8>>>` cannot be shared between threads safely
    |
    |
176 |     thread::spawn(move || {
    |     ^^^^^^^^^^^^^ `std::cell::RefCell<Vec<rustc_arena::TypedArenaChunk<u8>>>` cannot be shared between threads safely
   ::: /checkout/library/std/src/thread/mod.rs:619:8
    |
619 |     F: Send + 'static,
619 |     F: Send + 'static,
    |        ---- required by this bound in `std::thread::spawn`
    |
    = help: within `SessionGlobals`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<Vec<rustc_arena::TypedArenaChunk<u8>>>`
    = note: required because it appears within the type `rustc_arena::DroplessArena`
    = note: required because it appears within the type `Interner`
    = note: required because it appears within the type `SessionGlobals`
    = note: required because of the requirements on the impl of `Send` for `&SessionGlobals`
    = note: required because it appears within the type `[closure@compiler/rustc_interface/src/util.rs:176:19: 182:6]`

error[E0277]: `Cell<*mut u8>` cannot be shared between threads safely
    |
    |
212 |               let main_handler = move |thread: rayon::ThreadBuilder| {
    |  ________________________________-
213 | |                 rustc_span::set_session_globals_then(session_globals, || {
214 | |                     io::set_output_capture(stderr.clone());
215 | |                     thread.run()
217 | |             };
217 | |             };
    | |_____________- within this `[closure@compiler/rustc_interface/src/util.rs:212:32: 217:14]`
218 | 
219 |               config.build_scoped(main_handler, with_pool).unwrap()
    |                      ^^^^^^^^^^^^ `Cell<*mut u8>` cannot be shared between threads safely
    |
    = help: within `[closure@compiler/rustc_interface/src/util.rs:212:32: 217:14]`, the trait `std::marker::Sync` is not implemented for `Cell<*mut u8>`
    = note: required because it appears within the type `rustc_arena::DroplessArena`
    = note: required because it appears within the type `Interner`
    = note: required because it appears within the type `SessionGlobals`
    = note: required because it appears within the type `&SessionGlobals`
    = note: required because it appears within the type `[closure@compiler/rustc_interface/src/util.rs:212:32: 217:14]`

error[E0277]: `std::cell::RefCell<Vec<rustc_arena::TypedArenaChunk<u8>>>` cannot be shared between threads safely
    |
    |
212 |               let main_handler = move |thread: rayon::ThreadBuilder| {
    |  ________________________________-
213 | |                 rustc_span::set_session_globals_then(session_globals, || {
214 | |                     io::set_output_capture(stderr.clone());
215 | |                     thread.run()
217 | |             };
217 | |             };
    | |_____________- within this `[closure@compiler/rustc_interface/src/util.rs:212:32: 217:14]`
218 | 
219 |               config.build_scoped(main_handler, with_pool).unwrap()
    |                      ^^^^^^^^^^^^ `std::cell::RefCell<Vec<rustc_arena::TypedArenaChunk<u8>>>` cannot be shared between threads safely
    |
    = help: within `[closure@compiler/rustc_interface/src/util.rs:212:32: 217:14]`, the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<Vec<rustc_arena::TypedArenaChunk<u8>>>`
    = note: required because it appears within the type `rustc_arena::DroplessArena`
    = note: required because it appears within the type `Interner`
    = note: required because it appears within the type `SessionGlobals`
    = note: required because it appears within the type `&SessionGlobals`
    = note: required because it appears within the type `[closure@compiler/rustc_interface/src/util.rs:212:32: 217:14]`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_interface`
