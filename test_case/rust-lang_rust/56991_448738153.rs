
            asm!("
                mov rcx, QWORD PTR [$1]
                \tmov rcx, QWORD PTR [rcx]
                \tmov QWORD PTR [$1], rcx\n\t"
                : "+m"(*x)
                : "r"(x)
                : "rcx"
                : "intel"
