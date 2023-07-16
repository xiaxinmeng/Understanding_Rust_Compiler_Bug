
// Load address of memory region 1 into R25 and R24
00000092 ce.01                MOVW R24,R28		Copy register pair 
00000093 01.96                ADIW R24,0x01		Add immediate to word

// Load address of memory region 2 into R23 and R22
00000094 be.01                MOVW R22,R28		Copy register pair 
00000095 6a.5f                SUBI R22,0xFA		Subtract immediate 
00000096 7f.4f                SBCI R23,0xFF		Subtract immediate with carry

// Initialize the length of the array 
00000097 45.e0                LDI R20,0x05		Load immediate 
00000098 50.e0                LDI R21,0x00		Load immediate

// Call memcmp -- does not modify the contents of R22 and R23
00000099 0e.94.7b.01          CALL 0x0000017B		Call subroutine
0000009B e0.e0                LDI R30,0x00		Load immediate 
0000009C f0.e0                LDI R31,0x00		Load immediate

// Compare R22 (which still contains the address of memory region 2) with 0x00.  This clears the zero bit.
0000009D 60.30                CPI R22,0x00		Compare with immediate 
0000009E 70.07                CPC R23,R16		Compare with carry 

// Check the return value from memcmp
0000009F 8e.07                CPC R24,R30		Compare with carry 
000000A0 9f.07                CPC R25,R31		Compare with carry

// The zero bit is cleared so we never branch
000000A1 09.f0                BREQ PC+0x02		Branch if equal 
000000A2 10.e0                LDI R17,0x00		Load immediate 
000000A3 11.70                ANDI R17,0x01		Logical AND with immediate
...
0000017B fb.01                MOVW R30,R22		Copy register pair 
0000017C dc.01                MOVW R26,R24		Copy register pair 
0000017D 04.c0                RJMP PC+0x0005		Relative jump 
0000017E 8d.91                LD R24,X+		        Load indirect and postincrement 
0000017F 01.90                LD R0,Z+		        Load indirect and postincrement 
00000180 80.19                SUB R24,R0		Subtract without carry 
00000181 21.f4                BRNE PC+0x05		Branch if not equal 
00000182 41.50                SUBI R20,0x01		Subtract immediate 
00000183 50.40                SBCI R21,0x00		Subtract immediate with carry 
00000184 c8.f7                BRCC PC-0x06		Branch if carry cleared 
00000185 88.1b                SUB R24,R24		Subtract without carry 
00000186 99.0b                SBC R25,R25		Subtract with carry 
00000187 08.95                RET 		        Subroutine return 
