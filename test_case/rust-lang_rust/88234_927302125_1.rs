json
{
  "id": "0:0",
  "crate_id": 0,
  "name": "foo",
  "span": {
    "filename": "src/lib.rs",
    "begin": [
      1,
      0
    ],
    "end": [
      3,
      11
    ]
  },
  "visibility": "public",
  "docs": null,
  "links": {},
  "attrs": [
    "#![feature(doc_primitive)]"
  ],
  "deprecation": null,
  "kind": "module",
  "inner": {
    "is_crate": true,
    "items": [
      "0:3" // I assume this refers to `char` module but it's private
    ]
  }
}
