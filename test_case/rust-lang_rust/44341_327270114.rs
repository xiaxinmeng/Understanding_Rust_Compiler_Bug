
[01:05:41]    Compiling rustc v0.0.0 (file:///checkout/src/librustc)

[01:05:47] error: unused import: `DummyCrateStore`

[01:05:47]     --> /checkout/src/librustc/session/config.rs:1954:32

[01:05:47]      |

[01:05:47] 1954 |     use middle::cstore::{self, DummyCrateStore};

[01:05:47]      |                                ^^^^^^^^^^^^^^^

[01:05:47]      |

[01:05:47] note: lint level defined here

[01:05:47]     --> /checkout/src/librustc/lib.rs:20:9

[01:05:47]      |

[01:05:47] 20   | #![deny(warnings)]

[01:05:47]      |         ^^^^^^^^

[01:05:47]      = note: #[deny(unused_imports)] implied by #[deny(warnings)]

[01:05:47] 

[01:05:47] error: unused import: `std::rc::Rc`

[01:05:47]     --> /checkout/src/librustc/session/config.rs:1960:9

[01:05:47]      |

[01:05:47] 1960 |     use std::rc::Rc;

[01:05:47]      |         ^^^^^^^^^^^

[01:05:47] 
