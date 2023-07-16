plain
.................................................................................................... 9200/11445
.................................................................................................... 9300/11445
.................................................................................................... 9400/11445
....i......i........................................................................................ 9500/11445
..........................................iiiiiii..iiiiii.i......................................... 9600/11445
.................................................................................................... 9800/11445
.................................................................................................... 9900/11445
.................................................................................................... 10000/11445
.................................................................................................... 10100/11445
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.070 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.34s

 finished in 2.406 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
   Compiling difference v2.0.0
   Compiling once_cell v1.4.1
   Compiling expect-test v1.0.1
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0433]: failed to resolve: use of undeclared type `Ordering`
  --> src/librustdoc/html/render/tests.rs:23:44
   |
23 |     assert_eq!(compare_names("u8", "u16"), Ordering::Less);
   |                                            ^^^^^^^^ use of undeclared type `Ordering`

error[E0433]: failed to resolve: use of undeclared type `Ordering`
  --> src/librustdoc/html/render/tests.rs:24:45
   |
24 |     assert_eq!(compare_names("u32", "u16"), Ordering::Greater);
   |                                             ^^^^^^^^ use of undeclared type `Ordering`

error[E0433]: failed to resolve: use of undeclared type `Ordering`
  --> src/librustdoc/html/render/tests.rs:25:58
   |
25 |     assert_eq!(compare_names("u8_to_f64", "u16_to_f64"), Ordering::Less);
   |                                                          ^^^^^^^^ use of undeclared type `Ordering`

error[E0433]: failed to resolve: use of undeclared type `Ordering`
  --> src/librustdoc/html/render/tests.rs:26:59
   |
26 |     assert_eq!(compare_names("u32_to_f64", "u16_to_f64"), Ordering::Greater);
   |                                                           ^^^^^^^^ use of undeclared type `Ordering`

error[E0433]: failed to resolve: use of undeclared type `Ordering`
  --> src/librustdoc/html/render/tests.rs:27:59
   |
27 |     assert_eq!(compare_names("u16_to_f64", "u16_to_f64"), Ordering::Equal);
   |                                                           ^^^^^^^^ use of undeclared type `Ordering`

error[E0433]: failed to resolve: use of undeclared type `Ordering`
  --> src/librustdoc/html/render/tests.rs:28:59
   |
28 |     assert_eq!(compare_names("u16_to_f32", "u16_to_f64"), Ordering::Less);
   |                                                           ^^^^^^^^ use of undeclared type `Ordering`
error[E0425]: cannot find function `compare_names` in this scope
  --> src/librustdoc/html/render/tests.rs:21:20
   |
   |
21 |         assert_eq!(compare_names(a, b), a.cmp(b), "{:?} - {:?}", a, b);
   |
help: consider importing this function
   |
1  | use crate::html::render::print_item::compare_names;
1  | use crate::html::render::print_item::compare_names;
   |

error[E0425]: cannot find function `compare_names` in this scope
  --> src/librustdoc/html/render/tests.rs:23:16
   |
23 |     assert_eq!(compare_names("u8", "u16"), Ordering::Less);
   |
help: consider importing this function
   |
1  | use crate::html::render::print_item::compare_names;
1  | use crate::html::render::print_item::compare_names;
   |

error[E0425]: cannot find function `compare_names` in this scope
  --> src/librustdoc/html/render/tests.rs:24:16
   |
24 |     assert_eq!(compare_names("u32", "u16"), Ordering::Greater);
   |
help: consider importing this function
   |
1  | use crate::html::render::print_item::compare_names;
1  | use crate::html::render::print_item::compare_names;
   |

error[E0425]: cannot find function `compare_names` in this scope
  --> src/librustdoc/html/render/tests.rs:25:16
   |
25 |     assert_eq!(compare_names("u8_to_f64", "u16_to_f64"), Ordering::Less);
   |
help: consider importing this function
   |
1  | use crate::html::render::print_item::compare_names;
1  | use crate::html::render::print_item::compare_names;
   |

error[E0425]: cannot find function `compare_names` in this scope
  --> src/librustdoc/html/render/tests.rs:26:16
   |
26 |     assert_eq!(compare_names("u32_to_f64", "u16_to_f64"), Ordering::Greater);
   |
help: consider importing this function
   |
1  | use crate::html::render::print_item::compare_names;
1  | use crate::html::render::print_item::compare_names;
   |

error[E0425]: cannot find function `compare_names` in this scope
  --> src/librustdoc/html/render/tests.rs:27:16
   |
27 |     assert_eq!(compare_names("u16_to_f64", "u16_to_f64"), Ordering::Equal);
   |
help: consider importing this function
   |
1  | use crate::html::render::print_item::compare_names;
1  | use crate::html::render::print_item::compare_names;
   |

error[E0425]: cannot find function `compare_names` in this scope
  --> src/librustdoc/html/render/tests.rs:28:16
   |
28 |     assert_eq!(compare_names("u16_to_f32", "u16_to_f64"), Ordering::Less);
   |
help: consider importing this function
   |
1  | use crate::html::render::print_item::compare_names;
1  | use crate::html::render::print_item::compare_names;
   |

error[E0425]: cannot find function `compare_names` in this scope
  --> src/librustdoc/html/render/tests.rs:38:28
   |
38 |     sorted.sort_by(|&l, r| compare_names(l, r));
   |
help: consider importing this function
   |
1  | use crate::html::render::print_item::compare_names;
---
  |
1 | use super::*;
  |     ^^^^^
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: aborting due to 15 previous errors

Some errors have detailed explanations: E0425, E0433.
For more information about an error, try `rustc --explain E0425`.
For more information about an error, try `rustc --explain E0425`.
error: could not compile `rustdoc`

To learn more, run the command again with --verbose.


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:29:01
