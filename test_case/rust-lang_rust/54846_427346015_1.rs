rust
error[E0597]: borrowed value does not live long enough
    --> /home/simon/servo2/target/debug/build/script-1105d98fbcf10485/out/Bindings/AnalyserNodeBinding.rs:1522:10
     |
1522 |         &InterfaceConstructorBehavior::call(_constructor),
     |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ temporary value does not live long enough
...
1525 |         2);
     |          - temporary value only lives until here
     |
     = note: borrowed value must be valid for the static lifetime...
