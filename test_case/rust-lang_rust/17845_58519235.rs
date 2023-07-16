
asm!("mulq $2; divq $3" : "={rax}"(d) : "{rax}"(a), "{rdi}"(b), "{rsi}"(c) : "%rdx" );
