
[01:03:10] ---- test_snippet::multiple_labels_secondary_without_message_3 stdout ----

[01:03:10] 	span: Span { lo: BytePos(14), hi: BytePos(19), ctxt: #0 } label: "`a` is a good letter"

[01:03:10] text: Ok("a { b")

[01:03:10] span: Span { lo: BytePos(22), hi: BytePos(27), ctxt: #0 } label: ""

[01:03:10] text: Ok("c } d")

[01:03:10] expected output:

[01:03:10] ------

[01:03:10] error: foo

[01:03:10]  --> test.rs:3:3

[01:03:10]   |

[01:03:10] 3 |   a  bc  d

[01:03:10]   |   ^^^^----

[01:03:10]   |   |

[01:03:10]   |   `a` is a good letter

[01:03:10] 

[01:03:10] ------

[01:03:10] actual output:

[01:03:10] ------

[01:03:10] error: foo

[01:03:10]  --> test.rs:3:3

[01:03:10]   |

[01:03:10] 3 |   a { b { c } d }

[01:03:10]   |   ^^^^^   -----

[01:03:10]   |   |

[01:03:10]   |   `a` is a good letter

[01:03:10] 

[01:03:10] ------
