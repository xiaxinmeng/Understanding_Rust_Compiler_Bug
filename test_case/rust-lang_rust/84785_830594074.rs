plain
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "CI_ONLY_WHEN_SUBMODULES_CHANGED": 1
##[endgroup]
adding extra environment variable CI_ONLY_WHEN_SUBMODULES_CHANGED
linux builder detected, using docker to run the build
##[group]Run src/ci/scripts/should-skip-this.sh
---

---- compile_test stdout ----
diff of stderr:

 error: wildcard match will also match any future added variants
   --> $DIR/wildcard_enum_match_arm.rs:39:9
    |
 LL |         _ => eprintln!("Not red"),
    |         ^ help: try this: `Color::Green | Color::Blue | Color::Rgb(..) | Color::Cyan`
 note: the lint level is defined here
   --> $DIR/wildcard_enum_match_arm.rs:3:9
    |
    |
 LL | #![deny(clippy::wildcard_enum_match_arm)]
 
 
 error: wildcard match will also match any future added variants
   --> $DIR/wildcard_enum_match_arm.rs:43:9
    |
 LL |         _not_red => eprintln!("Not red"),
    |         ^^^^^^^^ help: try this: `_not_red @ Color::Green | _not_red @ Color::Blue | _not_red @ Color::Rgb(..) | _not_red @ Color::Cyan`
 
 error: wildcard match will also match any future added variants
   --> $DIR/wildcard_enum_match_arm.rs:47:9
    |
 LL |         not_red => format!("{:?}", not_red),
    |         ^^^^^^^ help: try this: `not_red @ Color::Green | not_red @ Color::Blue | not_red @ Color::Rgb(..) | not_red @ Color::Cyan`
 
 error: wildcard match will also match any future added variants
   --> $DIR/wildcard_enum_match_arm.rs:63:9
error: test failed, to rerun pass '--test compile-test'
error: test failed, to rerun pass '--test compile-test'
 LL |         _ => "No red",
    |         ^ help: try this: `Color::Red | Color::Green | Color::Blue | Color::Rgb(..) | Color::Cyan`
 
 error: wildcard matches known variants and will also match future added variants
   --> $DIR/wildcard_enum_match_arm.rs:80:9
 LL |         _ => {},
 LL |         _ => {},
-   |         ^ help: try this: `ErrorKind::PermissionDenied | ErrorKind::ConnectionRefused | ErrorKind::ConnectionReset | ErrorKind::ConnectionAborted | ErrorKind::NotConnected | ErrorKind::AddrInUse | ErrorKind::AddrNotAvailable | ErrorKind::BrokenPipe | ErrorKind::AlreadyExists | ErrorKind::WouldBlock | ErrorKind::InvalidInput | ErrorKind::InvalidData | ErrorKind::TimedOut | ErrorKind::WriteZero | ErrorKind::Interrupted | ErrorKind::Other | ErrorKind::UnexpectedEof | ErrorKind::Unsupported | _`
+   |         ^ help: try this: `ErrorKind::PermissionDenied | ErrorKind::ConnectionRefused | ErrorKind::ConnectionReset | ErrorKind::ConnectionAborted | ErrorKind::NotConnected | ErrorKind::AddrInUse | ErrorKind::AddrNotAvailable | ErrorKind::BrokenPipe | ErrorKind::AlreadyExists | ErrorKind::WouldBlock | ErrorKind::InvalidInput | ErrorKind::InvalidData | ErrorKind::TimedOut | ErrorKind::WriteZero | ErrorKind::Interrupted | ErrorKind::Other | ErrorKind::UnexpectedEof | ErrorKind::Unsupported | ErrorKind::OutOfMemory | _`
-error: aborting due to 5 previous errors
-error: aborting due to 5 previous errors
+error: wildcard matches known variants and will also match future added variants
+  --> $DIR/wildcard_enum_match_arm.rs:102:9
+LL |         _ => {},
+LL |         _ => {},
+   |         ^ help: try this: `ErrorKind::OutOfMemory | _`
+error: aborting due to 6 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/wildcard_enum_match_arm.stage-id.stderr

 // run-rustfix
 
 
 #![deny(clippy::wildcard_enum_match_arm)]
 #![allow(
     unreachable_code,
     unused_variables,
     clippy::single_match,
     clippy::single_match,
     clippy::wildcard_in_or_patterns,
     clippy::unnested_or_patterns,
     clippy::diverging_sub_expression
 
 use std::io::ErrorKind;
 
 
 #[derive(Clone, Copy, Debug, Eq, PartialEq)]
 enum Color {
     Red,
     Green,
     Blue,
     Rgb(u8, u8, u8),
     Cyan,
 
 impl Color {
 impl Color {
     fn is_monochrome(self) -> bool {
         match self {
             Color::Red | Color::Green | Color::Blue => true,
             Color::Rgb(r, g, b) => r | g == 0 || r | b == 0 || g | b == 0,
             Color::Cyan => false,
     }
 }
 
 fn main() {
 fn main() {
     let color = Color::Rgb(0, 0, 127);
     match color {
         Color::Red => println!("Red"),
         Color::Green | Color::Blue | Color::Rgb(..) | Color::Cyan => eprintln!("Not red"),
     match color {
     match color {
         Color::Red => println!("Red"),
         _not_red @ Color::Green | _not_red @ Color::Blue | _not_red @ Color::Rgb(..) | _not_red @ Color::Cyan => eprintln!("Not red"),
     let _str = match color {
     let _str = match color {
         Color::Red => "Red".to_owned(),
         not_red @ Color::Green | not_red @ Color::Blue | not_red @ Color::Rgb(..) | not_red @ Color::Cyan => format!("{:?}", not_red),
     match color {
     match color {
         Color::Red => {},
         Color::Green => {},
         Color::Blue => {},
         Color::Cyan => {},
         c if c.is_monochrome() => {},
         Color::Rgb(_, _, _) => {},
     let _str = match color {
     let _str = match color {
         Color::Red => "Red",
         c @ Color::Green | c @ Color::Blue | c @ Color::Rgb(_, _, _) | c @ Color::Cyan => "Not red",
     match color {
     match color {
         Color::Rgb(r, _, _) if r > 0 => "Some red",
         Color::Red | Color::Green | Color::Blue | Color::Rgb(..) | Color::Cyan => "No red",
     match color {
     match color {
         Color::Red | Color::Green | Color::Blue | Color::Cyan => {},
         Color::Rgb(..) => {},
     };
     let x: u8 = unimplemented!();
     match x {
         0 => {},
         140 => {},
         _ => {},
     };
     // We need to use an enum not defined in this test because non_exhaustive is ignored for the
     // purposes of dead code analysis within a crate.
     let error_kind = ErrorKind::NotFound;
     match error_kind {
         ErrorKind::NotFound => {},
-        ErrorKind::PermissionDenied | ErrorKind::ConnectionRefused | ErrorKind::ConnectionReset | ErrorKind::ConnectionAborted | ErrorKind::NotConnected | ErrorKind::AddrInUse | ErrorKind::AddrNotAvailable | ErrorKind::BrokenPipe | ErrorKind::AlreadyExists | ErrorKind::WouldBlock | ErrorKind::InvalidInput | ErrorKind::InvalidData | ErrorKind::TimedOut | ErrorKind::WriteZero | ErrorKind::Interrupted | ErrorKind::Other | ErrorKind::UnexpectedEof | ErrorKind::Unsupported | _ => {},
+        ErrorKind::PermissionDenied | ErrorKind::ConnectionRefused | ErrorKind::ConnectionReset | ErrorKind::ConnectionAborted | ErrorKind::NotConnected | ErrorKind::AddrInUse | ErrorKind::AddrNotAvailable | ErrorKind::BrokenPipe | ErrorKind::AlreadyExists | ErrorKind::WouldBlock | ErrorKind::InvalidInput | ErrorKind::InvalidData | ErrorKind::TimedOut | ErrorKind::WriteZero | ErrorKind::Interrupted | ErrorKind::Other | ErrorKind::UnexpectedEof | ErrorKind::Unsupported | ErrorKind::OutOfMemory | _ => {},
     match error_kind {
         ErrorKind::NotFound => {},
         ErrorKind::NotFound => {},
         ErrorKind::PermissionDenied => {},
         ErrorKind::ConnectionRefused => {},
         ErrorKind::ConnectionReset => {},
         ErrorKind::ConnectionAborted => {},
         ErrorKind::NotConnected => {},
         ErrorKind::AddrInUse => {},
         ErrorKind::AddrNotAvailable => {},
         ErrorKind::BrokenPipe => {},
         ErrorKind::AlreadyExists => {},
         ErrorKind::WouldBlock => {},
         ErrorKind::InvalidInput => {},
         ErrorKind::InvalidData => {},
         ErrorKind::TimedOut => {},
         ErrorKind::WriteZero => {},
         ErrorKind::Interrupted => {},
         ErrorKind::Other => {},
         ErrorKind::UnexpectedEof => {},
         ErrorKind::Unsupported => {},
-        _ => {},
+        ErrorKind::OutOfMemory | _ => {},
 }
 

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/wildcard_enum_match_arm.stage-id.fixed
To only update this specific test, also pass `--test-args wildcard_enum_match_arm.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/wildcard_enum_match_arm.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/wildcard_enum_match_arm.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3f3ead7dae58a5a8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-43d16fd8e2fbc291.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-934c285f6858724f.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-7f4531ca9e916653.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3365d689274e8da9.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/wildcard_enum_match_arm.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"wildcard match will also match any future added variants","code":{"code":"clippy::wildcard_enum_match_arm","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/wildcard_enum_match_arm.rs","byte_start":788,"byte_end":789,"line_start":39,"line_end":39,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        _ => eprintln!(\"Not red\"),","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the lint level is defined here","code":null,"level":"note","spans":[{"file_name":"tests/ui/wildcard_enum_match_arm.rs","byte_start":24,"byte_end":55,"line_start":3,"line_end":3,"column_start":9,"column_end":40,"is_primary":true,"text":[{"text":"#![deny(clippy::wildcard_enum_match_arm)]","highlight_start":9,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/wildcard_enum_match_arm.rs","byte_start":788,"byte_end":789,"line_start":39,"line_end":39,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        _ => eprintln!(\"Not red\"),","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"Color::Green | Color::Blue | Color::Rgb(..) | Color::Cyan","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: wildcard match will also match any future added variants\n  --> tests/ui/wildcard_enum_match_arm.rs:39:9\n   |\nLL |         _ => eprintln!(\"Not red\"),\n   |         ^ help: try this: `Color::Green | Color::Blue | Color::Rgb(..) | Color::Cyan`\n   |\nnote: the lint level is defined here\n  --> tests/ui/wildcard_enum_match_arm.rs:3:9\n   |\nLL | #![deny(clippy::wildcard_enum_match_arm)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"wildcard match will also match any future added variants","code":{"code":"clippy::wildcard_enum_match_arm","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/wildcard_enum_match_arm.rs","byte_start":887,"byte_end":895,"line_start":43,"line_end":43,"column_start":9,"column_end":17,"is_primary":true,"text":[{"text":"        _not_red => eprintln!(\"Not red\"),","highlight_start":9,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/wildcard_enum_match_arm.rs","byte_start":887,"byte_end":895,"line_start":43,"line_end":43,"column_start":9,"column_end":17,"is_primary":true,"text":[{"text":"        _not_red => eprintln!(\"Not red\"),","highlight_start":9,"highlight_end":17}],"label":null,"suggested_replacement":"_not_red @ Color::Green | _not_red @ Color::Blue | _not_red @ Color::Rgb(..) | _not_red @ Color::Cyan","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: wildcard match will also match any future added variants\n  --> tests/ui/wildcard_enum_match_arm.rs:43:9\n   |\nLL |         _not_red => eprintln!(\"Not red\"),\n   |         ^^^^^^^^ help: try this: `_not_red @ Color::Green | _not_red @ Color::Blue | _not_red @ Color::Rgb(..) | _not_red @ Color::Cyan`\n\n"}
{"message":"wildcard match will also match any future added variants","code":{"code":"clippy::wildcard_enum_match_arm","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/wildcard_enum_match_arm.rs","byte_start":1005,"byte_end":1012,"line_start":47,"line_end":47,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        not_red => format!(\"{:?}\", not_red),","highlight_start":9,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/wildcard_enum_match_arm.rs","byte_start":1005,"byte_end":1012,"line_start":47,"line_end":47,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        not_red => format!(\"{:?}\", not_red),","highlight_start":9,"highlight_end":16}],"label":null,"suggested_replacement":"not_red @ Color::Green | not_red @ Color::Blue | not_red @ Color::Rgb(..) | not_red @ Color::Cyan","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: wildcard match will also match any future added variants\n  --> tests/ui/wildcard_enum_match_arm.rs:47:9\n   |\nLL |         not_red => format!(\"{:?}\", not_red),\n   |         ^^^^^^^ help: try this: `not_red @ Color::Green | not_red @ Color::Blue | not_red @ Color::Rgb(..) | not_red @ Color::Cyan`\n\n"}
{"message":"wildcard match will also match any future added variants","code":{"code":"clippy::wildcard_enum_match_arm","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/wildcard_enum_match_arm.rs","byte_start":1499,"byte_end":1500,"line_start":63,"line_end":63,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        _ => \"No red\",","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/wildcard_enum_match_arm.rs","byte_start":1499,"byte_end":1500,"line_start":63,"line_end":63,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        _ => \"No red\",","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"Color::Red | Color::Green | Color::Blue | Color::Rgb(..) | Color::Cyan","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: wildcard match will also match any future added variants\n  --> tests/ui/wildcard_enum_match_arm.rs:63:9\n   |\nLL |         _ => \"No red\",\n   |         ^ help: try this: `Color::Red | Color::Green | Color::Blue | Color::Rgb(..) | Color::Cyan`\n\n"}
{"message":"wildcard matches known variants and will also match future added variants","code":{"code":"clippy::wildcard_enum_match_arm","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/wildcard_enum_match_arm.rs","byte_start":2012,"byte_end":2013,"line_start":80,"line_end":80,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        _ => {},","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/wildcard_enum_match_arm.rs","byte_start":2012,"byte_end":2013,"line_start":80,"line_end":80,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        _ => {},","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"ErrorKind::PermissionDenied | ErrorKind::ConnectionRefused | ErrorKind::ConnectionReset | ErrorKind::ConnectionAborted | ErrorKind::NotConnected | ErrorKind::AddrInUse | ErrorKind::AddrNotAvailable | ErrorKind::BrokenPipe | ErrorKind::AlreadyExists | ErrorKind::WouldBlock | ErrorKind::InvalidInput | ErrorKind::InvalidData | ErrorKind::TimedOut | ErrorKind::WriteZero | ErrorKind::Interrupted | ErrorKind::Other | ErrorKind::UnexpectedEof | ErrorKind::Unsupported | ErrorKind::OutOfMemory | _","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: wildcard matches known variants and will also match future added variants\n  --> tests/ui/wildcard_enum_match_arm.rs:80:9\n   |\nLL |         _ => {},\n   |         ^ help: try this: `ErrorKind::PermissionDenied | ErrorKind::ConnectionRefused | ErrorKind::ConnectionReset | ErrorKind::ConnectionAborted | ErrorKind::NotConnected | ErrorKind::AddrInUse | ErrorKind::AddrNotAvailable | ErrorKind::BrokenPipe | ErrorKind::AlreadyExists | ErrorKind::WouldBlock | ErrorKind::InvalidInput | ErrorKind::InvalidData | ErrorKind::TimedOut | ErrorKind::WriteZero | ErrorKind::Interrupted | ErrorKind::Other | ErrorKind::UnexpectedEof | ErrorKind::Unsupported | ErrorKind::OutOfMemory | _`\n\n"}
{"message":"wildcard matches known variants and will also match future added variants","code":{"code":"clippy::wildcard_enum_match_arm","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/wildcard_enum_match_arm.rs","byte_start":2794,"byte_end":2795,"line_start":102,"line_end":102,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        _ => {},","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/wildcard_enum_match_arm.rs","byte_start":2794,"byte_end":2795,"line_start":102,"line_end":102,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        _ => {},","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"ErrorKind::OutOfMemory | _","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: wildcard matches known variants and will also match future added variants\n  --> tests/ui/wildcard_enum_match_arm.rs:102:9\n   |\nLL |         _ => {},\n   |         ^ help: try this: `ErrorKind::OutOfMemory | _`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.6.0/src/lib.rs:105:22
