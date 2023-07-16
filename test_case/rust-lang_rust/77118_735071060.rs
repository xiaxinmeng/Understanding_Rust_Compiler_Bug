text
error: This stability annotation is useless
   --> library/alloc/src/string.rs:280:75
    |
280 | pub struct String<#[unstable(feature = "allocator_api", issue = "32838")] A: AllocRef = Global> {
    |                                                                           ^

error: This stability annotation is useless
   --> library/alloc/src/string.rs:319:82
    |
319 | pub struct FromUtf8Error<#[unstable(feature = "allocator_api", issue = "32838")] A: AllocRef = Global>  {
    |             
