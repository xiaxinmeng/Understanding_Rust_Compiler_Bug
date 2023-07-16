plain
........................................................................................ 12672/13761
........................................................................................ 12760/13761
........................................................................................ 12848/13761
.................................i...................................................... 12936/13761
............................................................................F.F......... 13024/13761
........................................................................................ 13200/13761
........................................................................................ 13288/13761
........................................................................................ 13376/13761
........................................................................................ 13464/13761
---
failures:

---- [ui] src/test/ui/type/issue-94187-verbose-type-name.rs#verbose stdout ----

error in revision `verbose`: test run failed!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/issue-94187-verbose-type-name.verbose/a"
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'assertion failed: `(left == right)`
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `"dyn core::ops::function::Fn(u32) -> u32"`,
 right: `"dyn core::ops::function::Fn<(u32,)>+Output = u32"`', /checkout/src/test/ui/type/issue-94187-verbose-type-name.rs:15:5
------------------------------------------


---- [ui] src/test/ui/type/issue-94187-verbose-type-name.rs#normal stdout ----
---- [ui] src/test/ui/type/issue-94187-verbose-type-name.rs#normal stdout ----

error in revision `normal`: test run failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/issue-94187-verbose-type-name.normal/a"
stdout: none
--- stderr -------------------------------
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `"dyn core::ops::function::Fn(u32) -> u32"`,
 right: `"dyn core::ops::function::Fn<(u32,)>+Output = u32"`', /checkout/src/test/ui/type/issue-94187-verbose-type-name.rs:15:5
------------------------------------------



