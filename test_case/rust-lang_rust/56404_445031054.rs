
.text:0000000180001235                 mov     rsi, rcx
.text:0000000180001238                 mov     rax, [rcx]
.text:000000018000123B                 mov     rcx, [rcx+8]
.text:000000018000123F                 add     rax, 7
.text:0000000180001243                 and     rax, 0FFFFFFFFFFFFFFF8h
.text:0000000180001247                 mov     [rsi], rax
.text:000000018000124A                 cmp     rcx, rax
.text:000000018000124D                 jb      short loc_180001281
.text:000000018000124F                 mov     rdx, rax
.text:0000000180001252                 add     rdx, 8
.text:0000000180001256                 cmp     rdx, rcx
.text:0000000180001259                 jnb     short loc_18000126B
.text:000000018000125B
.text:000000018000125B loc_18000125B:                          ; CODE XREF: alloctest+4F↓j
.text:000000018000125B                 mov     [rsi], rdx
...
.text:000000018000126B ; ---------------------------------------------------------------------------
.text:000000018000126B
.text:000000018000126B loc_18000126B:                          ; CODE XREF: alloctest+29↑j
.text:000000018000126B                 mov     edx, 8
.text:0000000180001270                 mov     rcx, rsi
.text:0000000180001273                 call    _ZN5arena13DroplessArena4grow17h2cc6dfe0f3889a5eE ; arena::DroplessArena::grow::h2cc6dfe0f3889a5e
.text:0000000180001278                 mov     rax, [rsi]
.text:000000018000127B                 lea     rdx, [rax+8]
.text:000000018000127F                 jmp     short loc_18000125B
.text:0000000180001281 ; ---------------------------------------------------------------------------
.text:0000000180001281
.text:0000000180001281 loc_180001281:                          ; CODE XREF: alloctest+1D↑j
.text:0000000180001281                 lea     rcx, aAssertionFaile ; "assertion failed: self.ptr <= self.end"
.text:0000000180001288                 lea     r8, off_180003230
.text:000000018000128F                 mov     edx, 26h
.text:0000000180001294                 call    _ZN3std9panicking11begin_panic17hca0ac606c2739ec6E ; std::panicking::begin_panic::hca0ac606c2739ec6
.text:0000000180001299 ; ---------------------------------------------------------------------------
.text:0000000180001299                 ud2
.text:0000000180001299 alloctest       endp
