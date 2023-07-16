rust
/// Owned wrapper for counter value.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum CounterValue {
    Dword(DWORD),
    Large(ULONGLONG),
    TextUnicode(U16CString),
    TextAscii(String),
    Zero,
}

/// Borrowed wrapper for counter value.
/// It is to the `CounterValue` as a str is to the String.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum CounterVal<'a> {
    Dword(DWORD),
    Large(ULONGLONG),
    TextUnicode(&'a U16CStr),
    TextAscii(&'a str),
    Zero,
}
