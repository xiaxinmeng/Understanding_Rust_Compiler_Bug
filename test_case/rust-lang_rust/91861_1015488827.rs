plain
.................................................................................................... 600/606
......
failures:

---- src/collections/btree/map.rs - collections::btree::map::BTreeMap<K,V>::range_mut (line 1098) stdout ----
error[E0277]: the trait bound `BTreeMap<&str, i32>: From<[&str; 4]>` is not satisfied
  |
  |
6 | let mut map: BTreeMap<&str, i32> = ["Alice", "Bob", "Carol", "Cheryl"].into();
  |                                                                        ^^^^ the trait `From<[&str; 4]>` is not implemented for `BTreeMap<&str, i32>`
  = help: the following implementations were found:
  = help: the following implementations were found:
            <BTreeMap<K, V> as From<[(K, V); N]>>
  = note: required because of the requirements on the impl of `Into<BTreeMap<&str, i32>>` for `[&str; 4]`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


Build completed unsuccessfully in 0:21:07
