 text
Explicitly specify HashMap's iterators to be non-deterministic between iterations.
This would allow e.g. next_back to be implemented as next, reducing code complexity.
