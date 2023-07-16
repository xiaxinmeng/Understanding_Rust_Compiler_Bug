plain

53    |
54 help: provide the argument
55    |
- LL | fn parse_type_2(iter: fn(&u8)->&u8) -> &str { iter({&u8}) }
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-    |                                               ~~~~~~~~~~~
+ LL | fn parse_type_2(iter: fn(&u8)->&u8) -> &str { iter(/* &u8 */) }
58 
59 error[E0308]: mismatched types
60   --> $DIR/issue-26638.rs:5:47

---
To only update this specific test, also pass `--test-args lifetimes/issue-26638.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/issue-26638.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-26638" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-26638/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0106]: missing lifetime specifier
   |
   |
LL | fn parse_type(iter: Box<dyn Iterator<Item=&str>+'static>) -> &str { iter.next() }
   |                     ------------------------------------     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say which one of `iter`'s 2 lifetimes it is borrowed from
help: consider introducing a named lifetime parameter
   |
LL | fn parse_type<'a>(iter: Box<dyn Iterator<Item=&'a str>+'static>) -> &'a str { iter.next() }
   |              ++++                              ++                    ++
error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/lifetimes/issue-26638.rs:5:40
   |
   |
LL | fn parse_type_2(iter: fn(&u8)->&u8) -> &str { iter() }
   |                                        ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
LL | fn parse_type_2(iter: fn(&u8)->&u8) -> &'static str { iter() }

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/lifetimes/issue-26638.rs:10:22
   |
   |
LL | fn parse_type_3() -> &str { unimplemented!() }
   |                      ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
LL | fn parse_type_3() -> &'static str { unimplemented!() }

error[E0308]: mismatched types
  --> /checkout/src/test/ui/lifetimes/issue-26638.rs:1:69
   |
   |
LL | fn parse_type(iter: Box<dyn Iterator<Item=&str>+'static>) -> &str { iter.next() }
   |                                                              ----   ^^^^^^^^^^^ expected `&str`, found enum `Option`
   |                                                              expected `&'static str` because of return type
   |
   = note: expected reference `&'static str`
                   found enum `Option<&str>`
                   found enum `Option<&str>`

error[E0061]: this function takes 1 argument but 0 arguments were supplied
  --> /checkout/src/test/ui/lifetimes/issue-26638.rs:5:47
   |
LL | fn parse_type_2(iter: fn(&u8)->&u8) -> &str { iter() }
   |                                               ^^^^-- an argument of type `&u8` is missing
help: provide the argument
   |
   |
LL | fn parse_type_2(iter: fn(&u8)->&u8) -> &str { iter(/* &u8 */) }

error[E0308]: mismatched types
  --> /checkout/src/test/ui/lifetimes/issue-26638.rs:5:47
   |
   |
LL | fn parse_type_2(iter: fn(&u8)->&u8) -> &str { iter() }
   |                                        ----   ^^^^^^ expected `str`, found `u8`
   |                                        expected `&'static str` because of return type
   |
   = note: expected reference `&'static str`
              found reference `&u8`
