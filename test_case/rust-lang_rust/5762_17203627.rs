
[17:26:42] <nmatsakis> anyway, so yeah, the idea would be to (1) add a new kind of adjustment for objects
[17:26:54] <nmatsakis> (2) modify code-generation (what we call `trans`) to deal with that
[17:27:10] <nmatsakis> (3) modify the method code to borrow in search_for_autosliced_method()
[17:27:24] <nmatsakis> and (4) modify the method code to handle the @self etc coercions
[17:27:34] <nmatsakis> if that all works out, then the end result is that whenever we call a method that expects &self
