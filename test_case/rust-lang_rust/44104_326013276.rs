
[00:40:37]  warning: unknown lint: `FOO_BAR`

[00:40:37] -  --> $DIR/not_found.rs:11:9

[00:40:37] +  --> $DIR/not_found.rs:14:9

[00:40:37]     |

[00:40:37]  14 | #[allow(FOO_BAR)]

[00:40:37]     |         ^^^^^^^

[00:40:37]     |

[00:40:37]     = note: #[warn(unknown_lints)] on by default

[00:40:37]  

[00:40:37]  warning: unknown lint: `DEAD_CODE`

[00:40:37]    --> $DIR/not_found.rs:16:8

[00:40:37]     |

[00:40:37]  16 | #[warn(DEAD_CODE)]

[00:40:37]     |        ^^^^^^^^^ help: lowercase the lint name: `dead_code`

[00:40:37]  

[00:40:37]  warning: unknown lint: `Warnings`

[00:40:37]    --> $DIR/not_found.rs:18:8

[00:40:37]     |

[00:40:37]  18 | #[deny(Warnings)]

[00:40:37]     |        ^^^^^^^^ help: lowercase the lint name: `warnings`
