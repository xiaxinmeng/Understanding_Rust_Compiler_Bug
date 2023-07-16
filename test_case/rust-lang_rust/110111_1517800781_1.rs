]``
* source: ``[  `  struct@Clone  `  ]``, suggestion: ``trait@ct@Clone  `  ]`` (sic!)
* source: ``[  `Clone ()`  ]``, suggestion: ``trait@[  `Clon`` (sic!)

There are several issues at play here: When creating a `Span`

*  we are basing off our index calculations on a HTML-normalized string (i.e. where consecutive spaces have been collapsed into single ones) instead of the source string (and we can't easily get the source string from `pulldown-cmark` unfortunately)
* we assume that the delimiter like `trait@` is right at the start of the destination string but actually spaces, backticks and sometimes even `[` can come first (under certain circumstances).
* sometimes the base `Span` is trimmed by one backtick, further rendering the offset calculations incorrect
