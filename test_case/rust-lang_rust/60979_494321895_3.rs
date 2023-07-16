json
{
  "reason": "compiler-message",
  "package_id": "test_sugg 0.1.0 (path+file:///home/pkrones/test_sugg)",
  "target": {
    "kind": [
      "bin"
    ],
    "crate_types": [
      "bin"
    ],
    "name": "test_sugg",
    "src_path": "/home/pkrones/test_sugg/src/main.rs",
    "edition": "2018"
  },
  "message": {
    "message": "literal non-ASCII character detected",
    "code": {
      "code": "clippy::non_ascii_literal",
      "explanation": null
    },
    "level": "warning",
    "spans": [
      {
        "file_name": "src/main.rs",
        "byte_start": 80,
        "byte_end": 88,
        "line_start": 4,
        "line_end": 4,
        "column_start": 12,
        "column_end": 19,
        "is_primary": true,
        "text": [
          {
            "text": "    print!(\"Üben!\");",
            "highlight_start": 12,
            "highlight_end": 19
          }
        ],
        "label": null,
        "suggested_replacement": null,
        "suggestion_applicability": null,
        "expansion": {
          "span": {
            "file_name": "<::std::macros::print macros>",
            "byte_start": 54,
            "byte_end": 85,
            "line_start": 2,
            "line_end": 2,
            "column_start": 27,
            "column_end": 58,
            "is_primary": false,
            "text": [
              {
                "text": "$ crate :: io :: _print ( format_args ! ( $ ( $ arg ) * ) ) ) ;",
                "highlight_start": 27,
                "highlight_end": 58
              }
            ],
            "label": null,
            "suggested_replacement": null,
            "suggestion_applicability": null,
            "expansion": {
              "span": {
                "file_name": "src/main.rs",
                "byte_start": 73,
                "byte_end": 90,
                "line_start": 4,
                "line_end": 4,
                "column_start": 5,
                "column_end": 21,
                "is_primary": false,
                "text": [
                  {
                    "text": "    print!(\"Üben!\");",
                    "highlight_start": 5,
                    "highlight_end": 21
                  }
                ],
                "label": null,
                "suggested_replacement": null,
                "suggestion_applicability": null,
                "expansion": null
              },
              "macro_decl_name": "print!",
              "def_site_span": {
                "file_name": "<::std::macros::print macros>",
                "byte_start": 0,
                "byte_end": 91,
                "line_start": 1,
                "line_end": 2,
                "column_start": 1,
                "column_end": 64,
                "is_primary": false,
                "text": [
                  {
                    "text": "( $ ( $ arg : tt ) * ) => (",
                    "highlight_start": 1,
                    "highlight_end": 28
                  },
                  {
                    "text": "$ crate :: io :: _print ( format_args ! ( $ ( $ arg ) * ) ) ) ;",
                    "highlight_start": 1,
                    "highlight_end": 64
                  }
                ],
                "label": null,
                "suggested_replacement": null,
                "suggestion_applicability": null,
                "expansion": null
              }
            }
          },
          "macro_decl_name": "format_args!",
          "def_site_span": null
        }
      }
    ],
    "children": [
      {
        "message": "for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#non_ascii_literal",
        "code": null,
        "level": "help",
        "spans": [],
        "children": [],
        "rendered": null
      }
    ],
    "rendered": "warning: literal non-ASCII character detected\n --> src/main.rs:4:12\n  |\n4 |     print!(\"Üben!\");\n  |            ^^^^^^^\n  |\n  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#non_ascii_literal\n\n"
  }
}
