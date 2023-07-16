
error[E0308]: method not compatible with trait
   --> src/actions/mod.rs:193:5
    |
193 |     fn from_str(s: &'a str) -> Result<Self, Self::Err> {
    |     ^ lifetime mismatch
    |
    = note: expected type `fn(&str) -> std::result::Result<actions::Action<'a>, actions::ActionError<'_>>`
    = note:    found type `fn(&'a str) -> std::result::Result<actions::Action<'a>, actions::ActionError<'_>>`
    = note: the anonymous lifetime #1 defined on unknown free region bounded by scope CodeExtent(687/CallSiteScope { fn_id: NodeId(436), body_id: NodeId(1197) })...
    = note: ...does not necessarily outlive the lifetime 'a as defined on unknown free region bounded by scope CodeExtent(687/CallSiteScope { fn_id: NodeId(436), body_id: NodeId(1197) })
help: consider using an explicit lifetime parameter as shown: fn from_str(s: &'a str) -> Result<Self, <Self>::Err>
   --> src/actions/mod.rs:193:5
    |
193 |     fn from_str(s: &'a str) -> Result<Self, Self::Err> {
    |     ^

error: aborting due to previous error
