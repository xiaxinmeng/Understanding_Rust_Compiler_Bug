
failures:

---- [rustdoc] rustdoc/impl-parts.rs stdout ----

error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python2.7" "/home/ws/ca8159/Projects/rust/rust/src/etc/htmldocck.py" "x86_64-unknown-linux-gnu/test/rustdoc/impl-parts.stage1-x86_64-unknown-linux-gnu" "/home/ws/ca8159/Projects/rust/rust/src/test/rustdoc/impl-parts.rs"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
19: @has check failed
    `XPATH PATTERN` did not match
    // @has impl_parts/struct.Foo.html '//*[@class="impl"]//code' "impl<T: Clone> !AnOibit for Foo<T> where T: Sync"
21: @has check failed
    `XPATH PATTERN` did not match
    // @has impl_parts/trait.AnOibit.html '//*[@class="item-list"]//code' "impl<T: Clone> !AnOibit for Foo<T> where T: Sync"

Encountered 2 errors

------------------------------------------

thread '[rustdoc] rustdoc/impl-parts.rs' panicked at 'explicit panic', /home/ws/ca8159/Projects/rust/rust/src/tools/compiletest/src/runtest.rs:2231
note: Run with `RUST_BACKTRACE=1` for a backtrace.

---- [rustdoc] rustdoc/issue-20727-2.rs stdout ----

error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python2.7" "/home/ws/ca8159/Projects/rust/rust/src/etc/htmldocck.py" "x86_64-unknown-linux-gnu/test/rustdoc/issue-20727-2.stage1-x86_64-unknown-linux-gnu" "/home/ws/ca8159/Projects/rust/rust/src/test/rustdoc/issue-20727-2.rs"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
18: @has check failed
    `XPATH PATTERN` did not match
        // @has - '//*[@class="rust trait"]' 'trait Add<RHS = Self> {'
28: @has check failed
    `XPATH PATTERN` did not match
        // @has - '//*[@class="rust trait"]' 'trait Add<RHS = Self> {'

Encountered 2 errors

------------------------------------------

thread '[rustdoc] rustdoc/issue-20727-2.rs' panicked at 'explicit panic', /home/ws/ca8159/Projects/rust/rust/src/tools/compiletest/src/runtest.rs:2231

---- [rustdoc] rustdoc/issue-20727-4.rs stdout ----

error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python2.7" "/home/ws/ca8159/Projects/rust/rust/src/etc/htmldocck.py" "x86_64-unknown-linux-gnu/test/rustdoc/issue-20727-4.stage1-x86_64-unknown-linux-gnu" "/home/ws/ca8159/Projects/rust/rust/src/test/rustdoc/issue-20727-4.rs"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
18: @has check failed
    `XPATH PATTERN` did not match
        // @has - '//*[@class="rust trait"]' 'trait Index<Idx: ?Sized> {'
29: @has check failed
    `XPATH PATTERN` did not match
        // @has - '//*[@class="rust trait"]' 'trait IndexMut<Idx: ?Sized>: Index<Idx> {'

Encountered 2 errors

------------------------------------------

thread '[rustdoc] rustdoc/issue-20727-4.rs' panicked at 'explicit panic', /home/ws/ca8159/Projects/rust/rust/src/tools/compiletest/src/runtest.rs:2231


failures:
    [rustdoc] rustdoc/impl-parts.rs
    [rustdoc] rustdoc/issue-20727-2.rs
    [rustdoc] rustdoc/issue-20727-4.rs

test result: FAILED. 99 passed; 3 failed; 1 ignored; 0 measured

