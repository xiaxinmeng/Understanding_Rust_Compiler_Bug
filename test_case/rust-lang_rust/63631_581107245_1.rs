asm
$ rustc -C opt-level=3 -Z mir-opt-level=3 --crate-type=dylib poc.rs
$ r2 -qc 's sym.got;af;afv-*;pdf;s sym.expect;af;afv-*;pdf' libpoc.so
┌ 69: sym.got ();
│ 0x000476e0  4156                  push r14
│ 0x000476e2  53                    push rbx
│ 0x000476e3  4883ec18              sub rsp, 0x18
│ 0x000476e7  49beefcdab8967452301  movabs r14, 0x123456789abcdef
│ 0x000476f1  4c89742408            mov qword [rsp + 8], r14
│ 0x000476f6  488b1d5b410b00        mov rbx, qword [reloc.show]
│ 0x000476fd  488d7c2408            lea rdi, [rsp + 8]
│ 0x00047702  ffd3                  call rbx
│ 0x00047704  4c893424              mov qword [rsp], r14
│ 0x00047708  4889e7                mov rdi, rsp
│ 0x0004770b  ffd3                  call rbx
│ 0x0004770d  488b0424              mov rax, qword [rsp]
│ 0x00047711  4889442410            mov qword [rsp + 0x10], rax
│ 0x00047716  488d7c2410            lea rdi, [rsp + 0x10]
│ 0x0004771b  ffd3                  call rbx
│ 0x0004771d  4883c418              add rsp, 0x18
│ 0x00047721  5b                    pop rbx
│ 0x00047722  415e                  pop r14
└ 0x00047724  c3                    ret
┌ 54: sym.expect ();
│ 0x00047730  4156                  push r14
│ 0x00047732  53                    push rbx
│ 0x00047733  50                    push rax
│ 0x00047734  48b8efcdab8967452301  movabs rax, 0x123456789abcdef
│ 0x0004773e  48890424              mov qword [rsp], rax
│ 0x00047742  4c8b350f410b00        mov r14, qword [reloc.show]
│ 0x00047749  4889e3                mov rbx, rsp
│ 0x0004774c  4889df                mov rdi, rbx
│ 0x0004774f  41ffd6                call r14
│ 0x00047752  4889df                mov rdi, rbx
│ 0x00047755  41ffd6                call r14
│ 0x00047758  4889df                mov rdi, rbx
│ 0x0004775b  41ffd6                call r14
│ 0x0004775e  4883c408              add rsp, 8
│ 0x00047762  5b                    pop rbx
│ 0x00047763  415e                  pop r14
└ 0x00047765  c3                    ret
