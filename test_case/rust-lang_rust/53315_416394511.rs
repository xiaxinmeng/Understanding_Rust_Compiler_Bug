
Author: Niko Matsakis <niko@alum.mit.edu>
Date:   Wed Jul 25 19:38:53 2018 +0300

    change from tuple struct to field struct

    Avoid using the private field directly (and call it `private` to
    emphasize that) except when needed. Add `as_u32`, `as_usize`,
    `from_u32`, and friends instead.

    Since we can't `assert!` in a `const fn` from what I can tell, add a
    `from_u32_unchecked` accessor that is unsafe for use in constants. This
    may be overkill here.
