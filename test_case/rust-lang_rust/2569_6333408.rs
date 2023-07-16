

// shebang means 'run through this interpreter', like always
#!/bin/env rustc --run 

// leading sharp-comments in the root source file are scanned for pre-parsing control tags.
# rust: <compiler-uuid> <low-lang-version> <high-lang-version>

// bang-attrs apply to their containing item.
// in practice so far these seem to be module-level
// most of the time, so are "evocative" of shebang and
// pre-parse control tags, without being ambiguous with them
#[!warn(no_implicit_copies)]

// normal-attrs apply to the following item.
#[test]
fn test_patience() { ... }

// bang-comments are docstrings that apply to their enclosing item.

//! this is a one-liner.
/*! This is a
 *  multi-line one.
 */

// Common triple-slash and double-star doc-comments
// apply to the next item.

/// Magic number
const x = 10;

/** Very enjoyable
 *  function doc
 */
fn joy() {
}
