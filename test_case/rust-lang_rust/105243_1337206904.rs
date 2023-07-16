plain

---- [ui] src/test/ui/span/send-is-not-static-std-sync.rs stdout ----
diff of stderr:

62    |     - `z` dropped here while still borrowed
64 LL | }
64 LL | }
-    | - borrow might be used here, when `tx` is dropped and runs the `Drop` code for type `Sender`
+    | - borrow might be used here, when `tx` is dropped and runs the destructor for type `Sender<&i32>`
66    |
67    = note: values in a scope are dropped in the opposite order they are defined


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/send-is-not-static-std-sync/send-is-not-static-std-sync.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/send-is-not-static-std-sync/send-is-not-static-std-sync.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args span/send-is-not-static-std-sync.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/send-is-not-static-std-sync.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/send-is-not-static-std-sync" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/send-is-not-static-std-sync/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0505]: cannot move out of `y` because it is borrowed
   |
   |
LL |     *lock.lock().unwrap() = &*y;
   |                             --- borrow of `*y` occurs here
LL |     drop(y); //~ ERROR cannot move out
   |          ^ move out of `y` occurs here
...
LL |         *lock.lock().unwrap() = &z;


error[E0597]: `z` does not live long enough
   |
   |
LL |         *lock.lock().unwrap() = &z;
   |                                 ^^ borrowed value does not live long enough
LL |     }
   |     - `z` dropped here while still borrowed
LL |     //~^^ ERROR `z` does not live long enough
LL |     lock.use_ref(); // (Mutex is #[may_dangle] so its dtor does not use `z` => needs explicit use)


error[E0505]: cannot move out of `y` because it is borrowed
   |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL |     *lock.write().unwrap() = &*y;
   |                              --- borrow of `*y` occurs here
LL |     drop(y); //~ ERROR cannot move out
   |          ^ move out of `y` occurs here
...
LL |         *lock.write().unwrap() = &z;


error[E0597]: `z` does not live long enough
   |
   |
LL |         *lock.write().unwrap() = &z;
   |                                  ^^ borrowed value does not live long enough
LL |     }
   |     - `z` dropped here while still borrowed
LL |     //~^^ ERROR `z` does not live long enough
LL |     lock.use_ref(); // (RwLock is #[may_dangle] so its dtor does not use `z` => needs explicit use)


error[E0505]: cannot move out of `y` because it is borrowed
   |
   |
LL |     tx.send(&*y);
   |             --- borrow of `*y` occurs here
LL |     drop(y); //~ ERROR cannot move out
   |          ^ move out of `y` occurs here
...
LL |         tx.send(&z).unwrap();


error[E0597]: `z` does not live long enough
   |
   |
LL |         tx.send(&z).unwrap();
   |                 ^^ borrowed value does not live long enough
LL |     }
   |     - `z` dropped here while still borrowed
LL | }
LL | }
   | - borrow might be used here, when `tx` is dropped and runs the destructor for type `Sender<&i32>`
   |
   = note: values in a scope are dropped in the opposite order they are defined
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0505, E0597.
For more information about an error, try `rustc --explain E0505`.
