
rustc 1.16.0 (30cf806ef 2017-03-10)
error: `Self` type is used before it's determined
 --> <anon>:2:11
  |
2 |     where Self::Item: PartialEq {
  |           ^^^^^^^^^^

error: internal compiler error: /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/librustc/ty/mod.rs:1849: No def'n found for DefId { krate: CrateNum(0), node: DefIndex(3) => rust_out/4089d7c8b778d88cec885baf7b69e6df-exe::{{impl}}[0] } in tcx.impl_trait_refs
