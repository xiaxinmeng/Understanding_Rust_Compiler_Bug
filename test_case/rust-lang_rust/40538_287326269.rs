
failures:

---- range.rs - range::RangeArgument::end (line 52) stdout ----

	error: this feature has been stable since 1.17.0. Attribute no longer needed

 --> <anon>:3:12

  |

3 | #![feature(collections_bound)]

  |            ^^^^^^^^^^^^^^^^^

  |

  = note: #[deny(stable_features)] implied by #[deny(warnings)]

note: lint level defined here

 --> <anon>:6:9

  |

6 | #![deny(warnings)]

  |         ^^^^^^^^

error: aborting due to previous error(s)

thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc/session/mod.rs:203

note: Run with `RUST_BACKTRACE=1` for a backtrace.

---- range.rs - range::RangeArgument::start (line 29) stdout ----

	error: this feature has been stable since 1.17.0. Attribute no longer needed

 --> <anon>:3:12

  |

3 | #![feature(collections_bound)]

  |            ^^^^^^^^^^^^^^^^^

  |

  = note: #[deny(stable_features)] implied by #[deny(warnings)]

note: lint level defined here

 --> <anon>:6:9

  |

6 | #![deny(warnings)]

  |         ^^^^^^^^

