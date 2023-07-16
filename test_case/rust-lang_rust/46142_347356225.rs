rust
[00:19:46] error[E0599]: no associated item named `Consume` found for type `rustc::mir::Operand<'_>` in the current scope
[00:19:46]    --> /checkout/src/librustc_mir/transform/add_moves_for_packed_drops.rs:135:34
[00:19:46]     |
[00:19:46] 135 |                      Rvalue::Use(Operand::Consume(location.clone())));
[00:19:46]     |                                  ^^^^^^^^^^^^^^^^
