
note.rs:8:32: 8:33 error: obsolete syntax: `/` lifetime notation
note.rs:8             fn next2(self) -> B/&self { let B { a: a } = self; B { a: a } }
                                          ^
note: instead of `&foo/bar`, write `&'foo bar`; instead of `bar/&foo`, write `&bar<'foo>
error: aborting due to previous error
