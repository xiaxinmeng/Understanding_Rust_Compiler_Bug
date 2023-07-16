
Reimplemented lifetime elision. This change was almost entirely compatible with existing code, but it did close a number of small bugs and loopholes, as well as being more accepting in some other cases (e.g. https://github.com/rust-lang/rust/issues/41105).
