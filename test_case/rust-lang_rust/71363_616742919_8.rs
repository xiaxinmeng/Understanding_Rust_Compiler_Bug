\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"
    },
    "level": "error",
    "message": "`MyError` doesn't implement `std::fmt::Debug`",
    "spans": [
      {
        "byte_end": 38,
        "byte_start": 21,
        "column_end": 23,
        "column_start": 6,
        "expansion": null,
        "file_name": "src/main.rs",
        "is_primary": true,
        "label": "`MyError` cannot be formatted using `{:?}`",
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
        "byte_end": 1875,
        "byte_start": 1870,
        "column_end": 23,
        "column_start": 18,
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
