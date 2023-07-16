rust
const _ASSERT_FILE_IS_SEND: () = { struct S<T: Send>(Option<T>); S::<File>(None); () };
