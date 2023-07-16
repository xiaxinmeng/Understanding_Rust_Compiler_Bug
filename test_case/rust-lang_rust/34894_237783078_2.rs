

Here, `y` is bound by-value in one case and by-reference in the other.

To fix this error, just use the same mode in both cases.
Generally using `ref` or `ref mut` where not already used will fix this:

