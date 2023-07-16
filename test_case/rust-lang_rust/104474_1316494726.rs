plain

error: expect test failed
   --> crates/proc-macro-srv/src/tests/mod.rs:19:9

You can update all `expect!` tests by running:

    env UPDATE_EXPECT=1 cargo test

To update a single test, place the cursor on `expect` token and use `run` feature of rust-analyzer.
Expect:
----
SUBTREE $
  IDENT   compile_error 4294967295
  IDENT   compile_error 4294967295
  PUNCH   ! [joint] 4294967295
  SUBTREE () 4294967295
    LITERAL "#[derive(DeriveError)] struct S ;" 4294967295
  PUNCH   ; [alone] 4294967295

Actual:
----
SUBTREE $
SUBTREE $
  IDENT   compile_error 4294967295
  PUNCH   ! [alone] 4294967295
  SUBTREE () 4294967295
    LITERAL "#[derive(DeriveError)] struct S ;" 4294967295
  PUNCH   ; [alone] 4294967295

Diff:
----
SUBTREE $
SUBTREE $
  IDENT   compile_error 4294967295
  PUNCH   ! [jointalone] 4294967295
  SUBTREE () 4294967295
    LITERAL "#[derive(DeriveError)] struct S ;" 4294967295
  PUNCH   ; [alone] 4294967295


---- tests::test_attr_macro stdout ----

---
Expect:
----
SUBTREE $
  IDENT   compile_error 4294967295
  PUNCH   ! [joint] 4294967295
  SUBTREE () 4294967295
    LITERAL "#[attr_error(some arguments)] mod m {}" 4294967295
  PUNCH   ; [alone] 4294967295

Actual:
----
SUBTREE $
SUBTREE $
  IDENT   compile_error 4294967295
  PUNCH   ! [alone] 4294967295
  SUBTREE () 4294967295
    LITERAL "#[attr_error(some arguments)] mod m {}" 4294967295
  PUNCH   ; [alone] 4294967295

Diff:
----
SUBTREE $
SUBTREE $
  IDENT   compile_error 4294967295
  PUNCH   ! [jointalone] 4294967295
  SUBTREE () 4294967295
    LITERAL "#[attr_error(some arguments)] mod m {}" 4294967295
  PUNCH   ; [alone] 4294967295


---- tests::test_fn_like_macro_clone_literals stdout ----



error: expect test failed
   --> crates/proc-macro-srv/src/tests/mod.rs:106:9

Expect:
----
SUBTREE $
  LITERAL 1u16 4294967295
  PUNCH   , [alone] 4294967295
  LITERAL 2_u32 4294967295
  PUNCH   , [alone] 4294967295
  PUNCH   - [joint] 4294967295
  LITERAL 4i64 4294967295
  PUNCH   , [alone] 4294967295
  LITERAL 3.14f32 4294967295
  PUNCH   , [alone] 4294967295
  LITERAL "hello bridge" 4294967295

Actual:
----
SUBTREE $
SUBTREE $
  LITERAL 1u16 4294967295
  PUNCH   , [alone] 4294967295
  LITERAL 2_u32 4294967295
  PUNCH   , [alone] 4294967295
  PUNCH   - [alone] 4294967295
  LITERAL 4i64 4294967295
  PUNCH   , [alone] 4294967295
  LITERAL 3.14f32 4294967295
  PUNCH   , [alone] 4294967295
  LITERAL "hello bridge" 4294967295

Diff:
----
SUBTREE $
SUBTREE $
  LITERAL 1u16 4294967295
  PUNCH   , [alone] 4294967295
  LITERAL 2_u32 4294967295
  PUNCH   , [alone] 4294967295
  PUNCH   - [jointalone] 4294967295
  LITERAL 4i64 4294967295
  PUNCH   , [alone] 4294967295
  LITERAL 3.14f32 4294967295
  PUNCH   , [alone] 4294967295
  LITERAL "hello bridge" 4294967295



failures:
