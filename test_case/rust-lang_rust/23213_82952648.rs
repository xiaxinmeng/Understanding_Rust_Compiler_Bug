
----- /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/src/test/run-make/rustdoc-src-links/ --------------------
------ stdout ---------------------------------------------
DYLD_LIBRARY_PATH="/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/obj/x86_64-apple-darwin/test/run-make/rustdoc-src-links:/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/obj/x86_64-apple-darwin/stage2/lib:" /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/obj/x86_64-apple-darwin/stage2/bin/rustdoc -w html -o /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/obj/x86_64-apple-darwin/test/run-make/rustdoc-src-links/doc foo.rs
/System/Library/Frameworks/Python.framework/Versions/2.7/Resources/Python.app/Contents/MacOS/Python /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/src/etc/htmldocck.py /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/obj/x86_64-apple-darwin/test/run-make/rustdoc-src-links/doc foo.rs

------ stderr ---------------------------------------------
Traceback (most recent call last):
  File "/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/src/etc/htmldocck.py", line 396, in <module>
    check(sys.argv[1], get_commands(sys.argv[2]))
  File "/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-nopt-t/build/src/etc/htmldocck.py", line 389, in check
    c.cmd, c.lineno))
RuntimeError: @has check failed at line 42
make[1]: *** [all] Error 1
