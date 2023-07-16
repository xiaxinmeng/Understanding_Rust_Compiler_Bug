
./x.py test src/test/run-pass --stage 1 --test-args reachable-unnameable-items
error: internal compiler error: src/librustc_trans/collector.rs:653: Cannot create local trans-item for DefId { krate: CrateNum(11), node: DefIndex(29) => reachable_unnameable_items/8cd878b::inner_private_module[0]::{{impl}}[7]::method_of_unnameable_type8[0] }
