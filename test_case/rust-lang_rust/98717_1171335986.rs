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
   Compiling aho-corasick v0.7.18
   Compiling cargo_metadata v0.14.0
   Compiling regex v1.5.5
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
error: value assigned to `skip_leading_newlines` is never read
   --> src/tools/tidy/src/style.rs:171:13
    |
166 | / macro_rules! suppressible_tidy_err {
167 | |     ($err:ident, $skip:ident, $msg:expr) => {
168 | |         if let Directive::Deny = $skip {
169 | |             $err($msg);
170 | |         } else {
171 | |             $skip = Directive::Ignore(true);
172 | |         }
173 | |     };
174 | | }
174 | | }
    | |_- in this expansion of `suppressible_tidy_err!`
...
373 |               suppressible_tidy_err!(err, skip_leading_newlines, "mising leading newline");
    |
    |
    = note: `-D unused-assignments` implied by `-D warnings`
    = help: maybe it is overwritten before being read?

error: value assigned to `skip_trailing_newlines` is never read
   --> src/tools/tidy/src/style.rs:171:13
    |
166 | / macro_rules! suppressible_tidy_err {
167 | |     ($err:ident, $skip:ident, $msg:expr) => {
168 | |         if let Directive::Deny = $skip {
169 | |             $err($msg);
170 | |         } else {
171 | |             $skip = Directive::Ignore(true);
172 | |         }
173 | |     };
174 | | }
174 | | }
    | |_- in this expansion of `suppressible_tidy_err!`
...
379 |               0 => suppressible_tidy_err!(err, skip_trailing_newlines, "missing trailing newline"),
    |
    |
    = help: maybe it is overwritten before being read?

error: value assigned to `skip_trailing_newlines` is never read
   --> src/tools/tidy/src/style.rs:171:13
    |
166 | / macro_rules! suppressible_tidy_err {
167 | |     ($err:ident, $skip:ident, $msg:expr) => {
168 | |         if let Directive::Deny = $skip {
169 | |             $err($msg);
170 | |         } else {
171 | |             $skip = Directive::Ignore(true);
172 | |         }
173 | |     };
174 | | }
174 | | }
    | |_- in this expansion of `suppressible_tidy_err!`
...
381 |               n => suppressible_tidy_err!(
382 | |                 err,
383 | |                 skip_trailing_newlines,
383 | |                 skip_trailing_newlines,
384 | |                 &format!("too many trailing newlines ({n})")
    | |_____________- in this macro invocation
    |
    |
    = help: maybe it is overwritten before being read?

error: value assigned to `skip_file_length` is never read
   --> src/tools/tidy/src/style.rs:171:13
    |
166 | / macro_rules! suppressible_tidy_err {
167 | |     ($err:ident, $skip:ident, $msg:expr) => {
168 | |         if let Directive::Deny = $skip {
169 | |             $err($msg);
170 | |         } else {
171 | |             $skip = Directive::Ignore(true);
172 | |         }
173 | |     };
174 | | }
174 | | }
    | |_- in this expansion of `suppressible_tidy_err!`
...
397 |               suppressible_tidy_err!(err, skip_file_length, "");
    |
    |
    = help: maybe it is overwritten before being read?
error: could not compile `tidy` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:00:07
