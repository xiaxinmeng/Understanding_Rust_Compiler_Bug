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
        "byte_start": 59,
        "byte_end": 67,
        "line_start": 3,
        "line_end": 3,
        "column_start": 13,
        "column_end": 20,
        "is_primary": true,
        "text": [
          {
            "text": "    let _ = \"Üben!\";",
            "highlight_start": 13,
            "highlight_end": 20
          }
        ],
        "label": null,
        "suggested_replacement": null,
        "suggestion_applicability": null,
        "expansion": null
      }
    ],
    "children": [
      {
        "message": "lint level defined here",
        "code": null,
        "level": "note",
        "spans": [
          {
            "file_name": "src/main.rs",
            "byte_start": 7,
            "byte_end": 32,
            "line_start": 1,
            "line_end": 1,
            "column_start": 8,
            "column_end": 33,
            "is_primary": true,
            "text": [
              {
                "text": "#[warn(clippy::non_ascii_literal)]",
                "highlight_start": 8,
                "highlight_end": 33
              }
            ],
            "label": null,
            "suggested_replacement": null,
            "suggestion_applicability": null,
            "expansion": null
          }
        ],
        "children": [],
        "rendered": null
      },
      {
        "message": "for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#non_ascii_literal",
        "code": null,
        "level": "help",
        "spans": [],
        "children": [],
        "rendered": null
      },
      {
        "message": "consider replacing the string with",
        "code": null,
        "level": "help",
        "spans": [
          {
            "file_name": "src/main.rs",
            "byte_start": 59,
            "byte_end": 67,
            "line_start": 3,
            "line_end": 3,
            "column_start": 13,
            "column_end": 20,
            "is_primary": true,
            "text": [
              {
                "text": "    let _ = \"Üben!\";",
                "highlight_start": 13,
                "highlight_end": 20
              }
            ],
            "label": null,
            "suggested_replacement": "\"\\u{dc}ben!\"",
            "suggestion_applicability": "MachineApplicable",
            "expansion": null
          }
        ],
        "children": [],
        "rendered": null
      }
    ],
    "rendered": "warning: literal non-ASCII character detected\n --> src/main.rs:3:13\n  |\n3 |     let _ = \"Üben!\";\n  |             ^^^^^^^ help: consider replacing the string with: `\"\\u{dc}ben!\"`\n  |\nnote: lint level defined here\n --> src/main.rs:1:8\n  |\n1 | #[warn(clippy::non_ascii_literal)]\n  |        ^^^^^^^^^^^^^^^^^^^^^^^^^\n  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#non_ascii_literal\n\n"
  }
}
