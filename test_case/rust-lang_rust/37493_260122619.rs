
error[E0204]: the trait `Copy` may not be implemented for type `Bar`
  --> $DIR/issue-19950.rs:23:1
   |
23 | impl Copy for Bar {}
   | ^^^^^^^^^^^^^^^^^^^^ field `item` doesn't implement `Copy`
