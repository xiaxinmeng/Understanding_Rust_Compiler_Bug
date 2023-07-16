rust
// copyright header

// libsyntax ICE when trimming the blank line in the middle of that comment

// @has issue_47197/fn.x.html
/**
* a

* b
*/
pub fn x() { }
