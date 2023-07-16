
unexpected errors (from JSON output): [
    Error {
        line_num: 34,
        kind: Some(
            Note
        ),
        msg: "34:30: 34:37: a collection of type `std::option::Option<std::collections::Vec<u8>>` cannot be built from an iterator over elements of type `&u8`"
    }
]
not found errors (from test file): [
    Error {
        line_num: 34,
        kind: Some(
            Note
        ),
        msg: "a collection of type `std::option::Option<std::vec::Vec<u8>>` cannot be built from an iterator over elements of type `&u8`"
    }
]
