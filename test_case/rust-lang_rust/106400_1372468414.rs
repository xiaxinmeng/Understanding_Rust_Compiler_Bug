
$ git log HEAD^^
commit 14298997005486529e7a97b2ab9e9e376d6fdc72
Merge: 03b9e1d154e e048ee2ac1c
Author: bors <bors@rust-lang.org>
Date:   Thu Jan 5 06:47:02 2023 +0000

    Auto merge of #106482 - compiler-errors:rollup-g7n1p39, r=compiler-errors
    
    Rollup of 6 pull requests
    
    Successful merges:
    
     - #105846 (Account for return-position `impl Trait` in trait in `opt_suggest_box_span`)
     - #106385 (Split `-Zchalk` flag into `-Ztrait-solver=(classic|chalk|next)` flag)
     - #106403 (Rename `hir::Map::{get_,find_}parent_node` to `hir::Map::{,opt_}parent_id`, and add `hir::Map::{get,find}_parent`)
     - #106462 (rustdoc: remove unnecessary wrapper around sidebar and mobile logos)
     - #106464 (Update Fuchsia walkthrough with new configs)
     - #106478 (Tweak wording of fn call with wrong number of args)
    
    Failed merges:
    
    r? `@ghost`
    `@rustbot` modify labels: rollup
