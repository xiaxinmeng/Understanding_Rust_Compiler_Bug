
failures:
    [run-pass] ./src/test/run-pass/triv-cast-be.rs
    [run-pass] ./src/test/run-pass/triv-cast-const.rs

$ rustc src/test/run-pass/triv-cast-be.rs

src/test/run-pass/triv-cast-be.rs:5:26: 5:26 error: unresolved typename: m_float
src/test/run-pass/triv-cast-be.rs:5 fn foo_float() -> m_float { ret 0.0 as m_float; }
                                                              ^
src/test/run-pass/triv-cast-be.rs:8:22: 8:22 error: unresolved typename: m_int
src/test/run-pass/triv-cast-be.rs:8 fn foo_int() -> m_int { ret 0 as m_int; }
                                                          ^
src/test/run-pass/triv-cast-be.rs:11:24: 11:24 error: unresolved typename: m_uint
src/test/run-pass/triv-cast-be.rs:11 fn foo_uint() -> m_uint { ret 0u as m_uint; }
