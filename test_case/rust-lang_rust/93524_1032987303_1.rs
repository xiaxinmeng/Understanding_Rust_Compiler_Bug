json
        "0:4": {
            "id": "0:4",
            "crate_id": 0,
            "name": "C",
            "visibility": "public",
            "kind": "constant",
            "inner": {
                "type": {
                    "kind": "primitive",
                    "inner": "usize"
                },
            }
        },
        "0:5": {
            "id": "0:5",
            "crate_id": 0,
            "name": "a",
            "visibility": "public",
            "kind": "module",
            "inner": {
                "is_crate": false,
                "items": [
                    "0:4"           <--- Note: Contains C
                ]
            }
        },
        "0:9": {
            "id": "0:9",
            "crate_id": 0,
            "name": "b",
            "visibility": "public",
            "kind": "module",
            "inner": {
                "is_crate": false,
                "items": [
                    "0:4"           <--- Note: Also contains C
                ]
            }
        },
