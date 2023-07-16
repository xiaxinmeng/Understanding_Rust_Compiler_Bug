
TMP0 = &mut foo; // create the receiver in a temporary the user cannot access
... // prepare the other arguments; this may introduce basic blocks etc
Foo::bar(TMP0, ...)
