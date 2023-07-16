\n"
    },
    "level": "error",
    "spans": [
        {
            "file_name": "ambiguous-display.rs",
            "byte_start": 64,
            "byte_end": 71,
            "line_start": 3,
            "line_end": 3,
            "column_start": 13,
            "column_end": 20,
            "is_primary": true,
            "text": [
                {
                    "text": "    let d: &Display = &xs;",
                    "highlight_start": 13,
                    "highlight_end": 20
                }
            ],
            "label": "not found in this scope",
            "suggested_replacement": null,
            "suggestion_applicability": null,
            "expansion": null
        }
    ],
    "children": [
        {
            "message": "possible candidates are found in other modules, you can import them into scope",
            "code": null,
            "level": "help",
            "spans": [
                {
                    "file_name": "ambiguous-display.rs",
                    "byte_start": 0,
                    "byte_end": 0,
                    "line_start": 1,
                    "line_end": 1,
                    "column_start": 1,
                    "column_end": 1,
                    "is_primary": true,
                    "text": [
                        {
                            "text": "fn main() {",
                            "highlight_start": 1,
                            "highlight_end": 1
                        }
                    ],
                    "label": null,
                    "suggested_replacement": "use std::fmt::Display;\n\n",
                    "suggestion_applicability": "Unspecified",
                    "expansion": null
                }
            ],
            "children": [],
            "rendered": null
        },
        {
            "message": "possible candidates are found in other modules, you can import them into scope",
            "code": null,
            "level": "help",
            "spans": [
                {
                    "file_name": "ambiguous-display.rs",
                    "byte_start": 0,
                    "byte_end": 0,
                    "line_start": 1,
                    "line_end": 1,
                    "column_start": 1,
                    "column_end": 1,
                    "is_primary": true,
                    "text": [
                        {
                            "text": "fn main() {",
                            "highlight_start": 1,
                            "highlight_end": 1
                        }
                    ],
                    "label": null,
                    "suggested_replacement": "use std::path::Display;\n\n",
                    "suggestion_applicability": "Unspecified",
                    "expansion": null
                }
            ],
            "children": [],
            "rendered": null
        }
    ],
    "rendered": "error[E0412]: cannot find type `Display` in this scope\n --> ambiguous-display.rs:3:13\n  |\n3 |     let d: &Display = &xs;\n  |             ^^^^^^^ not found in this scope\nhelp: possible candidates are found in other modules, you can import them into scope\n  |\n1 | use std::fmt::Display;\n  |\n1 | use std::path::Display;\n  |\n\n"
}
