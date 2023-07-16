plain
running 4 tests
FF..
failures:

---- lib.rs - map (line 89) stdout ----
error: cannot find macro `map` in this scope
  |
  |
3 | let letters = map!{"a" => "b", "c" => "d"};

error: aborting due to previous error

Couldn't compile the test.
Couldn't compile the test.
---- lib.rs - map::map (line 89) stdout ----
error: cannot find macro `map` in this scope
  |
  |
3 | let letters = map!{"a" => "b", "c" => "d"};

error: aborting due to previous error

Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"


Build completed unsuccessfully in 0:22:48
