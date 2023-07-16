plain
---- [ui] ui/binding/fn-arg-incomplete-pattern-drop-order.rs stdout ----

error: test run failed!
status: exit status: 1
command: "/emsdk-portable/node/latest/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/fn-arg-incomplete-pattern-drop-order/a.js"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/binding/fn-arg-incomplete-pattern-drop-order.rs:22:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
undefined
exception thrown: RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/fn-arg-incomplete-pattern-drop-order/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/fn-arg-incomplete-pattern-drop-order/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/fn-arg-incomplete-pattern-drop-order/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/fn-arg-incomplete-pattern-drop-order/a.js:4327:7)
    at <anonymous>:wasm-function[127]:0x5046
    at <anonymous>:wasm-function[156]:0x6431
    at <anonymous>:wasm-function[153]:0x631f
    at <anonymous>:wasm-function[146]:0x6053
    at <anonymous>:wasm-function[21]:0x695
    at <anonymous>:wasm-function[20]:0x65f,RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/fn-arg-incomplete-pattern-drop-order/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/fn-arg-incomplete-pattern-drop-order/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/fn-arg-incomplete-pattern-drop-order/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/fn-arg-incomplete-pattern-drop-order/a.js:4327:7)
    at <anonymous>:wasm-function[127]:0x5046
    at <anonymous>:wasm-function[156]:0x6431
    at <anonymous>:wasm-function[153]:0x631f
    at <anonymous>:wasm-function[146]:0x6053
    at <anonymous>:wasm-function[21]:0x695
    at <anonymous>:wasm-function[20]:0x65f
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/fn-arg-incomplete-pattern-drop-order/a.js:1656:11)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/fn-arg-incomplete-pattern-drop-order/a.js:4327:7)
    at <anonymous>:wasm-function[127]:0x5046
    at <anonymous>:wasm-function[156]:0x6431
    at <anonymous>:wasm-function[153]:0x631f
    at <anonymous>:wasm-function[146]:0x6053
    at <anonymous>:wasm-function[21]:0x695
    at <anonymous>:wasm-function[20]:0x65f
    at <anonymous>:wasm-function[24]:0x6e1
    at <anonymous>:wasm-function[30]:0x98c
------------------------------------------


---- [ui] ui/builtin-clone-unwind.rs stdout ----
---- [ui] ui/builtin-clone-unwind.rs stdout ----

error: test run failed!
status: exit status: 1
command: "/emsdk-portable/node/latest/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/builtin-clone-unwind/a.js"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'main' panicked at 'oops', /checkout/src/test/ui/builtin-clone-unwind.rs:19:13
undefined
undefined
exception thrown: RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/builtin-clone-unwind/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/builtin-clone-unwind/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/builtin-clone-unwind/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/builtin-clone-unwind/a.js:4327:7)
    at <anonymous>:wasm-function[116]:0x460e
    at <anonymous>:wasm-function[145]:0x59f4
    at <anonymous>:wasm-function[142]:0x58e2
    at <anonymous>:wasm-function[135]:0x5616
    at <anonymous>:wasm-function[21]:0x67d
    at <anonymous>:wasm-function[20]:0x647,RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/builtin-clone-unwind/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/builtin-clone-unwind/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/builtin-clone-unwind/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/builtin-clone-unwind/a.js:4327:7)
    at <anonymous>:wasm-function[116]:0x460e
    at <anonymous>:wasm-function[145]:0x59f4
    at <anonymous>:wasm-function[142]:0x58e2
    at <anonymous>:wasm-function[135]:0x5616
    at <anonymous>:wasm-function[21]:0x67d
    at <anonymous>:wasm-function[20]:0x647
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/builtin-clone-unwind/a.js:1656:11)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/builtin-clone-unwind/a.js:4327:7)
    at <anonymous>:wasm-function[116]:0x460e
    at <anonymous>:wasm-function[145]:0x59f4
    at <anonymous>:wasm-function[142]:0x58e2
    at <anonymous>:wasm-function[135]:0x5616
    at <anonymous>:wasm-function[21]:0x67d
    at <anonymous>:wasm-function[20]:0x647
    at <anonymous>:wasm-function[24]:0x6cb
    at <anonymous>:wasm-function[28]:0x749
------------------------------------------


---- [ui] ui/catch-unwind-bang.rs stdout ----
---- [ui] ui/catch-unwind-bang.rs stdout ----

error: test run failed!
status: exit status: 1
command: "/emsdk-portable/node/latest/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/catch-unwind-bang/a.js"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/catch-unwind-bang.rs:6:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
undefined
exception thrown: RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/catch-unwind-bang/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/catch-unwind-bang/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/catch-unwind-bang/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/catch-unwind-bang/a.js:4327:7)
    at <anonymous>:wasm-function[118]:0x461d
    at <anonymous>:wasm-function[147]:0x5a04
    at <anonymous>:wasm-function[144]:0x58f2
    at <anonymous>:wasm-function[137]:0x5626
    at <anonymous>:wasm-function[21]:0x680
    at <anonymous>:wasm-function[20]:0x64a,RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/catch-unwind-bang/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/catch-unwind-bang/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/catch-unwind-bang/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/catch-unwind-bang/a.js:4327:7)
    at <anonymous>:wasm-function[118]:0x461d
    at <anonymous>:wasm-function[147]:0x5a04
    at <anonymous>:wasm-function[144]:0x58f2
    at <anonymous>:wasm-function[137]:0x5626
    at <anonymous>:wasm-function[21]:0x680
    at <anonymous>:wasm-function[20]:0x64a
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/catch-unwind-bang/a.js:1656:11)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/catch-unwind-bang/a.js:4327:7)
    at <anonymous>:wasm-function[118]:0x461d
    at <anonymous>:wasm-function[147]:0x5a04
    at <anonymous>:wasm-function[144]:0x58f2
    at <anonymous>:wasm-function[137]:0x5626
    at <anonymous>:wasm-function[21]:0x680
    at <anonymous>:wasm-function[20]:0x64a
    at <anonymous>:wasm-function[24]:0x6ce
    at <anonymous>:wasm-function[26]:0x6da
------------------------------------------


---- [ui] ui/drop/dynamic-drop.rs stdout ----
---- [ui] ui/drop/dynamic-drop.rs stdout ----

error: test run failed!
status: exit status: 1
command: "/emsdk-portable/node/latest/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop/a.js"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'main' panicked at 'Box<dyn Any>', /checkout/src/test/ui/drop/dynamic-drop.rs:48:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
undefined
exception thrown: RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop/a.js:4327:7)
    at <anonymous>:wasm-function[179]:0xb56c
    at <anonymous>:wasm-function[208]:0xc96d
    at <anonymous>:wasm-function[205]:0xc858
    at <anonymous>:wasm-function[198]:0xc58a
    at <anonymous>:wasm-function[21]:0x6b3
    at <anonymous>:wasm-function[20]:0x683,RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop/a.js:4327:7)
    at <anonymous>:wasm-function[179]:0xb56c
    at <anonymous>:wasm-function[208]:0xc96d
    at <anonymous>:wasm-function[205]:0xc858
    at <anonymous>:wasm-function[198]:0xc58a
    at <anonymous>:wasm-function[21]:0x6b3
    at <anonymous>:wasm-function[20]:0x683
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop/a.js:1656:11)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop/a.js:4327:7)
    at <anonymous>:wasm-function[179]:0xb56c
    at <anonymous>:wasm-function[208]:0xc96d
    at <anonymous>:wasm-function[205]:0xc858
    at <anonymous>:wasm-function[198]:0xc58a
    at <anonymous>:wasm-function[21]:0x6b3
    at <anonymous>:wasm-function[20]:0x683
    at <anonymous>:wasm-function[75]:0x2217
    at <anonymous>:wasm-function[74]:0x220c
------------------------------------------


---- [ui] ui/drop/dynamic-drop-async.rs stdout ----
---- [ui] ui/drop/dynamic-drop-async.rs stdout ----

error: test run failed!
status: exit status: 1
command: "/emsdk-portable/node/latest/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop-async/a.js"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'main' panicked at 'Box<dyn Any>', /checkout/src/test/ui/drop/dynamic-drop-async.rs:80:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
undefined
exception thrown: RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop-async/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop-async/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop-async/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop-async/a.js:4327:7)
    at <anonymous>:wasm-function[187]:0xbd89
    at <anonymous>:wasm-function[216]:0xd194
    at <anonymous>:wasm-function[213]:0xd07f
    at <anonymous>:wasm-function[206]:0xcdb1
    at <anonymous>:wasm-function[22]:0x6cf
    at <anonymous>:wasm-function[21]:0x69f,RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop-async/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop-async/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop-async/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop-async/a.js:4327:7)
    at <anonymous>:wasm-function[187]:0xbd89
    at <anonymous>:wasm-function[216]:0xd194
    at <anonymous>:wasm-function[213]:0xd07f
    at <anonymous>:wasm-function[206]:0xcdb1
    at <anonymous>:wasm-function[22]:0x6cf
    at <anonymous>:wasm-function[21]:0x69f
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop-async/a.js:1656:11)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop-async/a.js:4327:7)
    at <anonymous>:wasm-function[187]:0xbd89
    at <anonymous>:wasm-function[216]:0xd194
    at <anonymous>:wasm-function[213]:0xd07f
    at <anonymous>:wasm-function[206]:0xcdb1
    at <anonymous>:wasm-function[22]:0x6cf
    at <anonymous>:wasm-function[21]:0x69f
    at <anonymous>:wasm-function[83]:0x498b
    at <anonymous>:wasm-function[82]:0x497f
------------------------------------------


---- [ui] ui/generator/panic-drops.rs stdout ----
---- [ui] ui/generator/panic-drops.rs stdout ----

error: test run failed!
status: exit status: 1
command: "/emsdk-portable/node/latest/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/panic-drops/a.js"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/generator/panic-drops.rs:31:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
undefined
exception thrown: RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/panic-drops/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/panic-drops/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/panic-drops/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/panic-drops/a.js:4327:7)
    at <anonymous>:wasm-function[117]:0x46b8
    at <anonymous>:wasm-function[146]:0x5a9e
    at <anonymous>:wasm-function[143]:0x598c
    at <anonymous>:wasm-function[136]:0x56c0
    at <anonymous>:wasm-function[21]:0x680
    at <anonymous>:wasm-function[20]:0x64a,RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/panic-drops/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/panic-drops/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/panic-drops/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/panic-drops/a.js:4327:7)
    at <anonymous>:wasm-function[117]:0x46b8
    at <anonymous>:wasm-function[146]:0x5a9e
    at <anonymous>:wasm-function[143]:0x598c
    at <anonymous>:wasm-function[136]:0x56c0
    at <anonymous>:wasm-function[21]:0x680
    at <anonymous>:wasm-function[20]:0x64a
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/panic-drops/a.js:1656:11)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/panic-drops/a.js:4327:7)
    at <anonymous>:wasm-function[117]:0x46b8
    at <anonymous>:wasm-function[146]:0x5a9e
    at <anonymous>:wasm-function[143]:0x598c
    at <anonymous>:wasm-function[136]:0x56c0
    at <anonymous>:wasm-function[21]:0x680
    at <anonymous>:wasm-function[20]:0x64a
    at <anonymous>:wasm-function[24]:0x6cd
    at <anonymous>:wasm-function[30]:0x80b
------------------------------------------


---- [ui] ui/generator/panic-safe.rs stdout ----
---- [ui] ui/generator/panic-safe.rs stdout ----

error: test run failed!
status: exit status: 1
command: "/emsdk-portable/node/latest/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/panic-safe/a.js"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/generator/panic-safe.rs:15:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
undefined
exception thrown: RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/panic-safe/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/panic-safe/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/panic-safe/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/panic-safe/a.js:4327:7)
    at <anonymous>:wasm-function[116]:0x460e
    at <anonymous>:wasm-function[145]:0x59f4
    at <anonymous>:wasm-function[142]:0x58e2
    at <anonymous>:wasm-function[135]:0x5616
    at <anonymous>:wasm-function[21]:0x67d
    at <anonymous>:wasm-function[20]:0x647,RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/panic-safe/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/panic-safe/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/panic-safe/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/panic-safe/a.js:4327:7)
    at <anonymous>:wasm-function[116]:0x460e
    at <anonymous>:wasm-function[145]:0x59f4
    at <anonymous>:wasm-function[142]:0x58e2
    at <anonymous>:wasm-function[135]:0x5616
    at <anonymous>:wasm-function[21]:0x67d
    at <anonymous>:wasm-function[20]:0x647
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/panic-safe/a.js:1656:11)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/panic-safe/a.js:4327:7)
    at <anonymous>:wasm-function[116]:0x460e
    at <anonymous>:wasm-function[145]:0x59f4
    at <anonymous>:wasm-function[142]:0x58e2
    at <anonymous>:wasm-function[135]:0x5616
    at <anonymous>:wasm-function[21]:0x67d
    at <anonymous>:wasm-function[20]:0x647
    at <anonymous>:wasm-function[24]:0x6cb
    at <anonymous>:wasm-function[28]:0x749
------------------------------------------


---- [ui] ui/generator/resume-after-return.rs stdout ----
---- [ui] ui/generator/resume-after-return.rs stdout ----

error: test run failed!
status: exit status: 1
command: "/emsdk-portable/node/latest/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/resume-after-return/a.js"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'main' panicked at 'generator resumed after completion', /checkout/src/test/ui/generator/resume-after-return.rs:13:19
undefined
undefined
exception thrown: RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/resume-after-return/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/resume-after-return/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/resume-after-return/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/resume-after-return/a.js:4327:7)
    at <anonymous>:wasm-function[111]:0x44f3
    at <anonymous>:wasm-function[140]:0x58d9
    at <anonymous>:wasm-function[137]:0x57c7
    at <anonymous>:wasm-function[130]:0x54fb
    at <anonymous>:wasm-function[121]:0x4bdc
    at <anonymous>:wasm-function[120]:0x4b64,RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/resume-after-return/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/resume-after-return/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/resume-after-return/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/resume-after-return/a.js:4327:7)
    at <anonymous>:wasm-function[111]:0x44f3
    at <anonymous>:wasm-function[140]:0x58d9
    at <anonymous>:wasm-function[137]:0x57c7
    at <anonymous>:wasm-function[130]:0x54fb
    at <anonymous>:wasm-function[121]:0x4bdc
    at <anonymous>:wasm-function[120]:0x4b64
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/resume-after-return/a.js:1656:11)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/resume-after-return/a.js:4327:7)
    at <anonymous>:wasm-function[111]:0x44f3
    at <anonymous>:wasm-function[140]:0x58d9
    at <anonymous>:wasm-function[137]:0x57c7
    at <anonymous>:wasm-function[130]:0x54fb
    at <anonymous>:wasm-function[121]:0x4bdc
    at <anonymous>:wasm-function[120]:0x4b64
    at <anonymous>:wasm-function[129]:0x5122
    at <anonymous>:wasm-function[152]:0x5e70
------------------------------------------


---- [ui] ui/hygiene/panic-location.rs stdout ----
---- [ui] ui/hygiene/panic-location.rs stdout ----
diff of run.stderr:

1 thread 'main' panicked at 'capacity overflow', $SRC_DIR/alloc/src/collections/vec_deque/mod.rs:LL:COL
+ undefined
+ undefined
+ exception thrown: RuntimeError: abort(undefined) at Error
+     at jsStackTrace ($TEST_BUILD_DIR/hygiene/panic-location/a.js:1893:17)
+     at stackTrace ($TEST_BUILD_DIR/hygiene/panic-location/a.js:1910:16)
+     at abort ($TEST_BUILD_DIR/hygiene/panic-location/a.js:1650:44)
+     at _abort ($TEST_BUILD_DIR/hygiene/panic-location/a.js:4327:7)
+     at <anonymous>:wasm-function[111]:0x44f3
+     at <anonymous>:wasm-function[140]:0x58d9
+     at <anonymous>:wasm-function[137]:0x57c7
+     at <anonymous>:wasm-function[130]:0x54fb
+     at <anonymous>:wasm-function[121]:0x4bdc
+     at <anonymous>:wasm-function[120]:0x4b64,RuntimeError: abort(undefined) at Error
+     at jsStackTrace ($TEST_BUILD_DIR/hygiene/panic-location/a.js:1893:17)
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=wasm32-unknown-emscripten
+     at stackTrace ($TEST_BUILD_DIR/hygiene/panic-location/a.js:1910:16)
+     at abort ($TEST_BUILD_DIR/hygiene/panic-location/a.js:1650:44)
+     at _abort ($TEST_BUILD_DIR/hygiene/panic-location/a.js:4327:7)
+     at <anonymous>:wasm-function[111]:0x44f3
+     at <anonymous>:wasm-function[140]:0x58d9
+     at <anonymous>:wasm-function[137]:0x57c7
+     at <anonymous>:wasm-function[130]:0x54fb
+     at <anonymous>:wasm-function[121]:0x4bdc
+     at <anonymous>:wasm-function[120]:0x4b64
+     at abort ($TEST_BUILD_DIR/hygiene/panic-location/a.js:1656:11)
+     at _abort ($TEST_BUILD_DIR/hygiene/panic-location/a.js:4327:7)
+     at <anonymous>:wasm-function[111]:0x44f3
+     at <anonymous>:wasm-function[140]:0x58d9
+     at <anonymous>:wasm-function[137]:0x57c7
+     at <anonymous>:wasm-function[130]:0x54fb
+     at <anonymous>:wasm-function[121]:0x4bdc
+     at <anonymous>:wasm-function[120]:0x4b64
+     at <anonymous>:wasm-function[129]:0x5122
+     at <anonymous>:wasm-function[152]:0x5e70


The actual run.stderr differed from the expected run.stderr.
The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/panic-location/panic-location.run.stderr
error: 1 errors occurred comparing run output.
status: exit status: 1
status: exit status: 1
command: "/emsdk-portable/node/latest/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/panic-location/a.js"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'main' panicked at 'capacity overflow', /checkout/library/alloc/src/collections/vec_deque/mod.rs:546:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
undefined
exception thrown: RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/panic-location/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/panic-location/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/panic-location/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/panic-location/a.js:4327:7)
    at <anonymous>:wasm-function[111]:0x44f3
    at <anonymous>:wasm-function[140]:0x58d9
    at <anonymous>:wasm-function[137]:0x57c7
    at <anonymous>:wasm-function[130]:0x54fb
    at <anonymous>:wasm-function[121]:0x4bdc
    at <anonymous>:wasm-function[120]:0x4b64,RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/panic-location/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/panic-location/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/panic-location/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/panic-location/a.js:4327:7)
    at <anonymous>:wasm-function[111]:0x44f3
    at <anonymous>:wasm-function[140]:0x58d9
    at <anonymous>:wasm-function[137]:0x57c7
    at <anonymous>:wasm-function[130]:0x54fb
    at <anonymous>:wasm-function[121]:0x4bdc
    at <anonymous>:wasm-function[120]:0x4b64
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/panic-location/a.js:1656:11)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/panic-location/a.js:4327:7)
    at <anonymous>:wasm-function[111]:0x44f3
    at <anonymous>:wasm-function[140]:0x58d9
    at <anonymous>:wasm-function[137]:0x57c7
    at <anonymous>:wasm-function[130]:0x54fb
    at <anonymous>:wasm-function[121]:0x4bdc
    at <anonymous>:wasm-function[120]:0x4b64
    at <anonymous>:wasm-function[129]:0x5122
    at <anonymous>:wasm-function[152]:0x5e70
------------------------------------------


---- [ui] ui/intrinsics/panic-uninitialized-zeroed.rs#thir stdout ----
---- [ui] ui/intrinsics/panic-uninitialized-zeroed.rs#thir stdout ----

error in revision `thir`: test run failed!
status: exit status: 1
command: "/emsdk-portable/node/latest/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/panic-uninitialized-zeroed.thir/a.js"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'main' panicked at 'attempted to instantiate uninhabited type `!`', /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:69:16
undefined
undefined
exception thrown: RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/panic-uninitialized-zeroed.thir/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/panic-uninitialized-zeroed.thir/a.js:1910:16)
---
To only update this specific test, also pass `--test-args panic-runtime/transitive-link-a-bunch.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-runtime/transitive-link-a-bunch.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/transitive-link-a-bunch" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/transitive-link-a-bunch/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: cannot link together two panic runtimes: panic_runtime_unwind and panic_runtime_abort
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/panic-runtime/want-unwind-got-abort.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-runtime/want-unwind-got-abort.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/want-unwind-got-abort" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/want-unwind-got-abort/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/panics/location-detail-panic-no-file.rs stdout ----
diff of run.stderr:

1 thread 'main' panicked at 'file-redacted', <redacted>:6:5
+ undefined
+ undefined
+ exception thrown: RuntimeError: abort(undefined) at Error
+     at jsStackTrace ($TEST_BUILD_DIR/panics/location-detail-panic-no-file/a.js:1893:17)
+     at stackTrace ($TEST_BUILD_DIR/panics/location-detail-panic-no-file/a.js:1910:16)
+     at abort ($TEST_BUILD_DIR/panics/location-detail-panic-no-file/a.js:1650:44)
+     at _abort ($TEST_BUILD_DIR/panics/location-detail-panic-no-file/a.js:4327:7)
+     at <anonymous>:wasm-function[116]:0x460e
+     at <anonymous>:wasm-function[145]:0x59f4
+     at <anonymous>:wasm-function[142]:0x58e2
+     at <anonymous>:wasm-function[135]:0x5616
+     at <anonymous>:wasm-function[21]:0x67d
+     at <anonymous>:wasm-function[20]:0x647,RuntimeError: abort(undefined) at Error
+     at jsStackTrace ($TEST_BUILD_DIR/panics/location-detail-panic-no-file/a.js:1893:17)
+     at stackTrace ($TEST_BUILD_DIR/panics/location-detail-panic-no-file/a.js:1910:16)
+     at abort ($TEST_BUILD_DIR/panics/location-detail-panic-no-file/a.js:1650:44)
+     at _abort ($TEST_BUILD_DIR/panics/location-detail-panic-no-file/a.js:4327:7)
+     at <anonymous>:wasm-function[116]:0x460e
+     at <anonymous>:wasm-function[145]:0x59f4
+     at <anonymous>:wasm-function[142]:0x58e2
+     at <anonymous>:wasm-function[135]:0x5616
+     at <anonymous>:wasm-function[21]:0x67d
+     at <anonymous>:wasm-function[20]:0x647
+     at abort ($TEST_BUILD_DIR/panics/location-detail-panic-no-file/a.js:1656:11)
+     at _abort ($TEST_BUILD_DIR/panics/location-detail-panic-no-file/a.js:4327:7)
+     at <anonymous>:wasm-function[116]:0x460e
+     at <anonymous>:wasm-function[145]:0x59f4
+     at <anonymous>:wasm-function[142]:0x58e2
+     at <anonymous>:wasm-function[135]:0x5616
+     at <anonymous>:wasm-function[21]:0x67d
+     at <anonymous>:wasm-function[20]:0x647
+     at <anonymous>:wasm-function[24]:0x6cb
+     at <anonymous>:wasm-function[28]:0x749


The actual run.stderr differed from the expected run.stderr.
The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-file/location-detail-panic-no-file.run.stderr
error: 1 errors occurred comparing run output.
status: exit status: 1
status: exit status: 1
command: "/emsdk-portable/node/latest/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-file/a.js"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'main' panicked at 'file-redacted', <redacted>:6:5
undefined
undefined
exception thrown: RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-file/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-file/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-file/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-file/a.js:4327:7)
    at <anonymous>:wasm-function[116]:0x460e
    at <anonymous>:wasm-function[145]:0x59f4
    at <anonymous>:wasm-function[142]:0x58e2
    at <anonymous>:wasm-function[135]:0x5616
    at <anonymous>:wasm-function[21]:0x67d
    at <anonymous>:wasm-function[20]:0x647,RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-file/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-file/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-file/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-file/a.js:4327:7)
    at <anonymous>:wasm-function[116]:0x460e
    at <anonymous>:wasm-function[145]:0x59f4
    at <anonymous>:wasm-function[142]:0x58e2
    at <anonymous>:wasm-function[135]:0x5616
    at <anonymous>:wasm-function[21]:0x67d
    at <anonymous>:wasm-function[20]:0x647
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-file/a.js:1656:11)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-file/a.js:4327:7)
    at <anonymous>:wasm-function[116]:0x460e
    at <anonymous>:wasm-function[145]:0x59f4
    at <anonymous>:wasm-function[142]:0x58e2
    at <anonymous>:wasm-function[135]:0x5616
    at <anonymous>:wasm-function[21]:0x67d
    at <anonymous>:wasm-function[20]:0x647
    at <anonymous>:wasm-function[24]:0x6cb
    at <anonymous>:wasm-function[28]:0x749
------------------------------------------


---- [ui] ui/panics/location-detail-panic-no-column.rs stdout ----
---- [ui] ui/panics/location-detail-panic-no-column.rs stdout ----
diff of run.stderr:

1 thread 'main' panicked at 'column-redacted', $DIR/location-detail-panic-no-column.rs:6:0
+ undefined
+ undefined
+ exception thrown: RuntimeError: abort(undefined) at Error
+     at jsStackTrace ($TEST_BUILD_DIR/panics/location-detail-panic-no-column/a.js:1893:17)
+     at stackTrace ($TEST_BUILD_DIR/panics/location-detail-panic-no-column/a.js:1910:16)
+     at abort ($TEST_BUILD_DIR/panics/location-detail-panic-no-column/a.js:1650:44)
+     at _abort ($TEST_BUILD_DIR/panics/location-detail-panic-no-column/a.js:4327:7)
+     at <anonymous>:wasm-function[116]:0x460e
+     at <anonymous>:wasm-function[145]:0x59f4
+     at <anonymous>:wasm-function[142]:0x58e2
+     at <anonymous>:wasm-function[135]:0x5616
+     at <anonymous>:wasm-function[21]:0x67d
+     at <anonymous>:wasm-function[20]:0x647,RuntimeError: abort(undefined) at Error
+     at jsStackTrace ($TEST_BUILD_DIR/panics/location-detail-panic-no-column/a.js:1893:17)
+     at stackTrace ($TEST_BUILD_DIR/panics/location-detail-panic-no-column/a.js:1910:16)
+     at abort ($TEST_BUILD_DIR/panics/location-detail-panic-no-column/a.js:1650:44)
+     at _abort ($TEST_BUILD_DIR/panics/location-detail-panic-no-column/a.js:4327:7)
+     at <anonymous>:wasm-function[116]:0x460e
+     at <anonymous>:wasm-function[145]:0x59f4
+     at <anonymous>:wasm-function[142]:0x58e2
+     at <anonymous>:wasm-function[135]:0x5616
+     at <anonymous>:wasm-function[21]:0x67d
+     at <anonymous>:wasm-function[20]:0x647
+     at abort ($TEST_BUILD_DIR/panics/location-detail-panic-no-column/a.js:1656:11)
+     at _abort ($TEST_BUILD_DIR/panics/location-detail-panic-no-column/a.js:4327:7)
+     at <anonymous>:wasm-function[116]:0x460e
+     at <anonymous>:wasm-function[145]:0x59f4
+     at <anonymous>:wasm-function[142]:0x58e2
+     at <anonymous>:wasm-function[135]:0x5616
+     at <anonymous>:wasm-function[21]:0x67d
+     at <anonymous>:wasm-function[20]:0x647
+     at <anonymous>:wasm-function[24]:0x6cb
+     at <anonymous>:wasm-function[28]:0x749


The actual run.stderr differed from the expected run.stderr.
The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-column/location-detail-panic-no-column.run.stderr
error: 1 errors occurred comparing run output.
status: exit status: 1
status: exit status: 1
command: "/emsdk-portable/node/latest/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-column/a.js"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'main' panicked at 'column-redacted', /checkout/src/test/ui/panics/location-detail-panic-no-column.rs:6:0
undefined
undefined
exception thrown: RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-column/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-column/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-column/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-column/a.js:4327:7)
    at <anonymous>:wasm-function[116]:0x460e
    at <anonymous>:wasm-function[145]:0x59f4
    at <anonymous>:wasm-function[142]:0x58e2
    at <anonymous>:wasm-function[135]:0x5616
    at <anonymous>:wasm-function[21]:0x67d
    at <anonymous>:wasm-function[20]:0x647,RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-column/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-column/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-column/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-column/a.js:4327:7)
    at <anonymous>:wasm-function[116]:0x460e
    at <anonymous>:wasm-function[145]:0x59f4
    at <anonymous>:wasm-function[142]:0x58e2
    at <anonymous>:wasm-function[135]:0x5616
    at <anonymous>:wasm-function[21]:0x67d
    at <anonymous>:wasm-function[20]:0x647
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-column/a.js:1656:11)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-column/a.js:4327:7)
    at <anonymous>:wasm-function[116]:0x460e
    at <anonymous>:wasm-function[145]:0x59f4
    at <anonymous>:wasm-function[142]:0x58e2
    at <anonymous>:wasm-function[135]:0x5616
    at <anonymous>:wasm-function[21]:0x67d
    at <anonymous>:wasm-function[20]:0x647
    at <anonymous>:wasm-function[24]:0x6cb
    at <anonymous>:wasm-function[28]:0x749
------------------------------------------


---- [ui] ui/panics/location-detail-panic-no-line.rs stdout ----
---- [ui] ui/panics/location-detail-panic-no-line.rs stdout ----
diff of run.stderr:

1 thread 'main' panicked at 'line-redacted', $DIR/location-detail-panic-no-line.rs:0:5
+ undefined
+ undefined
+ exception thrown: RuntimeError: abort(undefined) at Error
+     at jsStackTrace ($TEST_BUILD_DIR/panics/location-detail-panic-no-line/a.js:1893:17)
+     at stackTrace ($TEST_BUILD_DIR/panics/location-detail-panic-no-line/a.js:1910:16)
+     at abort ($TEST_BUILD_DIR/panics/location-detail-panic-no-line/a.js:1650:44)
+     at _abort ($TEST_BUILD_DIR/panics/location-detail-panic-no-line/a.js:4327:7)
+     at <anonymous>:wasm-function[116]:0x460e
+     at <anonymous>:wasm-function[145]:0x59f4
+     at <anonymous>:wasm-function[142]:0x58e2
+     at <anonymous>:wasm-function[135]:0x5616
+     at <anonymous>:wasm-function[21]:0x67d
+     at <anonymous>:wasm-function[20]:0x647,RuntimeError: abort(undefined) at Error
+     at jsStackTrace ($TEST_BUILD_DIR/panics/location-detail-panic-no-line/a.js:1893:17)
+     at stackTrace ($TEST_BUILD_DIR/panics/location-detail-panic-no-line/a.js:1910:16)
+     at abort ($TEST_BUILD_DIR/panics/location-detail-panic-no-line/a.js:1650:44)
+     at _abort ($TEST_BUILD_DIR/panics/location-detail-panic-no-line/a.js:4327:7)
+     at <anonymous>:wasm-function[116]:0x460e
+     at <anonymous>:wasm-function[145]:0x59f4
+     at <anonymous>:wasm-function[142]:0x58e2
+     at <anonymous>:wasm-function[135]:0x5616
+     at <anonymous>:wasm-function[21]:0x67d
+     at <anonymous>:wasm-function[20]:0x647
+     at abort ($TEST_BUILD_DIR/panics/location-detail-panic-no-line/a.js:1656:11)
+     at _abort ($TEST_BUILD_DIR/panics/location-detail-panic-no-line/a.js:4327:7)
+     at <anonymous>:wasm-function[116]:0x460e
+     at <anonymous>:wasm-function[145]:0x59f4
+     at <anonymous>:wasm-function[142]:0x58e2
+     at <anonymous>:wasm-function[135]:0x5616
+     at <anonymous>:wasm-function[21]:0x67d
+     at <anonymous>:wasm-function[20]:0x647
+     at <anonymous>:wasm-function[24]:0x6cb
+     at <anonymous>:wasm-function[28]:0x749


The actual run.stderr differed from the expected run.stderr.
The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-line/location-detail-panic-no-line.run.stderr
error: 1 errors occurred comparing run output.
status: exit status: 1
status: exit status: 1
command: "/emsdk-portable/node/latest/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-line/a.js"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'main' panicked at 'line-redacted', /checkout/src/test/ui/panics/location-detail-panic-no-line.rs:0:5
undefined
undefined
exception thrown: RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-line/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-line/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-line/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-line/a.js:4327:7)
    at <anonymous>:wasm-function[116]:0x460e
    at <anonymous>:wasm-function[145]:0x59f4
    at <anonymous>:wasm-function[142]:0x58e2
    at <anonymous>:wasm-function[135]:0x5616
    at <anonymous>:wasm-function[21]:0x67d
    at <anonymous>:wasm-function[20]:0x647,RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-line/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-line/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-line/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-line/a.js:4327:7)
    at <anonymous>:wasm-function[116]:0x460e
    at <anonymous>:wasm-function[145]:0x59f4
    at <anonymous>:wasm-function[142]:0x58e2
    at <anonymous>:wasm-function[135]:0x5616
    at <anonymous>:wasm-function[21]:0x67d
    at <anonymous>:wasm-function[20]:0x647
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-line/a.js:1656:11)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-panic-no-line/a.js:4327:7)
    at <anonymous>:wasm-function[116]:0x460e
    at <anonymous>:wasm-function[145]:0x59f4
    at <anonymous>:wasm-function[142]:0x58e2
    at <anonymous>:wasm-function[135]:0x5616
    at <anonymous>:wasm-function[21]:0x67d
    at <anonymous>:wasm-function[20]:0x647
    at <anonymous>:wasm-function[24]:0x6cb
    at <anonymous>:wasm-function[28]:0x749
------------------------------------------


---- [ui] ui/panic-runtime/want-unwind-got-abort2.rs stdout ----
---- [ui] ui/panic-runtime/want-unwind-got-abort2.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-runtime/want-unwind-got-abort2.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/want-unwind-got-abort2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/want-unwind-got-abort2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/panics/location-detail-unwrap-no-file.rs stdout ----
diff of run.stderr:

1 thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', <redacted>:7:9
+ undefined
+ undefined
+ exception thrown: RuntimeError: abort(undefined) at Error
+     at jsStackTrace ($TEST_BUILD_DIR/panics/location-detail-unwrap-no-file/a.js:1893:17)
+     at stackTrace ($TEST_BUILD_DIR/panics/location-detail-unwrap-no-file/a.js:1910:16)
+     at abort ($TEST_BUILD_DIR/panics/location-detail-unwrap-no-file/a.js:1650:44)
+     at _abort ($TEST_BUILD_DIR/panics/location-detail-unwrap-no-file/a.js:4327:7)
+     at <anonymous>:wasm-function[111]:0x44f3
+     at <anonymous>:wasm-function[140]:0x58d9
+     at <anonymous>:wasm-function[137]:0x57c7
+     at <anonymous>:wasm-function[130]:0x54fb
+     at <anonymous>:wasm-function[121]:0x4bdc
+     at <anonymous>:wasm-function[120]:0x4b64,RuntimeError: abort(undefined) at Error
+     at jsStackTrace ($TEST_BUILD_DIR/panics/location-detail-unwrap-no-file/a.js:1893:17)
+     at stackTrace ($TEST_BUILD_DIR/panics/location-detail-unwrap-no-file/a.js:1910:16)
+     at abort ($TEST_BUILD_DIR/panics/location-detail-unwrap-no-file/a.js:1650:44)
+     at _abort ($TEST_BUILD_DIR/panics/location-detail-unwrap-no-file/a.js:4327:7)
+     at <anonymous>:wasm-function[111]:0x44f3
+     at <anonymous>:wasm-function[140]:0x58d9
+     at <anonymous>:wasm-function[137]:0x57c7
+     at <anonymous>:wasm-function[130]:0x54fb
+     at <anonymous>:wasm-function[121]:0x4bdc
+     at <anonymous>:wasm-function[120]:0x4b64
+     at abort ($TEST_BUILD_DIR/panics/location-detail-unwrap-no-file/a.js:1656:11)
+     at _abort ($TEST_BUILD_DIR/panics/location-detail-unwrap-no-file/a.js:4327:7)
+     at <anonymous>:wasm-function[111]:0x44f3
+     at <anonymous>:wasm-function[140]:0x58d9
+     at <anonymous>:wasm-function[137]:0x57c7
+     at <anonymous>:wasm-function[130]:0x54fb
+     at <anonymous>:wasm-function[121]:0x4bdc
+     at <anonymous>:wasm-function[120]:0x4b64
+     at <anonymous>:wasm-function[129]:0x5122
+     at <anonymous>:wasm-function[152]:0x5e70


The actual run.stderr differed from the expected run.stderr.
The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-unwrap-no-file/location-detail-unwrap-no-file.run.stderr
error: 1 errors occurred comparing run output.
status: exit status: 1
status: exit status: 1
command: "/emsdk-portable/node/latest/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-unwrap-no-file/a.js"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', <redacted>:7:9
undefined
undefined
exception thrown: RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-unwrap-no-file/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-unwrap-no-file/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-unwrap-no-file/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-unwrap-no-file/a.js:4327:7)
    at <anonymous>:wasm-function[111]:0x44f3
    at <anonymous>:wasm-function[140]:0x58d9
    at <anonymous>:wasm-function[137]:0x57c7
    at <anonymous>:wasm-function[130]:0x54fb
    at <anonymous>:wasm-function[121]:0x4bdc
    at <anonymous>:wasm-function[120]:0x4b64,RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-unwrap-no-file/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-unwrap-no-file/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-unwrap-no-file/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-unwrap-no-file/a.js:4327:7)
    at <anonymous>:wasm-function[111]:0x44f3
    at <anonymous>:wasm-function[140]:0x58d9
    at <anonymous>:wasm-function[137]:0x57c7
    at <anonymous>:wasm-function[130]:0x54fb
    at <anonymous>:wasm-function[121]:0x4bdc
    at <anonymous>:wasm-function[120]:0x4b64
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-unwrap-no-file/a.js:1656:11)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/location-detail-unwrap-no-file/a.js:4327:7)
    at <anonymous>:wasm-function[111]:0x44f3
    at <anonymous>:wasm-function[140]:0x58d9
    at <anonymous>:wasm-function[137]:0x57c7
    at <anonymous>:wasm-function[130]:0x54fb
    at <anonymous>:wasm-function[121]:0x4bdc
    at <anonymous>:wasm-function[120]:0x4b64
    at <anonymous>:wasm-function[129]:0x5122
    at <anonymous>:wasm-function[152]:0x5e70
------------------------------------------


---- [ui] ui/privacy/reachable-unnameable-items.rs stdout ----
---- [ui] ui/privacy/reachable-unnameable-items.rs stdout ----

error: test run failed!
status: exit status: 1
command: "/emsdk-portable/node/latest/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/reachable-unnameable-items/a.js"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/src/test/ui/privacy/reachable-unnameable-items.rs:31:51
undefined
undefined
exception thrown: RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/reachable-unnameable-items/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/reachable-unnameable-items/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/reachable-unnameable-items/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/reachable-unnameable-items/a.js:4327:7)
    at <anonymous>:wasm-function[122]:0x48ed
    at <anonymous>:wasm-function[151]:0x5cd6
    at <anonymous>:wasm-function[148]:0x5bc4
    at <anonymous>:wasm-function[141]:0x58f8
    at <anonymous>:wasm-function[132]:0x4fd8
    at <anonymous>:wasm-function[131]:0x4f5f,RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/reachable-unnameable-items/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/reachable-unnameable-items/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/reachable-unnameable-items/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/reachable-unnameable-items/a.js:4327:7)
    at <anonymous>:wasm-function[122]:0x48ed
    at <anonymous>:wasm-function[151]:0x5cd6
    at <anonymous>:wasm-function[148]:0x5bc4
    at <anonymous>:wasm-function[141]:0x58f8
    at <anonymous>:wasm-function[132]:0x4fd8
    at <anonymous>:wasm-function[131]:0x4f5f
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/reachable-unnameable-items/a.js:1656:11)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/reachable-unnameable-items/a.js:4327:7)
    at <anonymous>:wasm-function[122]:0x48ed
    at <anonymous>:wasm-function[151]:0x5cd6
    at <anonymous>:wasm-function[148]:0x5bc4
    at <anonymous>:wasm-function[141]:0x58f8
    at <anonymous>:wasm-function[132]:0x4fd8
    at <anonymous>:wasm-function[131]:0x4f5f
    at <anonymous>:wasm-function[140]:0x551e
    at <anonymous>:wasm-function[162]:0x626b
------------------------------------------


---- [ui] ui/proc-macro/expand-with-a-macro.rs stdout ----
---- [ui] ui/proc-macro/expand-with-a-macro.rs stdout ----

error: test run failed!
status: exit status: 1
command: "/emsdk-portable/node/latest/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/expand-with-a-macro/a.js"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'main' panicked at 'hello', /checkout/src/test/ui/proc-macro/expand-with-a-macro.rs:14:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
undefined
exception thrown: RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/expand-with-a-macro/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/expand-with-a-macro/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/expand-with-a-macro/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/expand-with-a-macro/a.js:4327:7)
    at <anonymous>:wasm-function[116]:0x460e
    at <anonymous>:wasm-function[145]:0x59f4
    at <anonymous>:wasm-function[142]:0x58e2
    at <anonymous>:wasm-function[135]:0x5616
    at <anonymous>:wasm-function[21]:0x67d
    at <anonymous>:wasm-function[20]:0x647,RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/expand-with-a-macro/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/expand-with-a-macro/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/expand-with-a-macro/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/expand-with-a-macro/a.js:4327:7)
    at <anonymous>:wasm-function[116]:0x460e
    at <anonymous>:wasm-function[145]:0x59f4
    at <anonymous>:wasm-function[142]:0x58e2
    at <anonymous>:wasm-function[135]:0x5616
    at <anonymous>:wasm-function[21]:0x67d
    at <anonymous>:wasm-function[20]:0x647
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/expand-with-a-macro/a.js:1656:11)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/expand-with-a-macro/a.js:4327:7)
    at <anonymous>:wasm-function[116]:0x460e
    at <anonymous>:wasm-function[145]:0x59f4
    at <anonymous>:wasm-function[142]:0x58e2
    at <anonymous>:wasm-function[135]:0x5616
    at <anonymous>:wasm-function[21]:0x67d
    at <anonymous>:wasm-function[20]:0x647
    at <anonymous>:wasm-function[24]:0x6cb
    at <anonymous>:wasm-function[28]:0x749
------------------------------------------


---- [ui] ui/rfc-1937-termination-trait/termination-trait-in-test.rs stdout ----
---- [ui] ui/rfc-1937-termination-trait/termination-trait-in-test.rs stdout ----

error: test run failed!
status: exit status: 1
command: "/emsdk-portable/node/latest/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test/a.js"
------------------------------------------

running 3 tests
test is_a_num ... ok
test is_a_num ... ok

------------------------------------------
stderr:
------------------------------------------
undefined
exception thrown: RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test/a.js:1894:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test/a.js:1911:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test/a.js:1651:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test/a.js:4716:7)
    at <anonymous>:wasm-function[585]:0x4e6c4
    at <anonymous>:wasm-function[648]:0x50f1a
    at <anonymous>:wasm-function[627]:0x5021e
    at <anonymous>:wasm-function[620]:0x4ff45
    at <anonymous>:wasm-function[608]:0x4f397
    at <anonymous>:wasm-function[607]:0x4f2fc,RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test/a.js:1894:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test/a.js:1911:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test/a.js:1651:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test/a.js:4716:7)
    at <anonymous>:wasm-function[585]:0x4e6c4
    at <anonymous>:wasm-function[648]:0x50f1a
    at <anonymous>:wasm-function[627]:0x5021e
    at <anonymous>:wasm-function[620]:0x4ff45
    at <anonymous>:wasm-function[608]:0x4f397
    at <anonymous>:wasm-function[607]:0x4f2fc
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test/a.js:1657:11)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test/a.js:4716:7)
    at <anonymous>:wasm-function[585]:0x4e6c4
    at <anonymous>:wasm-function[648]:0x50f1a
    at <anonymous>:wasm-function[627]:0x5021e
    at <anonymous>:wasm-function[620]:0x4ff45
    at <anonymous>:wasm-function[608]:0x4f397
    at <anonymous>:wasm-function[607]:0x4f2fc
    at <anonymous>:wasm-function[619]:0x4fb56
    at <anonymous>:wasm-function[696]:0x57bba
------------------------------------------


---- [ui] ui/rfc-2091-track-caller/std-panic-locations.rs#default stdout ----
---- [ui] ui/rfc-2091-track-caller/std-panic-locations.rs#default stdout ----

error in revision `default`: test run failed!
status: exit status: 1
command: "/emsdk-portable/node/latest/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/std-panic-locations.default/a.js"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
undefined
exception thrown: RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/std-panic-locations.default/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/std-panic-locations.default/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/std-panic-locations.default/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/std-panic-locations.default/a.js:4327:7)
    at <anonymous>:wasm-function[134]:0x575a
    at <anonymous>:wasm-function[164]:0x6cc8
    at <anonymous>:wasm-function[161]:0x6bb4
    at <anonymous>:wasm-function[154]:0x68e6
    at <anonymous>:wasm-function[144]:0x5e4a
    at <anonymous>:wasm-function[143]:0x5dd1,RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/std-panic-locations.default/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/std-panic-locations.default/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/std-panic-locations.default/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/std-panic-locations.default/a.js:4327:7)
    at <anonymous>:wasm-function[134]:0x575a
    at <anonymous>:wasm-function[164]:0x6cc8
    at <anonymous>:wasm-function[161]:0x6bb4
    at <anonymous>:wasm-function[154]:0x68e6
    at <anonymous>:wasm-function[144]:0x5e4a
    at <anonymous>:wasm-function[143]:0x5dd1
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/std-panic-locations.default/a.js:1656:11)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/std-panic-locations.default/a.js:4327:7)
    at <anonymous>:wasm-function[134]:0x575a
    at <anonymous>:wasm-function[164]:0x6cc8
    at <anonymous>:wasm-function[161]:0x6bb4
    at <anonymous>:wasm-function[154]:0x68e6
    at <anonymous>:wasm-function[144]:0x5e4a
    at <anonymous>:wasm-function[143]:0x5dd1
    at <anonymous>:wasm-function[153]:0x650b
    at <anonymous>:wasm-function[176]:0x7264
------------------------------------------


---- [ui] ui/rfc-2091-track-caller/std-panic-locations.rs#mir-opt stdout ----
---- [ui] ui/rfc-2091-track-caller/std-panic-locations.rs#mir-opt stdout ----

error in revision `mir-opt`: test run failed!
status: exit status: 1
command: "/emsdk-portable/node/latest/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/std-panic-locations.mir-opt/a.js"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
undefined
exception thrown: RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/std-panic-locations.mir-opt/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/std-panic-locations.mir-opt/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/std-panic-locations.mir-opt/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/std-panic-locations.mir-opt/a.js:4327:7)
    at <anonymous>:wasm-function[136]:0x57b4
    at <anonymous>:wasm-function[166]:0x6d22
    at <anonymous>:wasm-function[163]:0x6c0e
    at <anonymous>:wasm-function[156]:0x6940
    at <anonymous>:wasm-function[146]:0x5ea4
    at <anonymous>:wasm-function[145]:0x5e2b,RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/std-panic-locations.mir-opt/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/std-panic-locations.mir-opt/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/std-panic-locations.mir-opt/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/std-panic-locations.mir-opt/a.js:4327:7)
    at <anonymous>:wasm-function[136]:0x57b4
    at <anonymous>:wasm-function[166]:0x6d22
    at <anonymous>:wasm-function[163]:0x6c0e
    at <anonymous>:wasm-function[156]:0x6940
    at <anonymous>:wasm-function[146]:0x5ea4
    at <anonymous>:wasm-function[145]:0x5e2b
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/std-panic-locations.mir-opt/a.js:1656:11)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/std-panic-locations.mir-opt/a.js:4327:7)
    at <anonymous>:wasm-function[136]:0x57b4
    at <anonymous>:wasm-function[166]:0x6d22
    at <anonymous>:wasm-function[163]:0x6c0e
    at <anonymous>:wasm-function[156]:0x6940
    at <anonymous>:wasm-function[146]:0x5ea4
    at <anonymous>:wasm-function[145]:0x5e2b
    at <anonymous>:wasm-function[155]:0x6565
    at <anonymous>:wasm-function[178]:0x72c0
------------------------------------------


---- [ui] ui/rfcs/rfc1857-drop-order.rs stdout ----
---- [ui] ui/rfcs/rfc1857-drop-order.rs stdout ----

error: test run failed!
status: exit status: 1
command: "/emsdk-portable/node/latest/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc1857-drop-order/a.js"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'main' panicked at 'this panic is caught :D', /checkout/src/test/ui/rfcs/rfc1857-drop-order.rs:62:10
undefined
undefined
exception thrown: RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc1857-drop-order/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc1857-drop-order/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc1857-drop-order/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc1857-drop-order/a.js:4327:7)
    at <anonymous>:wasm-function[124]:0x4cc1
    at <anonymous>:wasm-function[153]:0x60ac
    at <anonymous>:wasm-function[150]:0x5f9a
    at <anonymous>:wasm-function[143]:0x5cce
    at <anonymous>:wasm-function[21]:0x68e
    at <anonymous>:wasm-function[20]:0x658,RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc1857-drop-order/a.js:1893:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc1857-drop-order/a.js:1910:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc1857-drop-order/a.js:1650:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc1857-drop-order/a.js:4327:7)
    at <anonymous>:wasm-function[124]:0x4cc1
    at <anonymous>:wasm-function[153]:0x60ac
    at <anonymous>:wasm-function[150]:0x5f9a
    at <anonymous>:wasm-function[143]:0x5cce
    at <anonymous>:wasm-function[21]:0x68e
    at <anonymous>:wasm-function[20]:0x658
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc1857-drop-order/a.js:1656:11)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc1857-drop-order/a.js:4327:7)
    at <anonymous>:wasm-function[124]:0x4cc1
    at <anonymous>:wasm-function[153]:0x60ac
    at <anonymous>:wasm-function[150]:0x5f9a
    at <anonymous>:wasm-function[143]:0x5cce
    at <anonymous>:wasm-function[21]:0x68e
    at <anonymous>:wasm-function[20]:0x658
    at <anonymous>:wasm-function[24]:0x6db
    at <anonymous>:wasm-function[37]:0xdd4
------------------------------------------


---- [ui] ui/test-attrs/test-should-fail-good-message.rs stdout ----
---- [ui] ui/test-attrs/test-should-fail-good-message.rs stdout ----

error: test run failed!
status: exit status: 1
command: "/emsdk-portable/node/latest/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-should-fail-good-message/a.js"
------------------------------------------

running 2 tests


------------------------------------------
stderr:
------------------------------------------
undefined
exception thrown: RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-should-fail-good-message/a.js:1894:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-should-fail-good-message/a.js:1911:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-should-fail-good-message/a.js:1651:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-should-fail-good-message/a.js:4716:7)
    at <anonymous>:wasm-function[589]:0x4e5f7
    at <anonymous>:wasm-function[652]:0x50e4d
    at <anonymous>:wasm-function[631]:0x50151
    at <anonymous>:wasm-function[624]:0x4fe78
    at <anonymous>:wasm-function[45]:0xd53
    at <anonymous>:wasm-function[44]:0xd1d,RuntimeError: abort(undefined) at Error
    at jsStackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-should-fail-good-message/a.js:1894:17)
    at stackTrace (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-should-fail-good-message/a.js:1911:16)
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-should-fail-good-message/a.js:1651:44)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-should-fail-good-message/a.js:4716:7)
    at <anonymous>:wasm-function[589]:0x4e5f7
    at <anonymous>:wasm-function[652]:0x50e4d
    at <anonymous>:wasm-function[631]:0x50151
    at <anonymous>:wasm-function[624]:0x4fe78
    at <anonymous>:wasm-function[45]:0xd53
    at <anonymous>:wasm-function[44]:0xd1d
    at abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-should-fail-good-message/a.js:1657:11)
    at _abort (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-should-fail-good-message/a.js:4716:7)
    at <anonymous>:wasm-function[589]:0x4e5f7
    at <anonymous>:wasm-function[652]:0x50e4d
    at <anonymous>:wasm-function[631]:0x50151
    at <anonymous>:wasm-function[624]:0x4fe78
    at <anonymous>:wasm-function[45]:0xd53
    at <anonymous>:wasm-function[44]:0xd1d
    at <anonymous>:wasm-function[48]:0xda1
    at <anonymous>:wasm-function[51]:0xe0b
------------------------------------------


---- [ui] ui/unwind-no-uwtable.rs stdout ----
---- [ui] ui/unwind-no-uwtable.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unwind-no-uwtable.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unwind-no-uwtable/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-C" "panic=unwind" "-C" "force-unwind-tables=n" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unwind-no-uwtable/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: the crate `panic_unwind` does not have the panic strategy `unwind`
error: aborting due to previous error


------------------------------------------
---
test result: FAILED. 11871 passed; 39 failed; 569 ignored; 0 measured; 0 filtered out; finished in 398.73s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-emscripten" "--suite" "ui" "--mode" "ui" "--target" "wasm32-unknown-emscripten" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/emsdk-portable/node/latest/bin/node" "--npm" "/emsdk-portable/node/latest/bin/npm" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.59.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:24:44
