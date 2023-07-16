json
{
  "index": {
    "0:0": {
      "inner": {
        "is_crate": true,
        "items": [
          "0:1",
          "0:3",
          "0:5"
        ]
      },
      "kind": "module"
    },
    "0:1": {
      // Struct A
      "inner": {
        "impls": [
          "0:4"
        ],
        "struct_type": "unit"
      },
      "kind": "struct",
      "name": "A"
    },
    "0:3": {
      // Trait T
      "inner": {
        "implementors": [
          "0:4"
        ]
      },
      "kind": "trait",
      "name": "T"
    },
    "0:4": {
      // impl T for A
      "inner": {
        "for": {
          "inner": {
            "args": {
              "angle_bracketed": {
                "args": [],
                "bindings": []
              }
            },
            "id": "0:1",
            "name": "A",
            "param_names": []
          },
          "kind": "resolved_path"
        },
        "trait": {
          "inner": {
            "id": "0:3",
            "name": "T"
          },
          "kind": "resolved_path"
        }
      },
      "kind": "impl"
    },
    "0:5": {
      "id": "0:5",
      "inner": {
        // type B = A
        "type": {
          "inner": {
            "id": "0:1",
            "name": "A"
          },
          "kind": "resolved_path"
        }
      },
      "kind": "typedef",
      "name": "B"
    }
  },
  "root": "0:0"
}
