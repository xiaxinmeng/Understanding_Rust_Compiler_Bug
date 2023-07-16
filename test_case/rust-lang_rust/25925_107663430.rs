
---- Enumerate_1 stdout ----
    <anon>:2:31: 2:36 error: unresolved name `lines`
<anon>:2     for (linenumber, line) in lines.enumerate() {
                                       ^~~~~
note: in expansion of for loop expansion
<anon>:2:5: 4:6 note: expansion site
error: aborting due to previous error
thread 'Enumerate_1' panicked at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/libsyntax/diagnostic.rs:211
