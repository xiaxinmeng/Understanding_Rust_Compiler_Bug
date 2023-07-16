
RFC 2103 - Attributes for Tools Requirements

┌───────────┐
│ Long-term │
└───────────┘
[x] Some unspecified opt-in mechanism
    │
    ├> #[cfg_tool(mytool)] ?
    │ 
    └> #[allow_tool(mytool)] ?

┌──────────────────┐
│ RFC Requirements │
└──────────────────┘
[x] Allow any tool that ships with Rust to be included
    │   by default
    │
    └> { rustc, rustdoc, rls } (should I add clippy and rustfmt? the RFC talks about adding them "soon")

[x] Attributes must be long-lived in compilation

[x] Compiler doesn't warn on unused/unknown attributes,
    │   however tools must
    │
    │ Allowed by compiler
    ├> #[rustfmt::foo] 
    │ Tool produced warning/error
    └> ex. "rustfmt: warn: `foo` is an unknown attribute"