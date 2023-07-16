plain
2019-12-29T11:55:32.5525165Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-29T11:55:32.5734053Z ##[command]git config gc.auto 0
2019-12-29T11:55:32.5810301Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-29T11:55:32.5887901Z ##[command]git config --get-all http.proxy
2019-12-29T11:55:32.6031156Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67702/merge:refs/remotes/pull/67702/merge
---
2019-12-29T12:02:58.8422056Z     Checking syntax_expand v0.0.0 (/checkout/src/libsyntax_expand)
2019-12-29T12:02:59.0252177Z error[E0432]: unresolved import `unicode_normalization`
2019-12-29T12:02:59.0252571Z    --> src/libsyntax_expand/proc_macro_server.rs:330:13
2019-12-29T12:02:59.0252783Z     |
2019-12-29T12:02:59.0253079Z 330 |         use unicode_normalization::{is_nfc_quick, IsNormalized, UnicodeNormalization};
2019-12-29T12:02:59.0253527Z 
2019-12-29T12:02:59.0970874Z error[E0412]: cannot find type `SymbolStr` in module `ast`
2019-12-29T12:02:59.0971421Z    --> src/libsyntax_expand/proc_macro_server.rs:329:52
2019-12-29T12:02:59.0971636Z     |
2019-12-29T12:02:59.0971636Z     |
2019-12-29T12:02:59.0971927Z 329 |     fn nfc_normalize(sym: Symbol) -> (Symbol, ast::SymbolStr) {
2019-12-29T12:02:59.0972397Z     |
2019-12-29T12:02:59.0972633Z help: a struct with a similar name exists
2019-12-29T12:02:59.0972830Z     |
2019-12-29T12:02:59.0972830Z     |
2019-12-29T12:02:59.0973099Z 329 |     fn nfc_normalize(sym: Symbol) -> (Symbol, ast::Symbol) {
2019-12-29T12:02:59.0973655Z help: possible candidates are found in other modules, you can import them into scope
2019-12-29T12:02:59.0973845Z     |
2019-12-29T12:02:59.0974175Z 1   | use syntax::symbol::SymbolStr;
2019-12-29T12:02:59.0974363Z     |
2019-12-29T12:02:59.0974363Z     |
2019-12-29T12:02:59.0974784Z 1   | use syntax_pos::symbol::SymbolStr;
2019-12-29T12:02:59.0974999Z     |
2019-12-29T12:02:59.0975032Z 
2019-12-29T12:02:59.4117789Z error[E0599]: no method named `nfc` found for type `std::str::Chars<'_>` in the current scope
2019-12-29T12:02:59.4118253Z    --> src/libsyntax_expand/proc_macro_server.rs:335:54
2019-12-29T12:02:59.4118507Z     |
2019-12-29T12:02:59.4119022Z 335 |                 let sym_str: String = string.chars().nfc().collect();
2019-12-29T12:02:59.4119413Z     |                                                      ^^^ method not found in `std::str::Chars<'_>`
2019-12-29T12:02:59.4120080Z     = help: items from traits can only be used if the trait is in scope
2019-12-29T12:02:59.4120080Z     = help: items from traits can only be used if the trait is in scope
2019-12-29T12:02:59.4120427Z     = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2019-12-29T12:02:59.4120722Z             `use unicode_normalization::UnicodeNormalization;`
2019-12-29T12:02:59.4219563Z error[E0277]: the size for values of type `str` cannot be known at compilation time
2019-12-29T12:02:59.4219929Z    --> src/libsyntax_expand/proc_macro_server.rs:343:28
2019-12-29T12:02:59.4220135Z     |
2019-12-29T12:02:59.4220390Z 343 |         if !Self::is_valid(&string) {
---
2019-12-29T12:02:59.4222197Z 
2019-12-29T12:02:59.4383406Z error[E0277]: the size for values of type `str` cannot be known at compilation time
2019-12-29T12:02:59.4383962Z    --> <::std::macros::panic macros>:10:9
2019-12-29T12:02:59.4384191Z     |
2019-12-29T12:02:59.4384575Z 1   | / () => ({ $ crate :: panic ! ("explicit panic") }) ; ($ msg : expr) =>
2019-12-29T12:02:59.4385181Z 2   | | ({
2019-12-29T12:02:59.4385471Z 3   | |      $ crate :: rt :: begin_panic
2019-12-29T12:02:59.4385870Z 4   | |      ($ msg, &
2019-12-29T12:02:59.4386411Z 10  | |      (& $ crate :: format_args ! ($ fmt, $ ($ arg) +), &
2019-12-29T12:02:59.4386711Z     | |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-29T12:02:59.4387133Z     | |         |
2019-12-29T12:02:59.4387500Z     | |         doesn't have a size known at compile-time
2019-12-29T12:02:59.4387500Z     | |         doesn't have a size known at compile-time
2019-12-29T12:02:59.4387786Z     | |         in this macro invocation
2019-12-29T12:02:59.4388390Z 11  | |       ($ crate :: file ! (), $ crate :: line ! (), $ crate :: column ! ()))
2019-12-29T12:02:59.4388944Z     | |_____- in this expansion of `panic!`
2019-12-29T12:02:59.4389155Z     | 
2019-12-29T12:02:59.4389398Z    ::: /checkout/src/libcore/fmt/mod.rs:277:20
2019-12-29T12:02:59.4389597Z     |
2019-12-29T12:02:59.4389597Z     |
2019-12-29T12:02:59.4389924Z 277 |       pub fn new<'b, T>(x: &'b T, f: fn(&T, &mut Formatter<'_>) -> Result) -> ArgumentV1<'b> {
2019-12-29T12:02:59.4390418Z     |                      - required by this bound in `std::fmt::ArgumentV1::<'a>::new`
2019-12-29T12:02:59.4390858Z    ::: src/libsyntax_expand/proc_macro_server.rs:344:13
2019-12-29T12:02:59.4391044Z     |
2019-12-29T12:02:59.4391044Z     |
2019-12-29T12:02:59.4391311Z 344 |               panic!("`{:?}` is not a valid identifier", string)
2019-12-29T12:02:59.4392056Z     | 
2019-12-29T12:02:59.4392492Z    ::: <::core::macros::builtin::format_args macros>:1:1
2019-12-29T12:02:59.4392722Z     |
2019-12-29T12:02:59.4392722Z     |
2019-12-29T12:02:59.4393019Z 1   |   ($ fmt : expr) => { { } } ; ($ fmt : expr, $ ($ args : tt) *) => { { } } ;
2019-12-29T12:02:59.4393614Z     |
2019-12-29T12:02:59.4393885Z     = help: the trait `std::marker::Sized` is not implemented for `str`
2019-12-29T12:02:59.4394261Z     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-12-29T12:02:59.4394310Z 
2019-12-29T12:02:59.4394310Z 
2019-12-29T12:02:59.4536532Z error[E0277]: the size for values of type `str` cannot be known at compilation time
2019-12-29T12:02:59.4536852Z    --> <::std::macros::panic macros>:10:9
2019-12-29T12:02:59.4537040Z     |
2019-12-29T12:02:59.4537344Z 1   | / () => ({ $ crate :: panic ! ("explicit panic") }) ; ($ msg : expr) =>
2019-12-29T12:02:59.4537582Z 2   | | ({
2019-12-29T12:02:59.4537837Z 3   | |      $ crate :: rt :: begin_panic
2019-12-29T12:02:59.4538246Z 4   | |      ($ msg, &
2019-12-29T12:02:59.4538778Z 10  | |      (& $ crate :: format_args ! ($ fmt, $ ($ arg) +), &
2019-12-29T12:02:59.4539185Z     | |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-29T12:02:59.4539422Z     | |         |
2019-12-29T12:02:59.4539701Z     | |         doesn't have a size known at compile-time
2019-12-29T12:02:59.4539701Z     | |         doesn't have a size known at compile-time
2019-12-29T12:02:59.4539958Z     | |         in this macro invocation
2019-12-29T12:02:59.4540258Z 11  | |       ($ crate :: file ! (), $ crate :: line ! (), $ crate :: column ! ()))
2019-12-29T12:02:59.4540787Z     | |_____- in this expansion of `panic!`
2019-12-29T12:02:59.4540969Z     | 
2019-12-29T12:02:59.4541219Z    ::: /checkout/src/libcore/fmt/mod.rs:277:20
2019-12-29T12:02:59.4541398Z     |
2019-12-29T12:02:59.4541398Z     |
2019-12-29T12:02:59.4541679Z 277 |       pub fn new<'b, T>(x: &'b T, f: fn(&T, &mut Formatter<'_>) -> Result) -> ArgumentV1<'b> {
2019-12-29T12:02:59.4542005Z     |                      - required by this bound in `std::fmt::ArgumentV1::<'a>::new`
2019-12-29T12:02:59.4542434Z    ::: src/libsyntax_expand/proc_macro_server.rs:347:13
2019-12-29T12:02:59.4542624Z     |
2019-12-29T12:02:59.4542624Z     |
2019-12-29T12:02:59.4542882Z 347 |               panic!("`{}` cannot be a raw identifier", string);
2019-12-29T12:02:59.4543570Z     | 
2019-12-29T12:02:59.4543800Z    ::: <::core::macros::builtin::format_args macros>:1:1
2019-12-29T12:02:59.4544003Z     |
2019-12-29T12:02:59.4544003Z     |
2019-12-29T12:02:59.4544286Z 1   |   ($ fmt : expr) => { { } } ; ($ fmt : expr, $ ($ args : tt) *) => { { } } ;
2019-12-29T12:02:59.4545186Z     |
2019-12-29T12:02:59.4545441Z     = help: the trait `std::marker::Sized` is not implemented for `str`
2019-12-29T12:02:59.4545758Z     = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-12-29T12:02:59.4545815Z 
2019-12-29T12:02:59.4545815Z 
2019-12-29T12:03:00.4382011Z error: aborting due to 6 previous errors
2019-12-29T12:03:00.4382877Z 
2019-12-29T12:03:00.6657878Z Some errors have detailed explanations: E0277, E0412, E0432, E0599.
2019-12-29T12:03:00.6659096Z For more information about an error, try `rustc --explain E0277`.
2019-12-29T12:03:00.6659465Z error: could not compile `syntax_expand`.
2019-12-29T12:03:00.6660089Z warning: build failed, waiting for other jobs to finish...
2019-12-29T12:03:57.7756210Z error: build failed
2019-12-29T12:03:57.7780785Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-12-29T12:03:57.7804580Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-12-29T12:03:57.7804686Z Build completed unsuccessfully in 0:05:04
2019-12-29T12:03:57.7855476Z == clock drift check ==
2019-12-29T12:03:57.7873318Z   local time: Sun Dec 29 12:03:57 UTC 2019
2019-12-29T12:03:57.7873318Z   local time: Sun Dec 29 12:03:57 UTC 2019
2019-12-29T12:03:58.0681837Z   network time: Sun, 29 Dec 2019 12:03:58 GMT
2019-12-29T12:03:58.0682617Z == end clock drift check ==
2019-12-29T12:03:59.1118988Z 
2019-12-29T12:03:59.1212609Z ##[error]Bash exited with code '1'.
2019-12-29T12:03:59.1243084Z ##[section]Starting: Checkout
2019-12-29T12:03:59.1244778Z ==============================================================================
2019-12-29T12:03:59.1244829Z Task         : Get sources
2019-12-29T12:03:59.1244870Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
