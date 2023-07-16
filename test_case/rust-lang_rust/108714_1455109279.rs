plain
143 |       Handler::with_emitter(
    |  _____^^^^^^^^^^^^^^^^^^^^^-
144 | |         true,
145 | |         None,
146 | |         Box::new(SilentOnIgnoredFilesEmitter {
152 | |         }),
153 | |     )
    | |_____- an argument of type `std::option::Option<PathBuf>` is missing
    |
    |
note: associated function defined here
   --> /checkout/compiler/rustc_errors/src/lib.rs:593:12
    |
593 |     pub fn with_emitter(
    |            ^^^^^^^^^^^^
help: provide the argument
    |
143 ~     Handler::with_emitter(true, None, Box::new(SilentOnIgnoredFilesEmitter {
144 +             has_non_ignorable_parser_errors: false,
146 +             emitter,
147 +             ignore_path_set,
148 +             can_reset,
148 +             can_reset,
149 +         }), /* std::option::Option<PathBuf> */)

error[E0061]: this function takes 4 arguments but 3 arguments were supplied
   --> src/tools/rustfmt/src/parse/session.rs:223:43
    |
    |
223 |         self.parse_sess.span_diagnostic = Handler::with_emitter(true, None, silent_emitter());
    |                                           ^^^^^^^^^^^^^^^^^^^^^------------------------------ an argument of type `std::option::Option<PathBuf>` is missing
note: associated function defined here
   --> /checkout/compiler/rustc_errors/src/lib.rs:593:12
    |
593 |     pub fn with_emitter(
593 |     pub fn with_emitter(
    |            ^^^^^^^^^^^^
help: provide the argument
    |
223 |         self.parse_sess.span_diagnostic = Handler::with_emitter(true, None, silent_emitter(), /* std::option::Option<PathBuf> */);

For more information about this error, try `rustc --explain E0061`.
error: could not compile `rustfmt-nightly` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
