json
{
  "crate_version": "0.1.0",
  "external_crates": {
    "1": {"html_root_url": null, "name": "pavex_runtime"},
    "2": {"html_root_url": null, "name": "http"}
  },
  "format_version": 18,
  "includes_private": false,
  "index": {
    "0:0:1570": {
      "crate_id": 0,
      "id": "0:0:1570",
      "inner": {"is_crate": true, "is_stripped": false, "items": ["0:1:1565"]},
      "kind": "module",
      "name": "rust_101059"
    },
    "0:1:1565": {
      "crate_id": 0,
      "id": "0:1:1565",
      "inner": {
        "decl": {
          "inputs": [
            [
              "_req",
              {
                "inner": {
                  "id": "2:3:1569",
                  "name": "pavex_runtime::http::Request"
                },
                "kind": "resolved_path"
              }
            ]
          ],
          "output": null
        }
      },
      "kind": "function",
      "name": "extract"
    }
  },
  "paths": {
    "0:0:1570": {"crate_id": 0, "kind": "module", "path": ["rust_101059"]},
    "0:1:1565": {"crate_id": 0, "kind": "function", "path": ["rust_101059", "extract"]},
    "1:0:1567": {"crate_id": 1, "kind": "module", "path": ["pavex_runtime"]},
    "2:0:1568": {"crate_id": 2, "kind": "module", "path": ["http"]},
    "2:3:1569": {"crate_id": 2, "kind": "struct", "path": ["http", "Request"]}
  },
  "root": "0:0:1570"
}
