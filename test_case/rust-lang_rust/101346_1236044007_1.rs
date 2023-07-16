asm
example::foo:
        xor     eax, eax ; set EAX to zero
        xor     ecx, ecx ; set ECX to zero
        mov     eax, ebx ; "cache" calee-saved EBX to EAX
        cpuid ; WHOOPS not only we now use EAX which contains EBX's value,
              ; but also CPUID overwrites EAX, making the caching useless
        xchg    eax, ebx ; move EBX into EAX as return result
        ret
