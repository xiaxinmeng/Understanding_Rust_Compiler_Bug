json
{
    "root": "0:0",
    "crate_version": null,
    "includes_private": false,
    "index": {
        "0:6": {
            "id": "0:6",
            "crate_id": 0,
            "name": "B",
            "span": {
                "filename": "lib.rs",
                "begin": [
                    11,
                    0
                ],
                "end": [
                    11,
                    15
                ]
            },
            "visibility": "public",
            "docs": null,
            "links": {},
            "attrs": [],
            "deprecation": null,
            "kind": "typedef",
            "inner": {
                "type": {
                    "kind": "resolved_path",
                    "inner": {
                        "name": "A",
                        "id": "0:1",
                        "args": {
                            "angle_bracketed": {
                                "args": [],
                                "bindings": []
                            }
                        },
                        "param_names": []
                    }
                },
                "generics": {
                    "params": [],
                    "where_predicates": []
                }
            }
        },
        "0:7": {
            "id": "0:7",
            "crate_id": 0,
            "name": null,
            "span": {
                "filename": "lib.rs",
                "begin": [
                    13,
                    0
                ],
                "end": [
                    13,
                    15
                ]
            },
            "visibility": "crate",
            "docs": null,
            "links": {},
            "attrs": [],
            "deprecation": null,
            "kind": "impl",
            "inner": {
                "is_unsafe": false,
                "generics": {
                    "params": [],
                    "where_predicates": []
                },
                "provided_trait_methods": [],
                "trait": {
                    "kind": "resolved_path",
                    "inner": {
                        "name": "T2",
                        "id": "0:4",
                        "args": {
                            "angle_bracketed": {
                                "args": [],
                                "bindings": []
                            }
                        },
                        "param_names": []
                    }
                },
                "for": {
                    "kind": "resolved_path",
                    "inner": {
                        "name": "B",
                        "id": "0:6",
                        "args": {
                            "angle_bracketed": {
                                "args": [],
                                "bindings": []
                            }
                        },
                        "param_names": []
                    }
                },
                "items": [],
                "negative": false,
                "synthetic": false,
                "blanket_impl": null
            }
        },
        "0:4": {
            "id": "0:4",
            "crate_id": 0,
            "name": "T2",
            "span": {
                "filename": "lib.rs",
                "begin": [
                    7,
                    0
                ],
                "end": [
                    7,
                    14
                ]
            },
            "visibility": "public",
            "docs": null,
            "links": {},
            "attrs": [],
            "deprecation": null,
            "kind": "trait",
            "inner": {
                "is_auto": false,
                "is_unsafe": false,
                "items": [],
                "generics": {
                    "params": [],
                    "where_predicates": []
                },
                "bounds": [],
                "implementors": [
                    "0:7"
                ]
            }
        },
        "0:0": {
            "id": "0:0",
            "crate_id": 0,
            "name": "lib",
            "span": {
                "filename": "lib.rs",
                "begin": [
                    1,
                    0
                ],
                "end": [
                    13,
                    15
                ]
            },
            "visibility": "public",
            "docs": null,
            "links": {},
            "attrs": [
                "#![no_core]",
                "#![feature(no_core)]"
            ],
            "deprecation": null,
            "kind": "module",
            "inner": {
                "is_crate": true,
                "items": [
                    "0:1",
                    "0:3",
                    "0:4",
                    "0:6"
                ]
            }
        },
        "0:3": {
            "id": "0:3",
            "crate_id": 0,
            "name": "T1",
            "span": {
                "filename": "lib.rs",
                "begin": [
                    6,
                    0
                ],
                "end": [
                    6,
                    14
                ]
            },
            "visibility": "public",
            "docs": null,
            "links": {},
            "attrs": [],
            "deprecation": null,
            "kind": "trait",
            "inner": {
                "is_auto": false,
                "is_unsafe": false,
                "items": [],
                "generics": {
                    "params": [],
                    "where_predicates": []
                },
                "bounds": [],
                "implementors": [
                    "0:5"
                ]
            }
        },
        "0:5": {
            "id": "0:5",
            "crate_id": 0,
            "name": null,
            "span": {
                "filename": "lib.rs",
                "begin": [
                    9,
                    0
                ],
                "end": [
                    9,
                    15
                ]
            },
            "visibility": "crate",
            "docs": null,
            "links": {},
            "attrs": [],
            "deprecation": null,
            "kind": "impl",
            "inner": {
                "is_unsafe": false,
                "generics": {
                    "params": [],
                    "where_predicates": []
                },
                "provided_trait_methods": [],
                "trait": {
                    "kind": "resolved_path",
                    "inner": {
                        "name": "T1",
                        "id": "0:3",
                        "args": {
                            "angle_bracketed": {
                                "args": [],
                                "bindings": []
                            }
                        },
                        "param_names": []
                    }
                },
                "for": {
                    "kind": "resolved_path",
                    "inner": {
                        "name": "A",
                        "id": "0:1",
                        "args": {
                            "angle_bracketed": {
                                "args": [],
                                "bindings": []
                            }
                        },
                        "param_names": []
                    }
                },
                "items": [],
                "negative": false,
                "synthetic": false,
                "blanket_impl": null
            }
        },
        "0:1": {
            "id": "0:1",
            "crate_id": 0,
            "name": "A",
            "span": {
                "filename": "lib.rs",
                "begin": [
                    4,
                    0
                ],
                "end": [
                    4,
                    13
                ]
            },
            "visibility": "public",
            "docs": null,
            "links": {},
            "attrs": [],
            "deprecation": null,
            "kind": "struct",
            "inner": {
                "struct_type": "unit",
                "generics": {
                    "params": [],
                    "where_predicates": []
                },
                "fields_stripped": false,
                "fields": [],
                "impls": [
                    "0:5"
                ]
            }
        }
    },
    "paths": {
        "0:0": {
            "crate_id": 0,
            "path": [
                "lib"
            ],
            "kind": "module"
        },
        "0:1": {
            "crate_id": 0,
            "path": [
                "lib",
                "A"
            ],
            "kind": "struct"
        },
        "0:3": {
            "crate_id": 0,
            "path": [
                "lib",
                "T1"
            ],
            "kind": "trait"
        },
        "0:4": {
            "crate_id": 0,
            "path": [
                "lib",
                "T2"
            ],
            "kind": "trait"
        },
        "0:6": {
            "crate_id": 0,
            "path": [
                "lib",
                "B"
            ],
            "kind": "typedef"
        }
    },
    "external_crates": {},
    "format_version": 7
}
