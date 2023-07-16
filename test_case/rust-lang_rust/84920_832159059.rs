plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii....
test result: ok. 4 passed; 0 failed; 29 ignored; 0 measured; 0 filtered out; finished in 0.11s

 finished in 0.174 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.33s

 finished in 2.395 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
 finished in 37.201 seconds
Check compiletest suite=pretty mode=pretty (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 69 tests
2021-05-04T18:35:47.929927Z ERROR compiletest::runtest: fatal error, panic: "pretty-printed source does not match expected source\nexpected:\n------------------------------------------\n// pp-exact\n\n#![feature(rustc_attrs)]\n\nmacro_rules! mac { ($ ($ tt : tt) *) => () }\n\nmac! {\n    struct S { field1 : u8, field2 : u16, } impl Clone for S\n    {\n        fn clone() -> S\n        {\n            panic ! () ;\n\n        }\n    }\n}\n\nmac! {\n    a(aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n      aaaaaaaa aaaaaaaa) a\n    [aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n     aaaaaaaa aaaaaaaa] a\n    {\n        aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n        aaaaaaaa aaaaaaaa aaaaaaaa\n    } a\n}\n\nmac!(aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n     aaaaaaaa aaaaaaaa);\nmac![aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n     aaaaaaaa aaaaaaaa];\nmac! {\n    aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n    aaaaaaaa aaaaaaaa\n}\n\n#[rustc_dummy(aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n              aaaaaaaa aaaaaaaa aaaaaaaa)]\n#[rustc_dummy[aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n              aaaaaaaa aaaaaaaa aaaaaaaa]]\n#[rustc_dummy {\n      aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n      aaaaaaaa aaaaaaaa\n  }]\n#[rustc_dummy =\n  \"aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\"]\nfn main() { }\n\n------------------------------------------\nactual:\n------------------------------------------\n// pp-exact\n\n#![feature(rustc_attrs)]\n\nmacro_rules! mac { ($ ($ tt : tt) *) => () }\n\nmac! {\n    struct S { field1 : u8, field2 : u16, } impl Clone for S\n    {\n        fn clone() -> S\n        {\n            panic! () ;\n\n        }\n    }\n}\n\nmac! {\n    a(aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n      aaaaaaaa aaaaaaaa) a\n    [aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n     aaaaaaaa aaaaaaaa] a\n    {\n        aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n        aaaaaaaa aaaaaaaa aaaaaaaa\n    } a\n}\n\nmac!(aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n     aaaaaaaa aaaaaaaa);\nmac![aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n     aaaaaaaa aaaaaaaa];\nmac! {\n    aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n    aaaaaaaa aaaaaaaa\n}\n\n#[rustc_dummy(aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n              aaaaaaaa aaaaaaaa aaaaaaaa)]\n#[rustc_dummy[aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n              aaaaaaaa aaaaaaaa aaaaaaaa]]\n#[rustc_dummy {\n      aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\n      aaaaaaaa aaaaaaaa\n  }]\n#[rustc_dummy =\n  \"aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa\"]\nfn main() { }\n\n------------------------------------------\ndiff:\n------------------------------------------\n9\t    {\n10\t        fn clone() -> S\n11\t        {\n-\t            panic ! () ;\n+\t            panic! () ;\n13\t\n14\t        }\n15\t    }\n\n\n"
F2021-05-04T18:35:47.931862Z ERROR compiletest::runtest: fatal error, panic: "pretty-printed source does not match expected source\nexpected:\n------------------------------------------\n// pp-exact\n\nfn main() { }\n\n#[cfg(FALSE)]\nfn syntax() {\n    let _ = #[attr] box 0;\n    let _ = #[attr] [];\n    let _ = #[attr] [0];\n    let _ = #[attr] [0; 0];\n    let _ = #[attr] [0, 0, 0];\n    let _ = #[attr] foo();\n    let _ = #[attr] x.foo();\n    let _ = #[attr] ();\n    let _ = #[attr] (#[attr] 0,);\n    let _ = #[attr] (#[attr] 0, 0);\n    let _ = #[attr] 0 + #[attr] 0;\n    let _ = #[attr] 0 / #[attr] 0;\n    let _ = #[attr] 0 & #[attr] 0;\n    let _ = #[attr] 0 % #[attr] 0;\n    let _ = #[attr] (0 + 0);\n    let _ = #[attr] !0;\n    let _ = #[attr] -0;\n    let _ = #[attr] false;\n    let _ = #[attr] 0;\n    let _ = #[attr] 'c';\n    let _ = #[attr] x as Y;\n    let _ = #[attr] (x as Y);\n    let _ =\n        #[attr] while true {\n                    #![attr]\n                };\n    let _ =\n        #[attr] while let Some(false) = true {\n                    #![attr]\n                };\n    let _ =\n        #[attr] for x in y {\n                    #![attr]\n                };\n    let _ =\n        #[attr] loop  {\n                    #![attr]\n                };\n    let _ =\n        #[attr] match true\n                    {\n                     #[attr]\n                     _ => false,\n                };\n    let _ = #[attr] || #[attr] foo;\n    let _ = #[attr] move || #[attr] foo;\n    let _ =\n        #[attr] ||\n                    #[attr] {\n                                #![attr]\n                                foo\n                            };\n    let _ =\n        #[attr] move ||\n                    #[attr] {\n                                #![attr]\n                                foo\n                            };\n    let _ =\n        #[attr] ||\n                    {\n                        #![attr]\n                        foo\n                    };\n    let _ =\n        #[attr] move ||\n                    {\n                        #![attr]\n                        foo\n                    };\n    let _ =\n        #[attr] {\n                    #![attr]\n                };\n    let _ =\n        #[attr] {\n                    #![attr]\n                    let _ = ();\n                };\n    let _ =\n        #[attr] {\n                    #![attr]\n                    let _ = ();\n                    foo\n                };\n    let _ = #[attr] x = y;\n    let _ = #[attr] (x = y);\n    let _ = #[attr] x += y;\n    let _ = #[attr] (x += y);\n    let _ = #[attr] foo.bar;\n    let _ = (#[attr] foo).bar;\n    let _ = #[attr] foo.0;\n    let _ = (#[attr] foo).0;\n    let _ = #[attr] foo[bar];\n    let _ = (#[attr] foo)[bar];\n    let _ = #[attr] 0..#[attr] 0;\n    let _ = #[attr] 0..;\n    let _ = #[attr] (0..0);\n    let _ = #[attr] (0..);\n    let _ = #[attr] (..0);\n    let _ = #[attr] (..);\n    let _ = #[attr] foo::bar::baz;\n    let _ = #[attr] &0;\n    let _ = #[attr] &mut 0;\n    let _ = #[attr] &#[attr] 0;\n    let _ = #[attr] &mut #[attr] 0;\n    let _ = #[attr] break ;\n    let _ = #[attr] continue ;\n    let _ = #[attr] return;\n    let _ = #[attr] foo!();\n    let _ = #[attr] foo!(# ! [attr]);\n    let _ = #[attr] foo![];\n    let _ = #[attr] foo![# ! [attr]];\n    let _ = #[attr] foo! { };\n    let _ = #[attr] foo! { # ! [attr] };\n    let _ = #[attr] Foo{bar: baz,};\n    let _ = #[attr] Foo{..foo};\n    let _ = #[attr] Foo{bar: baz, ..foo};\n    let _ = #[attr] (0);\n\n    {\n        #[attr]\n        let _ = 0;\n\n        #[attr]\n        0;\n\n        #[attr]\n        foo!();\n\n        #[attr]\n        foo! { }\n\n        #[attr]\n        foo![];\n    }\n\n    {\n        #[attr]\n        let _ = 0;\n    }\n    {\n\n        #[attr]\n        0\n    }\n    {\n\n        #[attr]\n        {\n            #![attr]\n        }\n    }\n    {\n\n        #[attr]\n        foo!()\n    }\n    {\n\n        #[attr]\n        foo![]\n    }\n    {\n\n        #[attr]\n        foo! { }\n    }\n}\n\n------------------------------------------\nactual:\n------------------------------------------\n// pp-exact\n\nfn main() { }\n\n#[cfg(FALSE)]\nfn syntax() {\n    let _ = #[attr] box 0;\n    let _ = #[attr] [];\n    let _ = #[attr] [0];\n    let _ = #[attr] [0; 0];\n    let _ = #[attr] [0, 0, 0];\n    let _ = #[attr] foo();\n    let _ = #[attr] x.foo();\n    let _ = #[attr] ();\n    let _ = #[attr] (#[attr] 0,);\n    let _ = #[attr] (#[attr] 0, 0);\n    let _ = #[attr] 0 + #[attr] 0;\n    let _ = #[attr] 0 / #[attr] 0;\n    let _ = #[attr] 0 & #[attr] 0;\n    let _ = #[attr] 0 % #[attr] 0;\n    let _ = #[attr] (0 + 0);\n    let _ = #[attr] !0;\n    let _ = #[attr] -0;\n    let _ = #[attr] false;\n    let _ = #[attr] 0;\n    let _ = #[attr] 'c';\n    let _ = #[attr] x as Y;\n    let _ = #[attr] (x as Y);\n    let _ =\n        #[attr] while true {\n                    #![attr]\n                };\n    let _ =\n        #[attr] while let Some(false) = true {\n                    #![attr]\n                };\n    let _ =\n        #[attr] for x in y {\n                    #![attr]\n                };\n    let _ =\n        #[attr] loop  {\n                    #![attr]\n                };\n    let _ =\n        #[attr] match true\n                    {\n                     #[attr]\n                     _ => false,\n                };\n    let _ = #[attr] || #[attr] foo;\n    let _ = #[attr] move || #[attr] foo;\n    let _ =\n        #[attr] ||\n                    #[attr] {\n                                #![attr]\n                                foo\n                            };\n    let _ =\n        #[attr] move ||\n                    #[attr] {\n                                #![attr]\n                                foo\n                            };\n    let _ =\n        #[attr] ||\n                    {\n                        #![attr]\n                        foo\n                    };\n    let _ =\n        #[attr] move ||\n                    {\n                        #![attr]\n                        foo\n                    };\n    let _ =\n        #[attr] {\n                    #![attr]\n                };\n    let _ =\n        #[attr] {\n                    #![attr]\n                    let _ = ();\n                };\n    let _ =\n        #[attr] {\n                    #![attr]\n                    let _ = ();\n                    foo\n                };\n    let _ = #[attr] x = y;\n    let _ = #[attr] (x = y);\n    let _ = #[attr] x += y;\n    let _ = #[attr] (x += y);\n    let _ = #[attr] foo.bar;\n    let _ = (#[attr] foo).bar;\n    let _ = #[attr] foo.0;\n    let _ = (#[attr] foo).0;\n    let _ = #[attr] foo[bar];\n    let _ = (#[attr] foo)[bar];\n    let _ = #[attr] 0..#[attr] 0;\n    let _ = #[attr] 0..;\n    let _ = #[attr] (0..0);\n    let _ = #[attr] (0..);\n    let _ = #[attr] (..0);\n    let _ = #[attr] (..);\n    let _ = #[attr] foo::bar::baz;\n    let _ = #[attr] &0;\n    let _ = #[attr] &mut 0;\n    let _ = #[attr] &#[attr] 0;\n    let _ = #[attr] &mut #[attr] 0;\n    let _ = #[attr] break ;\n    let _ = #[attr] continue ;\n    let _ = #[attr] return;\n    let _ = #[attr] foo!();\n    let _ = #[attr] foo!(#! [attr]);\n    let _ = #[attr] foo![];\n    let _ = #[attr] foo![#! [attr]];\n    let _ = #[attr] foo! { };\n    let _ = #[attr] foo! { #! [attr] };\n    let _ = #[attr] Foo{bar: baz,};\n    let _ = #[attr] Foo{..foo};\n    let _ = #[attr] Foo{bar: baz, ..foo};\n    let _ = #[attr] (0);\n\n    {\n        #[attr]\n        let _ = 0;\n\n        #[attr]\n        0;\n\n        #[attr]\n        foo!();\n\n        #[attr]\n        foo! { }\n\n        #[attr]\n        foo![];\n    }\n\n    {\n        #[attr]\n        let _ = 0;\n    }\n    {\n\n        #[attr]\n        0\n    }\n    {\n\n        #[attr]\n        {\n            #![attr]\n        }\n    }\n    {\n\n        #[attr]\n        foo!()\n    }\n    {\n\n        #[attr]\n        foo![]\n    }\n    {\n\n        #[attr]\n        foo! { }\n    }\n}\n\n------------------------------------------\ndiff:\n------------------------------------------\n114\t    let _ = #[attr] continue ;\n115\t    let _ = #[attr] return;\n116\t    let _ = #[attr] foo!();\n-\t    let _ = #[attr] foo!(# ! [attr]);\n+\t    let _ = #[attr] foo!(#! [attr]);\n118\t    let _ = #[attr] foo![];\n-\t    let _ = #[attr] foo![# ! [attr]];\n+\t    let _ = #[attr] foo![#! [attr]];\n120\t    let _ = #[attr] foo! { };\n-\t    let _ = #[attr] foo! { # ! [attr] };\n+\t    let _ = #[attr] foo! { #! [attr] };\n122\t    let _ = #[attr] Foo{bar: baz,};\n123\t    let _ = #[attr] Foo{..foo};\n124\t    let _ = #[attr] Foo{bar: baz, ..foo};\n\n\n"
Some tests failed in compiletest suite=pretty mode=pretty host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
F...................................................................

---- [pretty] pretty/delimited-token-groups.rs stdout ----

error: pretty-printed source does not match expected source
error: pretty-printed source does not match expected source
expected:
------------------------------------------
// pp-exact
#![feature(rustc_attrs)]


macro_rules! mac { ($ ($ tt : tt) *) => () }
mac! {
mac! {
    struct S { field1 : u8, field2 : u16, } impl Clone for S
        fn clone() -> S
        {
        {
            panic ! () ;
        }
    }
}


mac! {
    a(aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
      aaaaaaaa aaaaaaaa) a
    [aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
     aaaaaaaa aaaaaaaa] a
    {
        aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
        aaaaaaaa aaaaaaaa aaaaaaaa
    } a


mac!(aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
     aaaaaaaa aaaaaaaa);
mac![aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
     aaaaaaaa aaaaaaaa];
mac! {
    aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
    aaaaaaaa aaaaaaaa


#[rustc_dummy(aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
              aaaaaaaa aaaaaaaa aaaaaaaa)]
#[rustc_dummy[aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
              aaaaaaaa aaaaaaaa aaaaaaaa]]
#[rustc_dummy {
      aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
      aaaaaaaa aaaaaaaa
#[rustc_dummy =
#[rustc_dummy =
  "aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa"]
fn main() { }
------------------------------------------
actual:
------------------------------------------
------------------------------------------
// pp-exact
#![feature(rustc_attrs)]


macro_rules! mac { ($ ($ tt : tt) *) => () }
mac! {
mac! {
    struct S { field1 : u8, field2 : u16, } impl Clone for S
        fn clone() -> S
        {
        {
            panic! () ;
        }
    }
}


mac! {
    a(aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
      aaaaaaaa aaaaaaaa) a
    [aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
     aaaaaaaa aaaaaaaa] a
    {
        aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
        aaaaaaaa aaaaaaaa aaaaaaaa
    } a


mac!(aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
     aaaaaaaa aaaaaaaa);
mac![aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
     aaaaaaaa aaaaaaaa];
mac! {
    aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
    aaaaaaaa aaaaaaaa


#[rustc_dummy(aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
              aaaaaaaa aaaaaaaa aaaaaaaa)]
#[rustc_dummy[aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
              aaaaaaaa aaaaaaaa aaaaaaaa]]
#[rustc_dummy {
      aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa
      aaaaaaaa aaaaaaaa
#[rustc_dummy =
#[rustc_dummy =
  "aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa aaaaaaaa"]
fn main() { }
------------------------------------------
diff:
------------------------------------------
9     {
9     {
10         fn clone() -> S
11         {
-             panic ! () ;
+             panic! () ;
14         }
15     }




thread '[pretty] pretty/delimited-token-groups.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2226:9

---- [pretty] pretty/ast-stmt-expr-attr.rs stdout ----

error: pretty-printed source does not match expected source
error: pretty-printed source does not match expected source
expected:
------------------------------------------
// pp-exact
fn main() { }

#[cfg(FALSE)]
#[cfg(FALSE)]
fn syntax() {
    let _ = #[attr] box 0;
    let _ = #[attr] [];
    let _ = #[attr] [0];
    let _ = #[attr] [0; 0];
    let _ = #[attr] [0, 0, 0];
    let _ = #[attr] foo();
    let _ = #[attr] x.foo();
    let _ = #[attr] ();
    let _ = #[attr] (#[attr] 0,);
    let _ = #[attr] (#[attr] 0, 0);
    let _ = #[attr] 0 + #[attr] 0;
    let _ = #[attr] 0 / #[attr] 0;
    let _ = #[attr] 0 & #[attr] 0;
    let _ = #[attr] 0 % #[attr] 0;
    let _ = #[attr] (0 + 0);
    let _ = #[attr] !0;
    let _ = #[attr] -0;
    let _ = #[attr] false;
    let _ = #[attr] 0;
    let _ = #[attr] 'c';
    let _ = #[attr] x as Y;
    let _ = #[attr] (x as Y);
    let _ =
        #[attr] while true {
                    #![attr]
    let _ =
    let _ =
        #[attr] while let Some(false) = true {
                    #![attr]
    let _ =
    let _ =
        #[attr] for x in y {
                    #![attr]
    let _ =
    let _ =
        #[attr] loop  {
                    #![attr]
    let _ =
    let _ =
        #[attr] match true
                    {
                     #[attr]
                };
                };
    let _ = #[attr] || #[attr] foo;
    let _ = #[attr] move || #[attr] foo;
    let _ =
        #[attr] ||
                    #[attr] {
                                #![attr]
                                foo
    let _ =
    let _ =
        #[attr] move ||
                    #[attr] {
                                #![attr]
                                foo
    let _ =
    let _ =
        #[attr] ||
                    {
                        #![attr]
                        foo
    let _ =
    let _ =
        #[attr] move ||
                    {
                        #![attr]
                        foo
    let _ =
    let _ =
        #[attr] {
                    #![attr]
    let _ =
    let _ =
        #[attr] {
                    #![attr]
                    let _ = ();
    let _ =
    let _ =
        #[attr] {
                    #![attr]
                    let _ = ();
                    foo
                };
    let _ = #[attr] x = y;
    let _ = #[attr] (x = y);
    let _ = #[attr] x += y;
    let _ = #[attr] (x += y);
    let _ = #[attr] foo.bar;
    let _ = (#[attr] foo).bar;
    let _ = #[attr] foo.0;
    let _ = (#[attr] foo).0;
    let _ = #[attr] foo[bar];
    let _ = (#[attr] foo)[bar];
    let _ = #[attr] 0..#[attr] 0;
    let _ = #[attr] 0..;
    let _ = #[attr] (0..0);
    let _ = #[attr] (0..);
    let _ = #[attr] (..0);
    let _ = #[attr] (..);
    let _ = #[attr] foo::bar::baz;
    let _ = #[attr] &0;
    let _ = #[attr] &mut 0;
    let _ = #[attr] &#[attr] 0;
    let _ = #[attr] &mut #[attr] 0;
    let _ = #[attr] break ;
    let _ = #[attr] continue ;
    let _ = #[attr] return;
    let _ = #[attr] foo!();
    let _ = #[attr] foo!(# ! [attr]);
    let _ = #[attr] foo![];
    let _ = #[attr] foo![# ! [attr]];
    let _ = #[attr] foo! { };
    let _ = #[attr] foo! { # ! [attr] };
    let _ = #[attr] Foo{bar: baz,};
    let _ = #[attr] Foo{..foo};
    let _ = #[attr] Foo{bar: baz, ..foo};
    let _ = #[attr] (0);
    {
    {
        #[attr]
        let _ = 0;

        #[attr]
        0;

        #[attr]
        foo!();

        #[attr]
        foo! { }

        #[attr]
        foo![];

    {
    {
        #[attr]
        let _ = 0;
    {


        #[attr]
    }
    {


        #[attr]
        {
            #![attr]
    }
    {


        #[attr]
        foo!()
    {


        #[attr]
        foo![]
    {


        #[attr]
        foo! { }
}

------------------------------------------
actual:
actual:
------------------------------------------
// pp-exact
fn main() { }

#[cfg(FALSE)]
#[cfg(FALSE)]
fn syntax() {
    let _ = #[attr] box 0;
    let _ = #[attr] [];
    let _ = #[attr] [0];
    let _ = #[attr] [0; 0];
    let _ = #[attr] [0, 0, 0];
    let _ = #[attr] foo();
    let _ = #[attr] x.foo();
    let _ = #[attr] ();
    let _ = #[attr] (#[attr] 0,);
    let _ = #[attr] (#[attr] 0, 0);
    let _ = #[attr] 0 + #[attr] 0;
    let _ = #[attr] 0 / #[attr] 0;
    let _ = #[attr] 0 & #[attr] 0;
    let _ = #[attr] 0 % #[attr] 0;
    let _ = #[attr] (0 + 0);
    let _ = #[attr] !0;
    let _ = #[attr] -0;
    let _ = #[attr] false;
    let _ = #[attr] 0;
    let _ = #[attr] 'c';
    let _ = #[attr] x as Y;
    let _ = #[attr] (x as Y);
    let _ =
        #[attr] while true {
                    #![attr]
    let _ =
    let _ =
        #[attr] while let Some(false) = true {
                    #![attr]
    let _ =
    let _ =
        #[attr] for x in y {
                    #![attr]
    let _ =
    let _ =
        #[attr] loop  {
                    #![attr]
    let _ =
    let _ =
        #[attr] match true
                    {
                     #[attr]
                };
                };
    let _ = #[attr] || #[attr] foo;
    let _ = #[attr] move || #[attr] foo;
    let _ =
        #[attr] ||
                    #[attr] {
                                #![attr]
                                foo
    let _ =
    let _ =
        #[attr] move ||
                    #[attr] {
                                #![attr]
                                foo
    let _ =
    let _ =
        #[attr] ||
                    {
                        #![attr]
                        foo
    let _ =
    let _ =
        #[attr] move ||
                    {
                        #![attr]
                        foo
    let _ =
    let _ =
        #[attr] {
                    #![attr]
    let _ =
    let _ =
        #[attr] {
                    #![attr]
                    let _ = ();
    let _ =
    let _ =
        #[attr] {
                    #![attr]
                    let _ = ();
                    foo
                };
    let _ = #[attr] x = y;
    let _ = #[attr] (x = y);
    let _ = #[attr] x += y;
    let _ = #[attr] (x += y);
    let _ = #[attr] foo.bar;
    let _ = (#[attr] foo).bar;
    let _ = #[attr] foo.0;
    let _ = (#[attr] foo).0;
    let _ = #[attr] foo[bar];
    let _ = (#[attr] foo)[bar];
    let _ = #[attr] 0..#[attr] 0;
    let _ = #[attr] 0..;
    let _ = #[attr] (0..0);
    let _ = #[attr] (0..);
    let _ = #[attr] (..0);
    let _ = #[attr] (..);
    let _ = #[attr] foo::bar::baz;
    let _ = #[attr] &0;
    let _ = #[attr] &mut 0;
    let _ = #[attr] &#[attr] 0;
    let _ = #[attr] &mut #[attr] 0;
    let _ = #[attr] break ;
    let _ = #[attr] continue ;
    let _ = #[attr] return;
    let _ = #[attr] foo!();
    let _ = #[attr] foo!(#! [attr]);
    let _ = #[attr] foo![];
    let _ = #[attr] foo![#! [attr]];
    let _ = #[attr] foo! { };
    let _ = #[attr] foo! { #! [attr] };
    let _ = #[attr] Foo{bar: baz,};
    let _ = #[attr] Foo{..foo};
    let _ = #[attr] Foo{bar: baz, ..foo};
    let _ = #[attr] (0);
    {
    {
        #[attr]
        let _ = 0;

        #[attr]
        0;

        #[attr]
        foo!();

        #[attr]
        foo! { }

        #[attr]
        foo![];

    {
    {
        #[attr]
        let _ = 0;
    {


        #[attr]
    }
    {


        #[attr]
        {
            #![attr]
    }
    {


        #[attr]
        foo!()
    {


        #[attr]
        foo![]
    {


        #[attr]
        foo! { }
}

------------------------------------------
diff:
diff:
------------------------------------------
114     let _ = #[attr] continue ;
115     let _ = #[attr] return;
116     let _ = #[attr] foo!();
-     let _ = #[attr] foo!(# ! [attr]);
