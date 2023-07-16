json
{
  "crate_version": null,
  "external_crates": {},
  "format_version": 18,
  "includes_private": false,
  "index": {
    "0:0:1570": {
      "attrs": ["#![feature(no_core)]", "#![no_core]"],
      "crate_id": 0,
      "deprecation": null,
      "docs": null,
      "id": "0:0:1570",
      "inner": {"is_crate": true, "is_stripped": false, "items": ["0:1:1565", "0:4:1567", "0:8:1568"]},
      "kind": "module",
      "links": {},
      "name": "single_file",
      "span": {"begin": [1, 0], "end": [12, 53], "filename": "single-file.rs"},
      "visibility": "public"
    },
    "0:1:1565": {
      "attrs": [],
      "crate_id": 0,
      "deprecation": null,
      "docs": null,
      "id": "0:1:1565",
      "inner": {"is_crate": false, "is_stripped": false, "items": ["0:2:1566"]},
      "kind": "module",
      "links": {},
      "name": "http",
      "span": {"begin": [4, 0], "end": [4, 12], "filename": "single-file.rs"},
      "visibility": "public"
    },
    "0:2:1566": {
      "attrs": [],
      "crate_id": 0,
      "deprecation": null,
      "docs": null,
      "id": "0:2:1566",
      "inner": {
        "fields": [],
        "fields_stripped": false,
        "generics": {"params": [], "where_predicates": []},
        "impls": [],
        "struct_type": "unit"
      },
      "kind": "struct",
      "links": {},
      "name": "Request",
      "span": {"begin": [5, 4], "end": [5, 23], "filename": "single-file.rs"},
      "visibility": "public"
    },
    "0:4:1567": {
      "attrs": [],
      "crate_id": 0,
      "deprecation": null,
      "docs": null,
      "id": "0:4:1567",
      "inner": {"is_crate": false, "is_stripped": false, "items": ["0:5"]},
      "kind": "module",
      "links": {},
      "name": "pavex_runtime",
      "span": {"begin": [8, 0], "end": [8, 21], "filename": "single-file.rs"},
      "visibility": "public"
    },
    "0:5": {
      "attrs": [],
      "crate_id": 0,
      "deprecation": null,
      "docs": null,
      "id": "0:5",
      "inner": {"glob": false, "id": "0:1:1565", "name": "http", "source": "crate::http"},
      "kind": "import",
      "links": {},
      "name": null,
      "span": {"begin": [9, 4], "end": [9, 24], "filename": "single-file.rs"},
      "visibility": "public"
    },
    "0:8:1568": {
      "attrs": [],
      "crate_id": 0,
      "deprecation": null,
      "docs": null,
      "id": "0:8:1568",
      "inner": {
        "decl": {
          "c_variadic": false,
          "inputs": [
            [
              "_req",
              {
                "inner": {
                  "args": {"angle_bracketed": {"args": [], "bindings": []}},
                  "id": "0:2:1566",
                  "name": "pavex_runtime::http::Request"
                },
                "kind": "resolved_path"
              }
            ]
          ],
          "output": null
        },
        "generics": {"params": [], "where_predicates": []},
        "header": {"abi": "Rust", "async": false, "const": false, "unsafe": false}
      },
      "kind": "function",
      "links": {},
      "name": "extract",
      "span": {"begin": [12, 0], "end": [12, 53], "filename": "single-file.rs"},
      "visibility": "public"
    }
  },
  "paths": {
    "0:0:1570": {"crate_id": 0, "kind": "module", "path": ["single_file"]},
    "0:1:1565": {"crate_id": 0, "kind": "module", "path": ["single_file", "http"]},
    "0:2:1566": {"crate_id": 0, "kind": "struct", "path": ["single_file", "http", "Request"]},
    "0:4:1567": {"crate_id": 0, "kind": "module", "path": ["single_file", "pavex_runtime"]},
    "0:8:1568": {"crate_id": 0, "kind": "function", "path": ["single_file", "extract"]}
  },
  "root": "0:0:1570"
}
