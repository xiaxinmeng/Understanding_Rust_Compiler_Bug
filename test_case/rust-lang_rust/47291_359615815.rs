rust
fn foo(x: !) {
  x; // ICEs
}
fn bar(x: !) {
  {x}; // No problem!
}
