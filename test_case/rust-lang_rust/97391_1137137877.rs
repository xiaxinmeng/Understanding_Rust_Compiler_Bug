plain
......................................................iii............................... 13200/13273
.........................................................................
failures:

---- [ui] src/test/ui/conditional-compilation/cfg_accessible-not_sure.rs#edition2015 stdout ----

41    |                  ^^^^^^^
42 
43 error: not sure whether the path is accessible or not
43 error: not sure whether the path is accessible or not
-   --> $DIR/cfg_accessible-not_sure.rs:51:18
+   --> $DIR/cfg_accessible-not_sure.rs:52:18
45    |
46 LL | #[cfg_accessible(Trait::foo)]

48 
49 error: not sure whether the path is accessible or not
-   --> $DIR/cfg_accessible-not_sure.rs:58:18
-   --> $DIR/cfg_accessible-not_sure.rs:58:18
+   --> $DIR/cfg_accessible-not_sure.rs:59:18
51    |
52 LL | #[cfg_accessible(TypeAlias::A)]

54 
55 error: not sure whether the path is accessible or not
-   --> $DIR/cfg_accessible-not_sure.rs:67:18
-   --> $DIR/cfg_accessible-not_sure.rs:67:18
+   --> $DIR/cfg_accessible-not_sure.rs:68:18
57    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
58 LL | #[cfg_accessible(ForeignType::A)]

60 
61 error: not sure whether the path is accessible or not
-   --> $DIR/cfg_accessible-not_sure.rs:76:18
-   --> $DIR/cfg_accessible-not_sure.rs:76:18
+   --> $DIR/cfg_accessible-not_sure.rs:77:18
63    |
64 LL | #[cfg_accessible(AssocType::AssocType::A)]

66 
67 error: not sure whether the path is accessible or not
-   --> $DIR/cfg_accessible-not_sure.rs:81:18
-   --> $DIR/cfg_accessible-not_sure.rs:81:18
+   --> $DIR/cfg_accessible-not_sure.rs:82:18
69    |
70 LL | #[cfg_accessible(u8::A)]


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg_accessible-not_sure.edition2015/cfg_accessible-not_sure.edition2015.stderr
To only update this specific test, also pass `--test-args conditional-compilation/cfg_accessible-not_sure.rs`

error in revision `edition2015`: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/conditional-compilation/cfg_accessible-not_sure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "edition2015" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg_accessible-not_sure.edition2015" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2015" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg_accessible-not_sure.edition2015/auxiliary"
stdout: none
--- stderr -------------------------------
error: not sure whether the path is accessible or not
  --> /checkout/src/test/ui/conditional-compilation/cfg_accessible-not_sure.rs:15:18
   |
LL | #[cfg_accessible(Struct::a)] //~ ERROR not sure

error: not sure whether the path is accessible or not
  --> /checkout/src/test/ui/conditional-compilation/cfg_accessible-not_sure.rs:17:18
   |
   |
LL | #[cfg_accessible(Struct::b)] //~ ERROR not sure

error: not sure whether the path is accessible or not
  --> /checkout/src/test/ui/conditional-compilation/cfg_accessible-not_sure.rs:19:18
   |
   |
LL | #[cfg_accessible(Struct::c)] //~ ERROR not sure

error: not sure whether the path is accessible or not
  --> /checkout/src/test/ui/conditional-compilation/cfg_accessible-not_sure.rs:29:18
   |
   |
LL | #[cfg_accessible(Union::a)] //~ ERROR not sure

error: not sure whether the path is accessible or not
  --> /checkout/src/test/ui/conditional-compilation/cfg_accessible-not_sure.rs:31:18
   |
   |
LL | #[cfg_accessible(Union::b)] //~ ERROR not sure

error: not sure whether the path is accessible or not
  --> /checkout/src/test/ui/conditional-compilation/cfg_accessible-not_sure.rs:33:18
   |
   |
LL | #[cfg_accessible(Union::c)] //~ ERROR not sure

error: not sure whether the path is accessible or not
  --> /checkout/src/test/ui/conditional-compilation/cfg_accessible-not_sure.rs:42:18
   |
   |
LL | #[cfg_accessible(Enum::B)] //~ ERROR not sure

error: not sure whether the path is accessible or not
  --> /checkout/src/test/ui/conditional-compilation/cfg_accessible-not_sure.rs:52:18
   |
   |
LL | #[cfg_accessible(Trait::foo)] //~ ERROR not sure

error: not sure whether the path is accessible or not
  --> /checkout/src/test/ui/conditional-compilation/cfg_accessible-not_sure.rs:59:18
   |
   |
LL | #[cfg_accessible(TypeAlias::A)] //~ ERROR not sure

error: not sure whether the path is accessible or not
  --> /checkout/src/test/ui/conditional-compilation/cfg_accessible-not_sure.rs:68:18
   |
   |
LL | #[cfg_accessible(ForeignType::A)] //~ ERROR not sure

error: not sure whether the path is accessible or not
  --> /checkout/src/test/ui/conditional-compilation/cfg_accessible-not_sure.rs:77:18
   |
   |
LL | #[cfg_accessible(AssocType::AssocType::A)] //~ ERROR not sure

error: not sure whether the path is accessible or not
  --> /checkout/src/test/ui/conditional-compilation/cfg_accessible-not_sure.rs:82:18
   |
   |
LL | #[cfg_accessible(u8::A)] //~ ERROR not sure

error: aborting due to 12 previous errors
------------------------------------------



---- [ui] src/test/ui/conditional-compilation/cfg_accessible-not_sure.rs#edition2021 stdout ----

41    |                  ^^^^^^^
42 
43 error: not sure whether the path is accessible or not
43 error: not sure whether the path is accessible or not
-   --> $DIR/cfg_accessible-not_sure.rs:51:18
+   --> $DIR/cfg_accessible-not_sure.rs:52:18
45    |
46 LL | #[cfg_accessible(Trait::foo)]

48 
49 error: not sure whether the path is accessible or not
-   --> $DIR/cfg_accessible-not_sure.rs:58:18
-   --> $DIR/cfg_accessible-not_sure.rs:58:18
+   --> $DIR/cfg_accessible-not_sure.rs:59:18
51    |
52 LL | #[cfg_accessible(TypeAlias::A)]

54 
55 error: not sure whether the path is accessible or not
-   --> $DIR/cfg_accessible-not_sure.rs:67:18
-   --> $DIR/cfg_accessible-not_sure.rs:67:18
+   --> $DIR/cfg_accessible-not_sure.rs:68:18
57    |
58 LL | #[cfg_accessible(ForeignType::A)]

60 
61 error: not sure whether the path is accessible or not
-   --> $DIR/cfg_accessible-not_sure.rs:76:18
-   --> $DIR/cfg_accessible-not_sure.rs:76:18
+   --> $DIR/cfg_accessible-not_sure.rs:77:18
63    |
64 LL | #[cfg_accessible(AssocType::AssocType::A)]

66 
67 error: not sure whether the path is accessible or not
-   --> $DIR/cfg_accessible-not_sure.rs:81:18
-   --> $DIR/cfg_accessible-not_sure.rs:81:18
+   --> $DIR/cfg_accessible-not_sure.rs:82:18
69    |
70 LL | #[cfg_accessible(u8::A)]


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg_accessible-not_sure.edition2021/cfg_accessible-not_sure.edition2021.stderr
To only update this specific test, also pass `--test-args conditional-compilation/cfg_accessible-not_sure.rs`

error in revision `edition2021`: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/conditional-compilation/cfg_accessible-not_sure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "edition2021" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg_accessible-not_sure.edition2021" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/conditional-compilation/cfg_accessible-not_sure.edition2021/auxiliary"
stdout: none
--- stderr -------------------------------
error: not sure whether the path is accessible or not
  --> /checkout/src/test/ui/conditional-compilation/cfg_accessible-not_sure.rs:15:18
   |
LL | #[cfg_accessible(Struct::a)] //~ ERROR not sure

error: not sure whether the path is accessible or not
  --> /checkout/src/test/ui/conditional-compilation/cfg_accessible-not_sure.rs:17:18
   |
   |
LL | #[cfg_accessible(Struct::b)] //~ ERROR not sure

error: not sure whether the path is accessible or not
  --> /checkout/src/test/ui/conditional-compilation/cfg_accessible-not_sure.rs:19:18
   |
   |
LL | #[cfg_accessible(Struct::c)] //~ ERROR not sure

error: not sure whether the path is accessible or not
  --> /checkout/src/test/ui/conditional-compilation/cfg_accessible-not_sure.rs:29:18
   |
   |
LL | #[cfg_accessible(Union::a)] //~ ERROR not sure

error: not sure whether the path is accessible or not
  --> /checkout/src/test/ui/conditional-compilation/cfg_accessible-not_sure.rs:31:18
   |
   |
LL | #[cfg_accessible(Union::b)] //~ ERROR not sure

error: not sure whether the path is accessible or not
  --> /checkout/src/test/ui/conditional-compilation/cfg_accessible-not_sure.rs:33:18
   |
   |
LL | #[cfg_accessible(Union::c)] //~ ERROR not sure

error: not sure whether the path is accessible or not
  --> /checkout/src/test/ui/conditional-compilation/cfg_accessible-not_sure.rs:42:18
   |
   |
LL | #[cfg_accessible(Enum::B)] //~ ERROR not sure

error: not sure whether the path is accessible or not
  --> /checkout/src/test/ui/conditional-compilation/cfg_accessible-not_sure.rs:52:18
   |
   |
LL | #[cfg_accessible(Trait::foo)] //~ ERROR not sure

error: not sure whether the path is accessible or not
  --> /checkout/src/test/ui/conditional-compilation/cfg_accessible-not_sure.rs:59:18
   |
   |
LL | #[cfg_accessible(TypeAlias::A)] //~ ERROR not sure

error: not sure whether the path is accessible or not
  --> /checkout/src/test/ui/conditional-compilation/cfg_accessible-not_sure.rs:68:18
   |
   |
LL | #[cfg_accessible(ForeignType::A)] //~ ERROR not sure

error: not sure whether the path is accessible or not
  --> /checkout/src/test/ui/conditional-compilation/cfg_accessible-not_sure.rs:77:18
   |
   |
LL | #[cfg_accessible(AssocType::AssocType::A)] //~ ERROR not sure

error: not sure whether the path is accessible or not
  --> /checkout/src/test/ui/conditional-compilation/cfg_accessible-not_sure.rs:82:18
   |
   |
LL | #[cfg_accessible(u8::A)] //~ ERROR not sure

error: aborting due to 12 previous errors
------------------------------------------

