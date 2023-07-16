
[11:48:39] <nmatsakis> barosl: I have mixed feelings; we used to do this, but it made it hard to know when errors would be reported, in turn making it hard to write reliable test cases
[11:48:57] <nmatsakis> in general I've been modifying instead to have things return Err() in theoretically impossible cases
