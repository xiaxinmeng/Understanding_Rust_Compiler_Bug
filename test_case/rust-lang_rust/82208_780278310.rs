plain
    Checking cargo_metadata v0.8.2
error[E0308]: mismatched types
   --> src/tools/rustfmt/src/syntux/session.rs:104:18
    |
104 |             Some(source_map.clone()),
    |                  ^^^^^^^^^^^^^^^^^^ expected struct `Lrc`, found struct `Rc`
    |
    = note: expected struct `Lrc<SourceMap>`
               found struct `Rc<SourceMap>`

error[E0277]: `Rc<IgnorePathSet>` cannot be sent between threads safely
   --> src/tools/rustfmt/src/syntux/session.rs:114:9
    |
114 | /         Box::new(SilentOnIgnoredFilesEmitter {
115 | |             has_non_ignorable_parser_errors: false,
117 | |             emitter,
118 | |             ignore_path_set,
119 | |             can_reset,
120 | |         }),
120 | |         }),
    | |__________^ `Rc<IgnorePathSet>` cannot be sent between threads safely
    |
    = help: within `SilentOnIgnoredFilesEmitter`, the trait `Send` is not implemented for `Rc<IgnorePathSet>`
    = note: required because it appears within the type `SilentOnIgnoredFilesEmitter`
    = note: required for the cast to the object type `dyn rustc_errors::emitter::Emitter + Send`

error[E0277]: `Rc<SourceMap>` cannot be sent between threads safely
   --> src/tools/rustfmt/src/syntux/session.rs:114:9
    |
114 | /         Box::new(SilentOnIgnoredFilesEmitter {
115 | |             has_non_ignorable_parser_errors: false,
117 | |             emitter,
118 | |             ignore_path_set,
119 | |             can_reset,
120 | |         }),
120 | |         }),
    | |__________^ `Rc<SourceMap>` cannot be sent between threads safely
    |
    = help: within `SilentOnIgnoredFilesEmitter`, the trait `Send` is not implemented for `Rc<SourceMap>`
    = note: required because it appears within the type `SilentOnIgnoredFilesEmitter`
    = note: required for the cast to the object type `dyn rustc_errors::emitter::Emitter + Send`

error[E0277]: `Rc<RefCell<bool>>` cannot be sent between threads safely
   --> src/tools/rustfmt/src/syntux/session.rs:114:9
    |
114 | /         Box::new(SilentOnIgnoredFilesEmitter {
115 | |             has_non_ignorable_parser_errors: false,
117 | |             emitter,
118 | |             ignore_path_set,
119 | |             can_reset,
120 | |         }),
120 | |         }),
    | |__________^ `Rc<RefCell<bool>>` cannot be sent between threads safely
    |
    = help: within `SilentOnIgnoredFilesEmitter`, the trait `Send` is not implemented for `Rc<RefCell<bool>>`
    = note: required because it appears within the type `SilentOnIgnoredFilesEmitter`
    = note: required for the cast to the object type `dyn rustc_errors::emitter::Emitter + Send`
error[E0308]: mismatched types
   --> src/tools/rustfmt/src/syntux/session.rs:139:67
    |
    |
139 |         let parse_sess = RawParseSess::with_span_handler(handler, source_map);
    |                                                                   ^^^^^^^^^^ expected struct `Lrc`, found struct `Rc`
    |
    = note: expected struct `Lrc<SourceMap>`
               found struct `Rc<SourceMap>`
error[E0308]: mismatched types
   --> src/tools/rustfmt/src/syntux/session.rs:213:23
    |
    |
213 |             Rc::clone(source_file.src.as_ref().unwrap()),
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Rc`, found struct `Lrc`
    |
    = note: expected reference `&Rc<std::string::String>`
               found reference `&Lrc<std::string::String>`
error[E0308]: mismatched types
   --> src/tools/rustfmt/src/syntux/session.rs:218:9
    |
    |
217 |       pub(crate) fn get_original_snippet(&self, file_name: &FileName) -> Option<Rc<String>> {
    |                                                                          ------------------ expected `std::option::Option<Rc<std::string::String>>` because of return type
218 | /         self.parse_sess
219 | |             .source_map()
220 | |             .get_source_file(&file_name.into())
221 | |             .and_then(|source_file| source_file.src.clone())
    | |____________________________________________________________^ expected struct `Rc`, found struct `Lrc`
    = note: expected enum `std::option::Option<Rc<std::string::String>>`
               found enum `std::option::Option<Lrc<std::string::String>>`

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> src/tools/rustfmt/src/syntux/session.rs:278:19
    |
278 |             file: lo.sf.clone(),
    |                   ^^^^^^^^^^^^^ expected struct `Rc`, found struct `Lrc`
    |
    = note: expected struct `Rc<rustc_span::SourceFile>`
               found struct `Lrc<rustc_span::SourceFile>`
error: aborting due to 8 previous errors

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
