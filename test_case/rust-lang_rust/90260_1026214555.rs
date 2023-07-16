
ninja: Entering directory `_build/'
[2/3] Compiling Rust source ../subprojects/demolib/demolib_test.rs
FAILED: subprojects/demolib/demolib_test 
rustc -C linker=cc --color=always --crate-type bin -W warnings -g --crate-name demolib_test --emit dep-info=subprojects/demolib/demolib_test.d --emit link -o subprojects/demolib/demolib_test --extern demolib=subprojects/demolib/libdemolib.so.0 -L subprojects/demolib -C prefer-dynamic -C 'link-arg=-Wl,-rpath,$ORIGIN/:/usr/lib' -C link-arg=-Wl,-rpath-link,/home/florian/workspace/rtest/_build/subprojects/demolib:/usr/lib ../subprojects/demolib/demolib_test.rs
error[E0463]: can't find crate for `demolib`
 --> ../subprojects/demolib/demolib_test.rs:1:1
  |
1 | extern crate demolib;
  | ^^^^^^^^^^^^^^^^^^^^^ can't find crate
  |
  = note: extern location for demolib is of an unknown type: subprojects/demolib/libdemolib.so.0
  = help: file name should be lib*.rlib or lib*..so

error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
[3/3] Compiling Rust source ../rdemo.rs
FAILED: rdemo 
rustc -C linker=cc --color=always --crate-type bin -W warnings -g --crate-name rdemo --emit dep-info=rdemo.d --emit link -o rdemo --extern demolib=subprojects/demolib/libdemolib.so.0 -L subprojects/demolib -C prefer-dynamic -C 'link-arg=-Wl,-rpath,$ORIGIN/subprojects/demolib:/usr/lib' -C link-arg=-Wl,-rpath-link,/home/florian/workspace/rtest/_build/subprojects/demolib:/usr/lib ../rdemo.rs
error[E0463]: can't find crate for `demolib`
 --> ../rdemo.rs:5:26
  |
5 |     println!("Demo: {}", demolib::dem_func());
  |                          ^^^^^^^ can't find crate
  |
  = note: extern location for demolib is of an unknown type: subprojects/demolib/libdemolib.so.0
  = help: file name should be lib*.rlib or lib*..so

error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
ninja: build stopped: subcommand failed.
