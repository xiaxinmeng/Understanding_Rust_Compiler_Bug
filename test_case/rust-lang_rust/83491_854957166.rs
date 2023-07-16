
$ rustc --pretty=normal src/main.rs -Zunstable-options
fn main() { println!("{}", std :: env :: var("LD_LIBRARY_PATH").unwrap()); }
$ rustc --pretty=identified src/main.rs -Zunstable-options
fn main() { println!("{}", std :: env :: var("LD_LIBRARY_PATH").unwrap()); }
/* block 4294967040 */ /* 4294967040 */
$ rustc --pretty=expanded src/main.rs -Zunstable-options
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::rust_2015::*;
#[macro_use]
extern crate std;
fn main() {
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(&["", "\n"],
                                                         &match (&std::env::var("LD_LIBRARY_PATH").unwrap(),)
                                                              {
                                                              (arg0,) =>
                                                              [::core::fmt::ArgumentV1::new(arg0,
                                                                                            ::core::fmt::Display::fmt)],
                                                          }));
    };
}
$ rustc --pretty=expanded,identified src/main.rs -Zunstable-options
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::rust_2015::*; /* 4 */
#[macro_use]
extern crate std; /* 10 */
fn main() {
    ({
         ((::std::io::_print /* 17
              */)(((::core::fmt::Arguments::new_v1 /* 24
                       */)((&([("" /* 25 */), ("\n" /* 26 */)] /* 27 */) /* 28
                               */),
                           (&(match (((&(((std::env::var /* 33
                                              */)(("LD_LIBRARY_PATH" /* 34
                                                      */)) /* 35 */).unwrap()
                                            /* 36 */) /* 37 */),) /* 38 */) {
                                  (arg0 /* pat 41 */,) /* pat 40 */ =>
                                  ([((::core::fmt::ArgumentV1::new /* 46
                                         */)((arg0 /* 48 */),
                                             (::core::fmt::Display::fmt /* 53
                                                 */)) /* 54 */)] /* 55 */),
                              } /* 56 */) /* 57 */)) /* 58 */)) /* 18 */);
     } /* block 13 */ /* 19 */);
} /* block 12 */ /* 11 */
(-bash@build-server) ~/test-rustdoc/hello [15:31:06]
$ rustc --pretty=expanded,hygiene src/main.rs -Zunstable-options
#![feature /* 491#0 */(prelude_import)]
#![no_std /* 747#0 */]
#[prelude_import /* 837#1 */]
use ::std /* 1108#1 */::prelude /* 836#1 */::rust_2015 /* 925#1 */::*;
#[macro_use /* 658#1 */]
extern crate std /* 1108#2 */;
fn main /* 661#0 */() {
    {
        ::std /* 2#3 */::io /* 1320#3 */::_print /* 1467#3
            */(::core /* 2#4 */::fmt /* 507#0 */::Arguments /* 67#0 */::new_v1
                   /* 1480#0
                   */(&["", "\n"],
                      &match (&std /* 1108#0 */::env /* 455#0 */::var /*
                                   1247#0 */("LD_LIBRARY_PATH").unwrap /*
                                   1234#0 */(),) {
                           (arg0 /* 1478#4 */,) =>
                           [::core /* 2#4 */::fmt /* 507#0 */::ArgumentV1 /*
                                66#0 */::new /* 727#0
                                */(arg0 /* 1478#4 */,
                                   ::core /* 2#4 */::fmt /* 507#0 */::Display
                                       /* 1479#0 */::fmt /* 507#0 */)],
                       }));
    };
}

/*
Expansions:
0: parent: ExpnId(0), call_site_ctxt: #0, def_site_ctxt: #0, kind: Root
1: parent: ExpnId(0), call_site_ctxt: #0, def_site_ctxt: #0, kind: AstPass(StdImports)
2: parent: ExpnId(0), call_site_ctxt: #0, def_site_ctxt: #0, kind: Macro { kind: Bang, name: "println", proc_macro: false }
3: parent: ExpnId(2), call_site_ctxt: #3, def_site_ctxt: #0, kind: Macro { kind: Bang, name: "$crate::format_args_nl", proc_macro: false }

SyntaxContexts:
#0: parent: #0, outer_mark: (ExpnId(0), Opaque)
#1: parent: #0, outer_mark: (ExpnId(1), Opaque)
#2: parent: #0, outer_mark: (ExpnId(1), Transparent)
#3: parent: #0, outer_mark: (ExpnId(2), SemiTransparent)
#4: parent: #0, outer_mark: (ExpnId(3), Opaque)
*/
