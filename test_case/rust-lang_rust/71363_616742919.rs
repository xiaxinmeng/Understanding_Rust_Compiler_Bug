json
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
    "rendered": "error[E0277]: `MyError` doesn't implement `std::fmt::Display`\n  --> src/main.rs:2:6\n   |\n2  | impl std::error::Error for MyError {}\n   |      ^^^^^^^^^^^^^^^^^ `MyError` cannot be formatted with the default formatter\n   |\n   = help: the trait `std::fmt::Display` is not implemented for `MyError`\n   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead\n\n",
    "children": [
      {
        "children": [],
        "code": null,
        "level": "help",
        "message": "the trait `std::fmt::Display` is not implemented for `MyError`",
        "rendered": null,
        "spans": []
      },
      {
        "children": [],
        "code": null,
        "level": "note",
        "message": "in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead",
        "rendered": null,
        "spans": []
      }
    ],
    "code": {
      "code": "E0277",
      "explanation": "You tried to use a type which doesn't implement some trait in a place which\nexpected that trait.\n\nErroneous code example:\n\n