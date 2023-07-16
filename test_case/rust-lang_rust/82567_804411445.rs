json
{
  "crate_version": null,
  "external_crates": {},
  "format_version": 4,
  "includes_private": false,
  "index": {
    "0:0": {
      "attrs": ["#![no_core]", "#![feature(no_core)]"],
      "crate_id": 0,
      "deprecation": null,
      "docs": null,
      "id": "0:0",
      "inner": {"is_crate": true, "items": ["0:1", "0:4"]},
      "kind": "module",
      "links": {},
      "name": "simple_public",
      "source": {"begin": [3, 0], "end": [18, 22], "filename": "simple_public.rs"},
      "visibility": "public"
    },
    "0:1": {
      "attrs": [],
      "crate_id": 0,
      "deprecation": null,
      "docs": null,
      "id": "0:1",
      "inner": {"is_crate": false, "items": ["0:2"]},
      "kind": "module",
      "links": {},
      "name": "inner",
      "source": {"begin": [8, 0], "end": [13, 1], "filename": "simple_public.rs"},
      "visibility": "public"
    },
    "0:2": {
      "attrs": [],
      "crate_id": 0,
      "deprecation": null,
      "docs": null,
      "id": "0:2",
      "inner": {
        "fields": [],
        "fields_stripped": false,
        "generics": {"params": [], "where_predicates": []},
        "impls": [],
        "struct_type": "unit"
      },
      "kind": "struct",
      "links": {},
      "name": "Public",
      "source": {"begin": [12, 4], "end": [12, 22], "filename": "simple_public.rs"},
      "visibility": "public"
    },
    "0:4": {
      "attrs": [],
      "crate_id": 0,
      "deprecation": null,
      "docs": null,
      "id": "0:4",
      "inner": {"glob": false, "id": "0:2", "name": "Public", "span": "inner::Public"},
      "kind": "import",
      "links": {},
      "name": null,
      "source": {"begin": [18, 0], "end": [18, 22], "filename": "simple_public.rs"},
      "visibility": "public"
    }
  },
  "paths": {
    "0:0": {"crate_id": 0, "kind": "module", "path": ["simple_public"]},
    "0:1": {"crate_id": 0, "kind": "module", "path": ["simple_public", "inner"]},
    "0:2": {
      "crate_id": 0,
      "kind": "struct",
      "path": ["simple_public", "inner", "Public"]
    }
  },
  "root": "0:0"
}
