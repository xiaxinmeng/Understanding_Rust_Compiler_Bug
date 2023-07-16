rust
   Compiling script v0.0.1 (file:///home/simon/servo2/components/script)
error[E0599]: no method named `trace` found for type `&fn(&dom::node::Node) -> u16` in the current scope
   --> /home/simon/servo2/components/script/dom/treewalker.rs:464:10
    |
464 | #[derive(JSTraceable)]
    |          ^^^^^^^^^^^
    |
    = note: JSTraceable is a function, perhaps you wish to call it
    = help: items from traits can only be used if the trait is implemented and in scope
    = note: the following trait defines an item `trace`, perhaps you need to implement it:
            candidate #1: `dom::bindings::trace::JSTraceable`

error: aborting due to previous error

error: Could not compile `script`.
