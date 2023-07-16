
(gdb) ptype nomangle::TEST
No symbol 'nomangle::TEST' in current context
(gdb) set lang c++
(gdb) ptype nomangle::TEST
type = u64
(gdb) info addr nomangle::TEST
Symbol "nomangle::TEST" is static storage at address 0x65ec0.
(gdb) set lang rust
(gdb) info addr nomangle::TEST
No symbol "nomangle::TEST" in current context.
(gdb) ptype TEST
No symbol 'TEST' in current context
(gdb) info addr TEST
Symbol "TEST" is at 0x65ec0 in a file compiled without debugging.
