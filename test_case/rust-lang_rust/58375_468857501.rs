
error[E0599]: no method named `start` found for type `proc_macro2::Span` in the current scope
  --> /home/jethro/.cargo/registry/src/github.com-1ecc6299db9ec823/cargo-tarpaulin-0.7.0/src/source_analysis.rs:82:27
   |
82 |             for i in span.start().line..(span.end().line + 1) {
   |                           ^^^^^
   |
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `start`, perhaps you need to implement it:
           candidate #1: `statemachine::StateData`
