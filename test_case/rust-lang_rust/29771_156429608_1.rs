
michael@daschlbook ~/rust/repro/src $ rustc --version
rustc 1.4.0 (8ab8581f6 2015-10-27)
michael@daschlbook ~/rust/repro/src $ rustdoc --test lib.rs 

running 1 test
test MySimpleEnum::SomeValue_0 ... FAILED

failures:

---- MySimpleEnum::SomeValue_0 stdout ----
    <anon>:2:10: 2:12 error: expected one of `!`, `.`, `::`, `;`, `{`, `}`, or an operator, found `is`
<anon>:2     here is some text
                  ^~
thread 'MySimpleEnum::SomeValue_0' panicked at 'Box<Any>', ../src/libsyntax/parse/mod.rs:98



failures:
    MySimpleEnum::SomeValue_0

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured
