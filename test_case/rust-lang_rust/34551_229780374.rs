 bash
actual errors (from JSON output): [
    Error {
        line_num: 18,
        kind: Some(
            Error
        ),
        msg: "18:42: 18:48: unknown meta item \'reason\' [E0541]"
    }
   // and every other errors
]

expected errors (from test file): [
    Error {
        line_num: 18,
        kind: Some(
            Error
        ),
        msg: "unknown meta item \'reason\'"
     }
     // and every other expected errors
]
