
let bytes: &[u8] = ...;
let os_string = OsString::from_platform_bytes(bytes).unwrap_or_else(|| {
    OsString::from(String::from_utf8_lossy(bytes))
});
