plain
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-scrape-examples-macros/rustdoc-scrape-examples-macros:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-scrape-examples-macros/rustdoc-scrape-examples-macros -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-scrape-examples-macros/rustdoc-scrape-examples-macros  src/proc.rs --crate-name foobar_macro --edition=2021 --crate-type proc-macro --emit=dep-info,link
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-scrape-examples-macros/rustdoc-scrape-examples-macros:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-scrape-examples-macros/rustdoc-scrape-examples-macros -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-scrape-examples-macros/rustdoc-scrape-examples-macros  src/lib.rs --crate-name foobar --edition=2021 --crate-type lib --emit=dep-info,link
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-scrape-examples-macros/rustdoc-scrape-examples-macros:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib examples/ex.rs --crate-name ex --crate-type bin --output "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-scrape-examples-macros/rustdoc-scrape-examples-macros/rustdoc" \
 --extern foobar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-scrape-examples-macros/rustdoc-scrape-examples-macros/libfoobar.rlib --extern foobar_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-scrape-examples-macros/rustdoc-scrape-examples-macros/libfoobar_macro.so \
 -Z unstable-options --scrape-examples-output-path /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-scrape-examples-macros/rustdoc-scrape-examples-macros/ex.calls --scrape-examples-target-crate foobar
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-scrape-examples-macros/rustdoc-scrape-examples-macros:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib src/lib.rs --crate-name foobar --crate-type lib --output "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-scrape-examples-macros/rustdoc-scrape-examples-macros/rustdoc" \
 -Z unstable-options --with-examples /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-scrape-examples-macros/rustdoc-scrape-examples-macros/ex.calls
'/usr/bin/python2.7' '/checkout/src/etc/htmldocck.py' "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-scrape-examples-macros/rustdoc-scrape-examples-macros/rustdoc" src/lib.rs
--- stderr -------------------------------
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 570, in <module>
    check(sys.argv[1], get_commands(rust_test_path))
  File "/checkout/src/etc/htmldocck.py", line 554, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 473, in check_command
    tree = cache.get_tree(c.args[0])
  File "/checkout/src/etc/htmldocck.py", line 342, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'foobar/fn.f.html': u'pr'
make: *** [Makefile:11: all] Error 1



failures:
