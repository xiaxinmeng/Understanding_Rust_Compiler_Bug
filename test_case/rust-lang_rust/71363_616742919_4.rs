\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"
    },
    "level": "error",
    "message": "`MyError` doesn't implement `std::fmt::Display`",
    "spans": [
      {
        "byte_end": 38,
        "byte_start": 21,
        "column_end": 23,
        "column_start": 6,
        "expansion": null,
        "file_name": "src/main.rs",
        "is_primary": true,
        "label": "`MyError` cannot be formatted with the default formatter",
        "line_end": 2,
        "line_start": 2,
        "suggested_replacement": null,
        "suggestion_applicability": null,
        "text": [
          {
            "highlight_end": 23,
            "highlight_start": 6,
            "text": "impl std::error::Error for MyError {}"
          }
        ]
      },
      {
        "byte_end": 1885,
        "byte_start": 1878,
        "column_end": 33,
        "column_start": 26,
        "expansion": null,
        "file_name": "/rustc/dbf8b6bf116c7bece2987ff4bd2792f008a6ee77/src/libstd/error.rs",
        "is_primary": false,
        "label": "required by this bound in `std::error::Error`",
        "line_end": 47,
        "line_start": 47,
        "suggested_replacement": null,
        "suggestion_applicability": null,
        "text": []
      }
    ]
  }
}
{
  "reason": "compiler-message",
  "package_id": "testing 0.0.0 (path+file:///git/testing)",
  "target": {
    "kind": [
      "bin"
    ],
    "crate_types": [
      "bin"
    ],
    "name": "testing",
    "src_path": "/git/testing/src/main.rs",
    "edition": "2018",
    "doctest": false
  },
  "message": {
    "rendered": "error[E0277]: `MyError` doesn't implement `std::fmt::Debug`\n  --> src/main.rs:2:6\n   |\n2  | impl std::error::Error for MyError {}\n   |      ^^^^^^^^^^^^^^^^^ `MyError` cannot be formatted using `{:?}`\n   |\n   = help: the trait `std::fmt::Debug` is not implemented for `MyError`\n   = note: add `#[derive(Debug)]` or manually implement `std::fmt::Debug`\n\n",
    "children": [
      {
        "children": [],
        "code": null,
        "level": "help",
        "message": "the trait `std::fmt::Debug` is not implemented for `MyError`",
        "rendered": null,
        "spans": []
      },
      {
        "children": [],
        "code": null,
        "level": "note",
        "message": "add `#[derive(Debug)]` or manually implement `std::fmt::Debug`",
        "rendered": null,
        "spans": []
      }
    ],
    "code": {
      "code": "E0277",
      "explanation": "You tried to use a type which doesn't implement some trait in a place which\nexpected that trait.\n\nErroneous code example:\n\n