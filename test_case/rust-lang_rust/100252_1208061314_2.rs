json
{
  "index": {
    "0:0:1564": {
      "id": "0:0:1564",
      "inner": {
        "is_crate": true,
        "is_stripped": false,
        "items": ["0:3:1562", "0:4"]
      },
      "kind": "module",
      "name": "clap",
      "visibility": "public"
    },
    "0:3:1562": {
      "id": "0:3:1562",
      "inner": {
        "is_crate": false,
        "is_stripped": false,
        "items": ["0:7:1563"]
      },
      "kind": "module",
      "name": "arg_matches",
      "visibility": "crate"
    },
    "0:4": {
      "id": "0:4",
      "inner": {"glob": false, "id": "0:7:1563", "name": "ArgMatches", "source": "arg_matches::ArgMatches"},
      "kind": "import",
      "name": null,
      "visibility": "public"
    },
    "0:7:1563": {
      "id": "0:7:1563",
      "inner": {
        "fields": [],
        "fields_stripped": false,
        "impls": [
          // Auto
          "a:2:9259:2290-0:7:1563", // RefUnwindSafe
          "a:2:3268:211-0:7:1563",  // Send
          "a:2:9258:2291-0:7:1563", // UnwindSafe
          "a:2:3280:220-0:7:1563",  // Sync
          "a:2:3309:1807-0:7:1563", // Unpin
          // Blanket
          "b:2:2871-0:7:1563", // BorrowMut
          "b:2:4001-0:7:1563", // Any
          "b:2:3197-0:7:1563", // Into
          "b:2:3212-0:7:1563", // TryFrom
          "b:2:3207-0:7:1563", // TryInto
          "b:2:2868-0:7:1563", // Borrow
          "b:2:3201-0:7:1563", // From
          "b:5:787-0:7:1563",  // ToOwned
          // Derives
          "0:9",  // Debug
          "0:11", // Clone
          "0:13", // Default
          "0:15", // StructuralPartialEq
          "0:16", // PartialEq
          "0:18", // StructuralEq
          "0:19"  // Eq
        ],
        "struct_type": "tuple"
      },
      "kind": "struct",
      "name": "ArgMatches",
      "visibility": "public"
    }
  },
  "root": "0:0:1564"
}
