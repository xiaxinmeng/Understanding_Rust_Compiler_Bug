swift
let str = "e\u{301}";
// Array of unicode scalars, equivalent to Rust's chars
print("\(Array(str.unicodeScalars))"); // ["e", "\u{0301}"]
// Array of unicode scalars converted into strings
print("\(Array(str.unicodeScalars).map({ String.init($0) }))"); // ["e", "́"]
