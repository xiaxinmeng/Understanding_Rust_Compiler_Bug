
------------------------------------------
stderr:
------------------------------------------
error[E0204]: the trait `Copy` may not be implemented for type `Bar`
  --> C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\src/test\ui\span\issue-19950.rs:23:1
   |
20 |     item: NoCopy,
   |     ------------ this field doesn't implement `Copy`
...
23 | impl Copy for Bar {}
   | ^^^^^^^^^^^^^^^^^^^^ field `item` doesn't implement `Copy`

error[E0205]: the trait `Copy` may not be implemented for type `Foo`
  --> C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\src/test\ui\span\issue-19950.rs:17:6
   |
14 |     MyVariant(NoCopy)
   |     ----------------- this variant doesn't implement `Copy`
...
17 | impl Copy for Foo {}
   |      ^^^^ variant `Foo::MyVariant` doesn't implement `Copy`

error: aborting due to 2 previous errors


------------------------------------------

thread '[ui] ui\span\issue-19950.rs' panicked at 'explicit panic', C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\src\tools\compiletest\src\runtest.rs:2407
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    [ui] ui\span\issue-19950.rs
