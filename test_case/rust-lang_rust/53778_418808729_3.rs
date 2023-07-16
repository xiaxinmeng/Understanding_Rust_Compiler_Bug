rust
macro m { } // candidate A
{
  macro gen_gen_inner_invoc() { /* definition not relevant */ }
  gen_inner_invoc!() as {
    macro m { .. } // candidate B

    m!() // ERROR
  }
}
