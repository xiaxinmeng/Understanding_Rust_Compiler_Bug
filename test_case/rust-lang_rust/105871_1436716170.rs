rust
let byte_offset = get_some_offset();
if let Some(inner) = self {
    assert!(byte_offset == <*const _>::byte_offset_from(inner, self));
}
… then use `byte_offset` to actually compute the return value ...
