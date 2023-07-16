json
{
  "crate_version": null,
  "external_crates": {},
  "format_version": 18,
  "includes_private": false,
  "index": {
    "0:0:1570": {
      "crate_id": 0,
      "id": "0:0:1570",
      "inner": {"is_crate": true, "is_stripped": false, "items": ["0:1:1565", "0:4:1567", "0:8:1568"]},
      "kind": "module",
      "name": "single_file_mod"
    },
    "0:1:1565": {
      "crate_id": 0,
      "id": "0:1:1565",
      "inner": {"is_crate": false, "is_stripped": false, "items": ["0:2:1566"]},
      "kind": "module",
      "name": "http"
    },
    "0:2:1566": {
      "crate_id": 0,
      "id": "0:2:1566",
      "inner": {"struct_type": "unit"},
      "kind": "struct",
      "name": "Request"
    },
    "0:4:1567": {
      "crate_id": 0,
      "id": "0:4:1567",
      "inner": {"is_crate": false, "is_stripped": false, "items": ["0:5"]},
      "kind": "module",
      "name": "pavex_runtime"
    },
    "0:5": {
      "crate_id": 0,
      "id": "0:5",
      "inner": {"glob": false, "id": "0:2:1566", "name": "Request", "source": "crate::http::Request"},
      "kind": "import",
      "name": null
    },
    "0:8:1568": {
      "crate_id": 0,
      "id": "0:8:1568",
      "inner": {
        "decl": {
          "c_variadic": false,
          "inputs": [
            [
              "_req",
              {
                "inner": {"id": "0:2:1566", "name": "pavex_runtime::Request"},
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
  "root": "0:0:1570"
}
