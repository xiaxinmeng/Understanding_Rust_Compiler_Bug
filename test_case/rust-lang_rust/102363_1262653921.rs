
ne321@vertex39:rust$ rm -rf build/x86_64-unknown-linux-gnu/doc/
ne321@vertex39:rust$ RUSTDOCFLAGS="--output-format=json --document-private-items" ./x.py doc library/std --stage 2
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.54s
Documenting stage2 std (x86_64-unknown-linux-gnu) in HTML format
Building rustdoc for stage2 (x86_64-unknown-linux-gnu)
   Compiling rustdoc v0.0.0 (/data/ne321/rust/src/librustdoc)
   Compiling rustdoc-tool v0.0.0 (/data/ne321/rust/src/tools/rustdoc)
    Finished release [optimized] target(s) in 9.61s
 Documenting core v0.0.0 (/data/ne321/rust/library/core)
    Finished release [optimized] target(s) in 7.53s
 Documenting alloc v0.0.0 (/data/ne321/rust/library/alloc)
    Finished release [optimized] target(s) in 1.71s
 Documenting std v0.0.0 (/data/ne321/rust/library/std)
    Finished release [optimized] target(s) in 2.78s
Build completed successfully in 0:00:22
ne321@vertex39:rust$ oj -x "$.index[?(@.name=='usize' && @.kind=='primitive')]" -d "$..impls" -p 200.9 build/x86_64-unknown-linux-gnu/doc/core.json 
{
  "attrs": ["#[doc(primitive = \"usize\")]", "#[stable(feature = \"rust1\", since = \"1.0.0\")]"],
  "crate_id": 0,
  "deprecation": null,
  "docs": "The pointer-sized unsigned integer type.\n\nThe size of this primitive is how many bytes it takes to reference any\nlocation in memory. For example, on a 32 bit target, this is 4 bytes\nand on a 64 bit target, this is 8 bytes.",
  "id": "0:75795:1516",
  "inner": {"name": "usize"},
  "kind": "primitive",
  "links": {},
  "name": "usize",
  "span": {"begin": [1241, 0], "end": [1241, 17], "filename": "library/core/src/primitive_docs.rs"},
  "visibility": "public"
}
ne321@vertex39:rust$ oj -x "$.index[?(@.name=='prim_usize')]" -p 200.9 build/x86_64-unknown-linux-gnu/doc/core.json
{
  "attrs": ["#[doc(primitive = \"usize\")]", "#[stable(feature = \"rust1\", since = \"1.0.0\")]"],
  "crate_id": 0,
  "deprecation": null,
  "docs": "The pointer-sized unsigned integer type.\n\nThe size of this primitive is how many bytes it takes to reference any\nlocation in memory. For example, on a 32 bit target, this is 4 bytes\nand on a 64 bit target, this is 8 bytes.",
  "id": "0:75795:77753",
  "inner": {"is_crate": false, "is_stripped": false, "items": []},
  "kind": "module",
  "links": {},
  "name": "prim_usize",
  "span": {"begin": [1241, 0], "end": [1241, 14], "filename": "library/core/src/primitive_docs.rs"},
  "visibility": "crate"
}
