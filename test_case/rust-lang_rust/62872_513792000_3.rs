rust
        .buffered(16)
// Uncallable, `where Self::Item: Future` doesn't hold because `Self::Item = ()`
